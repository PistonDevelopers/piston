// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// #![crate_id = "png#0.1"]
// #![crate_type = "lib"]
// #![feature(macro_rules, phase)]

// #![deny(warnings)]
#![allow(missing_doc)]

#[phase(syntax, link)]
extern crate log;

#[cfg(test)]
extern crate extra;

use std::cast;
use std::cmp::min;
use std::io;
use std::io::File;
use std::iter::range_step_inclusive;
use std::mem::size_of;
use std::num::abs;
use std::str::from_utf8;

use inflate::InflateStream;

mod inflate;

#[deriving(Eq)]
pub enum ColorType {
    K1, K2, K4, K8, K16,
    KA8, KA16,
    Pal1, Pal2, Pal4, Pal8,
    RGB8, RGB16,
    RGBA8, RGBA16,
}

impl ColorType {
    fn is_palette(self) -> bool {
        match self {
            Pal1 | Pal2 | Pal4 | Pal8 => true,
            _ => false
        }
    }

    fn pixel_bits(self) -> uint {
        match self {
            K1 | Pal1 => 1,
            K2 | Pal2 => 2,
            K4 | Pal4 => 4,
            K8 | Pal8 => 8,
            K16 | KA8 => 16,
            RGB8 => 24,
            KA16 | RGBA8 => 32,
            RGB16 => 48,
            RGBA16 => 64
        }
    }
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub pixels: Vec<u8>
}

pub enum ImageState<'a> {
    Partial(Option<&'a Image>),
    Complete(Image),
    Error(~str)
}

static MAGIC: [u8, ..8] = [
    0x89,
    'P' as u8,
    'N' as u8,
    'G' as u8,
    '\r' as u8, // DOS line ending (CR
    '\n' as u8, //                  LF)
    0x1A,       // DOS EOF
    '\n' as u8  // Unix line ending (LF)
];

#[packed]
struct Ihdr {
    width: u32,
    height: u32,
    bits: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8
}

impl Ihdr {
    fn get_color_type(&self) -> Result<ColorType, ~str> {
        let bits = self.bits;
        let invalid = |name| Err(format!("invalid bit depth {} for color type {} ({:s})",
                                         bits, self.color_type, name));
        Ok(match self.color_type {
            0 => match bits {
                1 => K1,
                2 => K2,
                4 => K4,
                8 => K8,
                16 => K16,
                _ => return invalid("grayscale")
            },
            2 => match bits {
                8 => RGB8,
                16 => RGB16,
                _ => return invalid("truecolor")
            },
            3 => match bits {
                1 => Pal1,
                2 => Pal2,
                4 => Pal4,
                8 => Pal8,
                _ => return invalid("palette")
            },
            4 => match bits {
                8 => KA8,
                16 => KA16,
                _ => return invalid("grayscale with alpha")
            },
            6 => match bits {
                8 => RGBA8,
                16 => RGBA16,
                _ => return invalid("truecolor with alpha")
            },
            _ => return Err(format!("invalid color type {}", self.color_type))
        })
    }

    fn to_image(&self) -> Result<PartialImage, ~str> {
        let color_type = match self.get_color_type() {
            Ok(c) => c,
            Err(m) => return Err(m)
        };

        let color_decoded = match color_type {
            K1 | K2 | K4 | K8 | KA8 => KA8,
            K16 | KA16 | RGB16 | RGBA16 => return Err("unsupported bit depth of 16".to_owned()),
            _ => RGBA8
        };

        let pixel_bytes = color_decoded.pixel_bits() / 8;

        let pixel_bits_raw = color_type.pixel_bits();

        if self.compression_method != 0 {
            return Err(format!("unknown compression method {}", self.compression_method));
        }

        if self.filter_method != 0 {
            return Err(format!("unknown filter method {}", self.filter_method));
        }

        if self.interlace_method > 1 {
            return Err(format!("unknown interlace method {}", self.interlace_method));
        }

        let w = self.width as uint;
        let h = self.height as uint;

        let initial_scanline_width = if self.interlace_method == 1 {
            (w + 7) / 8
        } else {
            w
        };

        Ok(PartialImage {
            image: Image {
                width: self.width,
                height: self.height,
                color_type: color_decoded,
                pixels: Vec::from_elem(w * h * pixel_bytes, 0u8)
            },
            color_type: color_type,
            filter: 0,
            interlace: self.interlace_method,
            palette: None,
            transparent_color: None,
            idat_inflate_stream: None,
            x_byte_pos: 0,
            y_byte_pos: 0,
            scanline_bytes: w * pixel_bytes,
            scanline_pos: None,
            pixel_prev: [0, ..4],
            pixel_bytes_raw: (pixel_bits_raw + 7) / 8,
            scanline_bytes_raw: (initial_scanline_width * pixel_bits_raw + 7) / 8
        })
    }
}

trait Filter {
    fn apply(&mut self, value: u8, a: &u8, b: &u8, c: &u8) -> u8 {
        value + self.addend(a, b, c)
    }

    fn addend(&mut self, a: &u8, b: &u8, c: &u8) -> u8;
}

