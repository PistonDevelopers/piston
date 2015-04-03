
//! Back-end agnostic mouse buttons.

use num::{ FromPrimitive, ToPrimitive };

/// Represent a mouse button.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq,
    Eq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    /// Unknown mouse button.
    Unknown,
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Extra mouse button number 1.
    X1,
    /// Extra mouse button number 2.
    X2,
    /// Mouse button number 6.
    Button6,
    /// Mouse button number 7.
    Button7,
    /// Mouse button number 8.
    Button8,
}

impl FromPrimitive for MouseButton {
    fn from_u64(n: u64) -> Option<MouseButton> {
        match n {
            0 => Some(MouseButton::Unknown),
            1 => Some(MouseButton::Left),
            2 => Some(MouseButton::Right),
            3 => Some(MouseButton::Middle),
            4 => Some(MouseButton::X1),
            5 => Some(MouseButton::X2),
            6 => Some(MouseButton::Button6),
            7 => Some(MouseButton::Button7),
            8 => Some(MouseButton::Button8),
            _ => Some(MouseButton::Unknown),
        }
    }

    #[inline(always)]
    fn from_i64(n: i64) -> Option<MouseButton> {
        FromPrimitive::from_u64(n as u64)
    }

    #[inline(always)]
    fn from_isize(n: isize) -> Option<MouseButton> {
        FromPrimitive::from_u64(n as u64)
    }
}

impl ToPrimitive for MouseButton {
    fn to_u64(&self) -> Option<u64> {
        match self {
            &MouseButton::Unknown => Some(0),
            &MouseButton::Left => Some(1),
            &MouseButton::Right => Some(2),
            &MouseButton::Middle => Some(3),
            &MouseButton::X1 => Some(4),
            &MouseButton::X2 => Some(5),
            &MouseButton::Button6 => Some(6),
            &MouseButton::Button7 => Some(7),
            &MouseButton::Button8 => Some(8), 
        }
    }

    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        self.to_u64().map(|x| x as i64)
    }

    #[inline(always)]
    fn to_isize(&self) -> Option<isize> {
        self.to_u64().map(|x| x as isize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_button_primitives() {
        use num::{ FromPrimitive, ToPrimitive };

        for i in 0u64..9 {
            let button: MouseButton = FromPrimitive::from_u64(i).unwrap();
            let j = ToPrimitive::to_u64(&button).unwrap();
            assert_eq!(i, j);
        }
    }
}