struct NoFilter;
impl Filter for NoFilter {
    fn addend(&mut self, _: &u8, _: &u8, _: &u8) -> u8 { 0 }
}

struct Sub;
impl Filter for Sub {
    fn addend(&mut self, a: &u8, _: &u8, _: &u8) -> u8 { *a }
}

struct Up;
impl Filter for Up {
    fn addend(&mut self, _: &u8, b: &u8, _: &u8) -> u8 { *b }
}

struct Average;
impl Filter for Average {
    fn addend(&mut self, a: &u8, b: &u8, _: &u8) -> u8 {
        ((*a as u16 + *b as u16) / 2) as u8
    }
}

struct Half<T>(T);
impl<T: Filter> Filter for Half<T> {
    fn addend(&mut self, a: &u8, b: &u8, c: &u8) -> u8 {
        let &Half(ref mut this) = self;
        this.addend(a, b, c) / 2
    }
}

struct Paeth;
impl Filter for Paeth {
    fn addend(&mut self, a: &u8, b: &u8, c: &u8) -> u8 {
        let (a, b, c) = (*a as i16, *b as i16, *c as i16);

        let p = a + b - c;
        let pa = abs(p - a);
        let pb = abs(p - b);
        let pc = abs(p - c);

        if pa <= pb && pa <= pc { a as u8 }
        else if pb <= pc { b as u8 }
        else { c as u8 }
    }
}

struct PartialImage {
    image: Image,
    color_type: ColorType,
    filter: u8,
    interlace: u8,
    palette: Option<Vec<u8>>,
    transparent_color: Option<[u16, ..3]>,
    idat_inflate_stream: Option<Box<InflateStream>>,
    x_byte_pos: uint,
    y_byte_pos: uint,
    scanline_bytes: uint,
    scanline_pos: Option<uint>,
    pixel_prev: [u8, ..4],
    pixel_bytes_raw: uint, // FIXME(eddyb) don't waste space.
    scanline_bytes_raw: uint
}

impl PartialImage {
    fn update_idat(&mut self, mut data: &[u8]) -> Result<(), ~str> {
        let mut scanline_pos = self.scanline_pos;
        let mut filter = self.filter;

        while data.len() > 0 {
            let mut i = match scanline_pos {
                Some(pos) => pos,
                None => {
                    filter = data[0];
                    data = data.slice_from(1);
                    0
                }
            };

            let line = data.slice_to(min(self.scanline_bytes_raw - i, data.len()));

            match filter {
                0 => self.update_scanline(line, NoFilter),
                1 => {
                    if i < self.pixel_bytes_raw {
                        let noop = min(self.pixel_bytes_raw - i, line.len());
                        self.update_scanline(line.slice_to(noop), NoFilter);
                        self.update_scanline(line.slice_from(noop), Sub); // FIXME(eddyb) DRY Sub vvv
                    } else {
                        self.update_scanline(line, Sub);
                    }
                }
                2 => {
                    let (_, _, _, dy) = self.interlace_params();
                    if self.y_byte_pos < dy * self.scanline_bytes {
                        self.update_scanline(line, NoFilter);
                    } else {
                        self.update_scanline(line, Up);
                    }
                }
                3 => {
                    let (_, _, _, dy) = self.interlace_params();
                    if self.y_byte_pos < dy * self.scanline_bytes {
                        // FIXME(eddyb) maybe it's forbidden to have Average for the first scanline?
                        if i < self.pixel_bytes_raw {
                            let noop = min(self.pixel_bytes_raw - i, line.len());
                            self.update_scanline(line.slice_to(noop), NoFilter);
                            //self.update_scanline(line.slice_from(noop), Half(Sub)); // FIXME(eddyb) DRY Half(Sub) vvv
                        } else {
                            //self.update_scanline(line, Half(Sub));
                        }
                    } else {
                        if i < self.pixel_bytes_raw {
                            let noop = min(self.pixel_bytes_raw - i, line.len());
                            //self.update_scanline(line.slice_to(noop), Half(Up));
                            self.update_scanline(line.slice_from(noop), Average); // FIXME(eddyb) DRY Average vvv
                        } else {
                            self.update_scanline(line, Average);
                        }
                    }
                }
                4 => {
                    let (_, _, _, dy) = self.interlace_params();
                    if self.y_byte_pos < dy * self.scanline_bytes {
                        // FIXME(eddyb) maybe it's forbidden to have Paeth for the first scanline?
                        if i < self.pixel_bytes_raw {
                            let noop = min(self.pixel_bytes_raw - i, line.len());
                            self.update_scanline(line.slice_to(noop), NoFilter);
                            self.update_scanline(line.slice_from(noop), Sub); // FIXME(eddyb) DRY Sub vvv
                        } else {
                            self.update_scanline(line, Sub);
                        }
                    } else {
                        if i < self.pixel_bytes_raw {
                            let noop = min(self.pixel_bytes_raw - i, line.len());
                            self.update_scanline(line.slice_to(noop), Up);
                            self.update_scanline(line.slice_from(noop), Paeth); // FIXME(eddyb) DRY Paeth vvv
                        } else {
                            self.update_scanline(line, Paeth);
                        }
                    }
                }
                _ => return Err(format!("unknown filter `{}`", filter))
            }

            /*match filter {
                NoFilter => {
                    self.scanline.mut_slice(i, i + line.len()).copy_from(line);
                    i += line.len();
                }
                Sub => {
                    let mut line = line;
                    if i < self.pixel_bytes_raw {
                        let initial = self.pixel_bytes_raw - i;
                        self.scanline.mut_slice(i, i + initial).copy_from(line.slice_to(initial));
                        line = line.slice_from(initial);
                        i += initial;
                    }
                    for &x in line.iter() {
                        self.scanline[i] = self.scanline[i - self.pixel_bytes_raw] + x;
                        i += 1;
                    }
                }
                Up => {
                    for (a, &b) in self.scanline.mut_slice(i, i + line.len()).mut_iter().zip(line.iter()) {
                        *a += b;
                    }
                    i += line.len();
                }
                Average => for &x in line.iter() {
                    let a = if i < self.pixel_bytes_raw {
                        0
                    } else {
                        self.scanline[i - self.pixel_bytes_raw] as u16
                    };
                    let b = self.scanline[i] as u16;
                    self.scanline[i] = ((a + b) / 2) as u8 + x;
                    i += 1;
                },
                Paeth => {
                    let mut px_pos = i % self.pixel_bytes_raw;
                    for &x in line.iter() {
                        let a = if i < self.pixel_bytes_raw {
                            0
                        } else {
                            self.scanline[i - self.pixel_bytes_raw] as i16
                        };
                        let b = self.scanline[i] as i16;
                        let c = self.pixel_prev[px_pos] as i16;
                        self.pixel_prev[px_pos] = self.scanline[i];
                        let p = a + b - c;
                        let pa = abs(p - a);
                        let pb = abs(p - b);
                        let pc = abs(p - c);
                        let pr = if pa <= pb && pa <= pc { a }
                            else if pb <= pc { b }
                            else { c };
                        self.scanline[i] = pr as u8 + x;
                        i += 1;
                        px_pos += 1;
                        if px_pos >= self.pixel_bytes_raw {
                            px_pos -= self.pixel_bytes_raw;
                        }
                    }
                }
            }*/

            data = data.slice_from(line.len());

            //self.update_scanline(i, i + line.len());
            i += line.len();

            scanline_pos = if i < self.scanline_bytes_raw {
                Some(i)
            } else {
                None
            };
        }

        self.scanline_pos = scanline_pos;
        self.filter = filter;

        Ok(())
    }

    fn interlace_params(&self) -> (/*x0*/ uint, /*y0*/ uint, /*dx*/ uint, /*dy*/ uint) {
        match self.interlace {
            // interlace_method = 0:
            0 => (0, 0, 1, 1),

            // interlace_method = 1 (7 steps):
            1 => (0, 0, 8, 8),

            /* NOTE these seem to follow the pattern:
             * (i, 0, 2*i, 2*i);
             * (0, i,   i, 2*i);
             * with i in [4, 2, 1].
             */
            2 => (4, 0, 8, 8),
            3 => (0, 4, 4, 8),

            4 => (2, 0, 4, 4),
            5 => (0, 2, 2, 4),

            6 => (1, 0, 2, 2),
            7 => (0, 1, 1, 2),
            _ => fail!("unreacheable (interlace step)")
        }
    }

    fn update_scanline<F: Filter>(&mut self, data: &[u8], f: F) {
        // HACK(eddyb) specialize update_scanline_with_dx for the best cases.
        // See interlace_params for more information.
        match self.interlace {
            0 | 7 => self.update_scanline_with_dx::<[u8, ..1], F>(data, f),
            1 | 2 => self.update_scanline_with_dx::<[u8, ..8], F>(data, f),
            3 | 4 => self.update_scanline_with_dx::<[u8, ..4], F>(data, f),
            5 | 6 => self.update_scanline_with_dx::<[u8, ..2], F>(data, f),
            _ => fail!("unreacheable (interlace step)")
        }
    }

    fn update_scanline_with_dx<DX, F: Filter>(&mut self, mut data: &[u8], mut f: F) {
        // HACK extract dx from the size of DX = [u8, ..dx].
        let dx = ::std::mem::size_of::<DX>();
        let (x0, _, _, dy) = self.interlace_params();

        let mut i = self.y_byte_pos + self.x_byte_pos;
        let next_line = self.y_byte_pos + self.scanline_bytes;

        {
            let pixels = self.image.pixels.as_mut_slice();

            macro_rules! filter (($x:expr, $pixel_bytes:expr) => ({
                // HACK(eddyb) this requires the filter to not deref invalid references.
                let (a, b, c): (&u8, &u8, &u8) = unsafe {(
                    cast::transmute(pixels.unsafe_ref(i - dx * $pixel_bytes)),
                    cast::transmute(pixels.unsafe_ref(i - dy * self.scanline_bytes)),
                    cast::transmute(pixels.unsafe_ref(i - dx * $pixel_bytes - dy * self.scanline_bytes))
                )};
                f.apply($x, a, b, c)
            }))

            // Alpha for a RGB8 color (transparent if equal to tRNS, opaque otherwise).
            macro_rules! trns_rgb8 (($r:expr, $g:expr, $b:expr) => ({
                match self.transparent_color {
                    Some([tr, tg, tb]) if tr == $r as u16
                                       && tg == $g as u16
                                       && tb == $b as u16 => 0x00,
                    _ => 0xff
                }
            }))

            // One byte of RGBA or KA (just one store, not updating i).
            macro_rules! pixel_byte_store (($byte:expr, $offset:expr) => ({
                // HACK(eddyb) avoid bound checks, LLVM can't optimize this.
                let byte = $byte;
                // pixels[$offset] = byte;
                unsafe {
                    pixels.unsafe_set($offset, byte);
                }
            }))

            // Grayscale + Alpha.
            macro_rules! pixel_ka (($k:expr, $alpha:expr) => ({
                // FIXME(eddyb) Why is this slower?
                /*pixel_byte_store!($k, i);
                pixel_byte_store!($alpha, i + 1);
                i += dx * 2;*/
                pixel_byte_store!($k, i); i += 1;
                pixel_byte_store!($alpha, i); i += 1;
                i += (dx - 1) * 2;
            }))

            // Grayscale.
            macro_rules! pixel_k (($k:expr, $multiplier:expr) => ({
                let k = $k;
                let alpha = match self.transparent_color {
                    Some([tk, ..]) if tk == k as u16 => 0x00,
                    _ => 0xff
                };
                pixel_ka!(k * $multiplier, alpha);
            }))
            // Palette.
            /*let pixel_pal = |j: u8| {
                let palette = self.palette.as_ref().unwrap();
                let j = j as uint * 4;
                pixel_byte_store!(palette[j], i);
                pixel_byte_store!(palette[j + 1], i + 1);
                pixel_byte_store!(palette[j + 2], i + 2);
                pixel_byte_store!(palette[j + 3], i + 3);
                i += dx * 4;
            };*/
            // One byte of RGBA or KA.
            macro_rules! pixel_byte (($byte:expr, $pixel_bytes:expr) => ({
                pixel_byte_store!($byte, i);

                i += 1;

                if i % $pixel_bytes == 0 {
                    i += (dx - 1) * $pixel_bytes;
                }
            }))

            match self.color_type {
                // FIXME(eddyb) enable this whenever it doesn't slow everything down anymore.
                /*K1 | K2 | K4 => for &x in data.map(|&x| filter!(x, 2)).iter() {
                    let bits = self.color_type.pixel_bits();
                    let multiplier = match bits {
                        2 => 0x55,
                        4 => 0x11,
                        _ => 0xff
                    };
                    for bit in range_step_inclusive(8 - bits, 0, bits) {
                        pixel_k((x >> bit) & ((1 << bits) - 1), multiplier);
                        if i > next_line { // TODO(eddyb) optimize (check only for last byte).
                            break;
                        }
                    }
                },*/
                K8 => for &x in data.iter() {
                    pixel_k!(filter!(x, 2), 0x01);
                },
                KA8 => {
                    // Process the first few bytes until i is 2-aligned.
                    if i % 2 == 1 {
                        pixel_byte!(filter!(data[0], 2), 2);
                        data = data.slice_from(1);
                    }
                    if data.len() >= 2 {
                        assert!(i % 2 == 0);
                        // HACK(eddyb) Works better than a chunks iterator.
                        for j in range_step_inclusive(0, data.len() - 2, 2) {
                            let (k, a) = unsafe {(
                                *data.unsafe_ref(j),
                                *data.unsafe_ref(j + 1)
                            )};
                            // FIXME(eddyb) Why is this slower?
                            /*pixel_byte_store!(k, i);
                             pixel_byte_store(alpha, i + 1);
                             i += dx * 2;*/
                            pixel_byte_store!(filter!(k, 2), i); i += 1;
                            pixel_byte_store!(filter!(a, 2), i); i += 1;
                            i += (dx - 1) * 2;
                        }
                    }
                    if data.len() % 2 == 1 {
                        // Process the last byte (if any).
                        pixel_byte!(filter!(data[data.len() - 1], 2), 2);
                    }
                }
                // FIXME(eddyb) enable this whenever it doesn't slow everything down anymore.
                /*Pal1 | Pal2 | Pal4 => for &x in data.iter() {
                    let bits = self.color_type.pixel_bits();
                    for bit in range_step_inclusive(8 - bits, 0, bits) {
                        pixel_pal!((x >> bit) & ((1 << bits) - 1));
                        if i > next_line { // TODO(eddyb) optimize (check only for last byte).
                            break;
                        }
                    }
                },*/
                // FIXME(eddyb) enable this when scanline-preserving filters work.
                /*Pal8 => for &x in data.iter() {
                    pixel_pal!(x);
                },*/
                RGB8 => {
                    // Process the first few bytes until i is 4-aligned.
                    while data.len() > 0 {
                        pixel_byte!(filter!(data[0], 4), 4);
                        data = data.slice_from(1);

                        if i % 4 == 3 {
                            pixel_byte!(trns_rgb8!(pixels[i - 3], pixels[i - 2], pixels[i - 1]), 4);
                            break;
                        }
                    }
                    let mut last = 0u;
                    if data.len() >= 3 {
                        assert!(i % 4 == 0);
                        macro_rules! with_trns (($trns:expr) => ({
                            // HACK(eddyb) Works better than a chunks iterator.
                            for j in range_step_inclusive(0, data.len() - 3, 3) {
                                let (r, g, b) = unsafe {(
                                    *data.unsafe_ref(j),
                                    *data.unsafe_ref(j + 1),
                                    *data.unsafe_ref(j + 2)
                                )};
                                // FIXME(eddyb) Why is this slower?
                                /*pixel_byte_store!(r, i);
                                pixel_byte_store!(g, i + 1);
                                pixel_byte_store!(b, i + 2);
                                pixel_byte_store!(if has_trns {trns_rgb8!(r, g, b)} else {0xff}, i + 3);
                                i += dx * 4;*/
                                pixel_byte_store!(filter!(r, 4), i); i += 1;
                                pixel_byte_store!(filter!(g, 4), i); i += 1;
                                pixel_byte_store!(filter!(b, 4), i); i += 1;
                                pixel_byte_store!($trns(r, g, b), i); i += 1;
                                i += (dx - 1) * 4;
                                last = j + 3;
                            }
                        }))
                        match self.transparent_color {
                            Some([tr, tg, tb]) => with_trns!(|r: u8, g: u8, b: u8| {
                                if tr == r as u16
                                && tg == g as u16
                                && tb == b as u16 {
                                    0x00
                                } else {
                                    0xff
                                }
                            }),
                            None => with_trns!(|_, _, _| 0xff)
                        }
                        if last < data.len() {
                            // Process the last few (one or two) bytes.
                            for &x in data.slice_from(last).iter() {
                                pixel_byte!(filter!(x, 4), 4);
                            }
                        }
                    }
                }
                RGBA8 => {
                    // Process the first few bytes until i is 4-aligned.
                    while data.len() > 0 {
                        pixel_byte!(filter!(data[0], 4), 4);
                        data = data.slice_from(1);

                        if i % 4 == 0 {
                            break;
                        }
                    }
                    if data.len() >= 4 {
                        assert!(i % 4 == 0);
                        // HACK(eddyb) Works better than a chunks iterator.
                        for j in range_step_inclusive(0, data.len() - 4, 4) {
                            let (r, g, b, a) = unsafe {(
                                *data.unsafe_ref(j),
                                *data.unsafe_ref(j + 1),
                                *data.unsafe_ref(j + 2),
                                *data.unsafe_ref(j + 3)
                            )};
                            // FIXME(eddyb) Why is this slower?
                            /*pixel_byte_store!(r, i);
                            pixel_byte_store!(g, i + 1);
                            pixel_byte_store!(b, i + 2);
                            pixel_byte_store!(a, i + 3);
                            i += dx * 4;*/
                            pixel_byte_store!(filter!(r, 4), i); i += 1;
                            pixel_byte_store!(filter!(g, 4), i); i += 1;
                            pixel_byte_store!(filter!(b, 4), i); i += 1;
                            pixel_byte_store!(filter!(a, 4), i); i += 1;
                            i += (dx - 1) * 4;
                        }
                    }
                    let rest = data.len() % 4;
                    if rest > 0 {
                        // Process the last few (one to three) bytes.
                        for &x in data.slice_from(data.len() - rest).iter() {
                            pixel_byte!(filter!(x, 4), 4);
                        }
                    }
                }
                _ => fail!("unreacheable (TODO implement more bit depths)")
            }
        }

        if i < next_line {
            self.x_byte_pos = i - self.y_byte_pos;
        } else {
            let pixel_bytes = self.image.color_type.pixel_bits() / 8;

            self.x_byte_pos = x0 * pixel_bytes;
            self.y_byte_pos += dy * self.scanline_bytes;
            if self.y_byte_pos >= self.image.pixels.len() {
                match self.interlace {
                    0 | 7 => {
                        // FIXME(eddyb) free all temporary structures.
                        self.palette = None;
                        self.idat_inflate_stream = None;
                    }
                    _ => {
                        self.interlace += 1;
                        let (x0, y0, _, _) = self.interlace_params();
                        self.x_byte_pos = x0 * pixel_bytes;
                        self.y_byte_pos = y0 * pixel_bytes;
                    }
                }
            }
        }
    }
}

enum State {
    CheckMagic(/*offset*/ u8),
    U16(U16Next),
    U16Byte1(U16Next, /*value*/ u8),
    U32(U32Next, /*offset*/ u8, /*value*/ u32),
    Chunk4CC(/*size*/ u32),
    Chunk4CC1(/*size*/ u32, [u8, ..1]),
    Chunk4CC2(/*size*/ u32, [u8, ..2]),
    Chunk4CC3(/*size*/ u32, [u8, ..3]),
    IgnoreChunk(/*left*/ u32),
    IhdrBits(/*width*/ u32, /*height*/ u32),
    IhdrColorType(/*width*/ u32, /*height*/ u32, /*bits*/ u8),
    IhdrCompressionMethod(/*width*/ u32, /*height*/ u32, /*bits*/ u8, /*color_type*/ u8),
    IhdrFilterMethod(/*width*/ u32, /*height*/ u32, /*bits*/ u8, /*color_type*/ u8, /*compression_method*/ u8),
    IhdrInterlaceMethod(/*width*/ u32, /*height*/ u32, /*bits*/ u8, /*color_type*/ u8, /*compression_method*/ u8, /*filter_method*/ u8),
    Plte(/*left*/ u32),
    Trns(/*left*/ u32, /*index*/ u32),
    IdatInflate(/*left*/ u32)
}

enum U16Next {
    U16TrnsK,
    U16TrnsR,
    U16TrnsG(/*red*/ u16),
    U16TrnsB(/*red*/ u16, /*green*/ u16)
}

enum U32Next {
    U32ChunkSize,
    U32ChunkCRC(/*last_chunk*/ bool),
    U32IhdrWidth,
    U32IhdrHeight(/*width*/ u32)
}

pub struct Decoder {
    state: Option<State>,
    image: Option<PartialImage>
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            state: Some(CheckMagic(0)),
            image: None
        }
    }

    fn next_state(&mut self, data: &[u8]) -> Result<uint, ~str> {
        let b = data[0];
        macro_rules! ok2 (($n:expr, $state:expr) => ({
            self.state = Some($state);
            Ok($n as uint)
        }))
        macro_rules! ok (($state:expr) => ({
            ok2!(1, $state)
        }))
        macro_rules! ok_u32 (($next:expr) => ({
            ok!(U32($next, 0, 0))
        }))
        let skip_crc = U32(U32ChunkCRC(false), 0, 0);

        let state = match self.state {
            Some(state) => state,
            None => return Err("called png::Decoder::next_state with non-existent state".to_owned())
        };

        match state {
            CheckMagic(i) => {
                if b != MAGIC[i as uint] {
                    Err(format!("PNG header mismatch, expected {:#02x} but found {:#02x} for byte {}", MAGIC[i as uint], b, i))
                } else if i < 7 {
                    ok!(CheckMagic(i + 1))
                } else {
                    ok_u32!(U32ChunkSize)
                }
            }
            U16(next) => ok!(U16Byte1(next, b)),
            U16Byte1(next, value) => {
                let value = (value as u16 << 8) | b as u16;
                match (next, value) {
                    (U16TrnsK, k) => {
                        let image = self.image.as_mut().unwrap();
                        image.transparent_color = Some([k, k, k]);
                        ok!(skip_crc)
                    }
                    (U16TrnsR, r) => ok!(U16(U16TrnsG(r))),
                    (U16TrnsG(r), g) => ok!(U16(U16TrnsB(r, g))),
                    (U16TrnsB(r, g), b) => {
                        let image = self.image.as_mut().unwrap();
                        image.transparent_color = Some([r, g, b]);
                        ok!(skip_crc)
                    }
                }
            }
            U32(next, i, value) => {
                let value = (value << 8) | b as u32;
                if i < 3 {
                    ok!(U32(next, i + 1, value))
                } else {
                    match next {
                        U32ChunkSize => ok!(Chunk4CC(value)),
                        U32ChunkCRC(last_chunk) => {
                            // TODO(eddyb) check the CRC.
                            if last_chunk {
                                self.state = None;
                                Ok(1)
                            } else {
                                ok_u32!(U32ChunkSize)
                            }
                        }
                        U32IhdrWidth => ok_u32!(U32IhdrHeight(value)),
                        U32IhdrHeight(w) => ok!(IhdrBits(w, value))
                    }
                }
            }
            Chunk4CC(size) => ok!(Chunk4CC1(size, [b])),
            Chunk4CC1(size, [b0]) => ok!(Chunk4CC2(size, [b0, b])),
            Chunk4CC2(size, [b0, b1]) => ok!(Chunk4CC3(size, [b0, b1, b])),
            Chunk4CC3(size, [b0, b1, b2]) => {
                let name = [b0, b1, b2, b];
                let name = match from_utf8(name) {
                    Some(name) => name,
                    None => return Err(format!("non-utf8 chunk name {:?}", name))
                };
                match name {
                    "IHDR" => {
                        if self.image.is_some() {
                            Err("duplicate IHDR".to_owned())
                        } else if size != size_of::<Ihdr>() as u32 {
                            Err(format!("IHDR size mismatch, expected {} but found {}", size_of::<Ihdr>(), size))
                        } else {
                            ok_u32!(U32IhdrWidth)
                        }
                    }
                    "PLTE" => {
                        if size > 0 && size % 3 != 0 {
                            Err(format!("PLTE has non multiple of 3 size {}", size))
                        } else {
                            match self.image {
                                None => Err("PLTE before IHDR".to_owned()),
                                Some(ref mut image) => {
                                    if image.idat_inflate_stream.is_some() {
                                        Err("PLTE after IDAT".to_owned())
                                    } else if image.palette.is_some() {
                                        Err("duplicate PLTE".to_owned())
                                    } else if !image.color_type.is_palette() {
                                        // Ignore a palette that's not used to decode the image.
                                        ok!(IgnoreChunk(size))
                                    } else {
                                        image.palette = Some(Vec::with_capacity(size as uint / 3 * 4));
                                        ok!(Plte(size))
                                    }
                                }
                            }
                        }
                    }
                    "tRNS" => {
                        match self.image {
                            None => Err("tRNS before IHDR".to_owned()),
                            Some(ref mut image) => {
                                if image.idat_inflate_stream.is_some() {
                                    Err("tRNS after IDAT".to_owned())
                                } else {
                                    match image.color_type {
                                        K1 | K2 | K4 | K8 | K16 => ok!(U16(U16TrnsK)),
                                        Pal1 | Pal2 | Pal4 | Pal8 => ok!(Trns(size, 3)),
                                        RGB8 | RGB16 => ok!(U16(U16TrnsR)),
                                        _ => ok!(IgnoreChunk(size))
                                    }
                                }
                            }
                        }
                    }
                    "IDAT" => {
                        if self.image.is_none() {
                            Err("IDAT before IHDR".to_owned())
                        } else if self.image.as_ref().unwrap().color_type.is_palette()
                            && self.image.as_ref().unwrap().palette.is_none() {
                            Err("IDAT before PLTE".to_owned())
                        } else {
                            let stream = &mut self.image.as_mut().unwrap().idat_inflate_stream;
                            if stream.is_none() {
                                *stream = Some(box InflateStream::from_zlib());
                            }
                            ok!(IdatInflate(size))
                        }
                    }
                    "IEND" => ok_u32!(U32ChunkCRC(true)),
                    // TODO(eddyb) maybe save the data?
                    "tEXt" | "iTXt" | "iCCP" | "pHYs" | "gAMA" | "cHRM" | "sBIT" | "sRGB" | "bKGD" => ok!(IgnoreChunk(size)),
                    name => {
                        error!("skipping unrecognized PNG chunk `{}` (size={})", name, size);
                        ok!(IgnoreChunk(size))
                    }
                }
            }
            IgnoreChunk(left) => {
                let n = min(left, data.len() as u32);
                if left > n {
                    ok2!(n, IgnoreChunk(left - n))
                } else {
                    ok2!(n, skip_crc)
                }
            }
            IhdrBits(w, h) => ok!(IhdrColorType(w, h, b)),
            IhdrColorType(w, h, bits) => ok!(IhdrCompressionMethod(w, h, bits, b)),
            IhdrCompressionMethod(w, h, bits, c) => ok!(IhdrFilterMethod(w, h, bits, c, b)),
            IhdrFilterMethod(w, h, bits, c, z) => ok!(IhdrInterlaceMethod(w, h, bits, c, z, b)),
            IhdrInterlaceMethod(w, h, bits, c, z, f) => {
                let header = Ihdr {
                    width: w,
                    height: h,
                    bits: bits,
                    color_type: c,
                    compression_method: z,
                    filter_method: f,
                    interlace_method: b
                };
                match header.to_image() {
                    Ok(image) => {
                        self.image = Some(image);
                        ok!(skip_crc)
                    }
                    Err(m) => Err(m)
                }
            }
            Plte(left) => {
                let n = min(left, data.len() as u32);
                let image = self.image.as_mut().unwrap();
                let palette = image.palette.as_mut().unwrap();
                for &x in data.slice_to(n as uint).iter() {
                    palette.push(x);

                    if palette.len() % 4 == 3 {
                        palette.push(0xff);
                    }
                }
                if left > n {
                    ok2!(n, Plte(left - n))
                } else {
                    ok2!(n, skip_crc)
                }
            }
            Trns(left, mut i) => {
                let n = min(left, data.len() as u32);
                let image = self.image.as_mut().unwrap();
                let palette = image.palette.as_mut().unwrap();
                for &x in data.slice_to(n as uint).iter() {
                    *palette.get_mut(i as uint) = x;
                    i += 4;
                }
                if left > n {
                    ok2!(n, Trns(left - n, i))
                } else {
                    ok2!(n, skip_crc)
                }
            }
            IdatInflate(left) => {
                let mut n = min(left, data.len() as u32);
                let image = self.image.as_mut().unwrap();
                let mut stream = image.idat_inflate_stream.take_unwrap();
                match stream.update(data.slice_to(n as uint)) {
                    Ok((used, output)) => {
                        match image.update_idat(output) {
                            Err(m) => return Err(format!("IDAT error: {:s}", m)),
                            _ => {}
                        }
                        n = used as u32;
                    }
                    Err(m) => return Err(format!("IDAT decompression error: {:s}", m))
                }
                // FIXME(eddyb) don't put back if it's no longer required.
                image.idat_inflate_stream = Some(stream);
                if left > n {
                    ok2!(n, IdatInflate(left - n))
                } else {
                    ok2!(n, skip_crc)
                }
            }
        }
    }

    pub fn update<'a>(&'a mut self, mut data: &[u8]) -> ImageState<'a> {
        while data.len() > 0 {
            match self.next_state(data) {
                Ok(n) => { data = data.slice_from(n); }
                Err(m) => return Error(m)
            }
        }
        Partial(self.image.as_ref().map(|partial| &partial.image))
    }
}

pub trait DecoderRef {
    fn update<'a>(&'a mut self, data: &[u8]) -> ImageState<'a>;
}

impl DecoderRef for Option<Box<Decoder>> {
    fn update<'a>(&'a mut self, data: &[u8]) -> ImageState<'a> {
        match self.take() {
            Some(mut decoder) => {
                match decoder.update(data) {
                    Partial(_) if decoder.state.is_some() => {
                        *self = Some(decoder);
                        Partial(self.as_ref().unwrap().image.as_ref().map(|partial| &partial.image))
                    }
                    Error(m) => Error(m),
                    _ => Complete(decoder.image.take_unwrap().image)
                }
            }
            None => Error("called Option<~png::Decoder>::update on None".to_owned())
        }
    }
}

/*
pub fn is_png(image: &[u8]) -> bool {
    do image.as_imm_buf |bytes, _len| {
        unsafe {
            ffi::png_sig_cmp(bytes, 0, 8) == 0
        }
    }
}*/

#[allow(dead_code)]
#[allow(deprecated_owned_vector)]
pub fn load_png(path: &Path) -> Result<Image, ~str> {
    match File::open_mode(path, io::Open, io::Read) {
        Ok(mut r) => match r.read_to_end() {
            Ok(data) => load_png_from_memory(data.as_slice()),
            Err(m) => Err(m.to_str())
        },
        Err(m) => Err(m.to_str())
    }
}

#[allow(dead_code)]
pub fn load_png_from_memory(image: &[u8]) -> Result<Image, ~str> {
    let mut decoder = Some(box Decoder::new());
    match decoder.update(image) {
        Partial(_) => Err("incomplete PNG file".to_owned()),
        Complete(image) => Ok(image),
        Error(m) => Err(m)
    }
}

#[cfg(test)]
mod test {
    use extra::test::{bench, fmt_bench_samples};
    use std::io;
    use std::io::File;
    use std::vec;
    use super::{load_png, load_png_from_memory, ColorType, RGBA8, KA8, Decoder, DecoderRef, Partial, Complete, Error};

    fn load_rgba8(file: &'static str, w: u32, h: u32) {
        match load_png(&Path::new(file)) {
            Err(m) => fail!(m),
            Ok(image) => {
                assert_eq!(image.color_type, RGBA8);
                assert_eq!(image.width, w);
                assert_eq!(image.height, h);
            }
        }
    }

    #[test]
    fn test_load() {
        load_rgba8("test.png", 831, 624);
        load_rgba8("test_store.png", 10, 10);
    }

    fn load_rgba8_in_chunks(file: &'static str, chunk_size: uint, w: u32, h: u32) {
        spawn(proc() {
            let mut reader = match File::open_mode(&Path::new(file), io::Open, io::Read) {
                Ok(r) => r,
                Err(m) => fail!("could not open file {}", m),
            };

            let mut buf = Vec::from_elem(chunk_size, 0u8);
            let mut decoder = Some(box Decoder::new());
            loop {
                match reader.read(buf.mut_slice(0, chunk_size)) {
                    Ok(count) => match decoder.update(buf.slice_to(count)) {
                        Partial(_) => {}
                        Complete(image) => {
                            assert_eq!(image.color_type, RGBA8);
                            assert_eq!(image.width, w);
                            assert_eq!(image.height, h);
                            break;
                        },
                        Error(m) => fail!(m)
                    },
                    Err(m) => fail!("incomplete PNG file {}", m)
                }
            }
        })
    }

    #[test]
    fn test_load_big_parallel() {
        // HACK(eddyb) arbitrary values.
        for _ in range(0, 128) {
            load_rgba8_in_chunks("test.png", 1024, 831, 624);
        }
    }

    fn bench_file_from_memory(file: &'static str, w: u32, h: u32, c: ColorType) {
        let mut reader = match File::open_mode(&Path::new(file), io::Open, io::Read) {
            Ok(r) => r,
            Err(m) => fail!("could not open '{}' {}", file, m)
        };
        let buf = reader.read_to_end().ok().unwrap();
        let bs = bench::benchmark(|b| b.iter(|| {
            match load_png_from_memory(buf) {
                Err(m) => fail!(m),
                Ok(image) => {
                    assert_eq!(image.color_type, c);
                    assert_eq!(image.width, w);
                    assert_eq!(image.height, h);
                }
            }
        }));
        println!("png load '{}': {}", file, fmt_bench_samples(&bs));
    }

    #[test]
    fn test_load_perf() {
        bench_file_from_memory("test/loading.png", 1326, 1079, RGBA8);
        bench_file_from_memory("test/store.png", 10, 10, RGBA8);
        bench_file_from_memory("test/servo-screenshot.png", 831, 624, RGBA8);
        bench_file_from_memory("test/mozilla-dinosaur-head-logo.png", 1300, 929, RGBA8);
        bench_file_from_memory("test/rust-huge-logo.png", 4000, 4000, KA8);
    }
}
