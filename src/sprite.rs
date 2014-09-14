
use std::rc::Rc;
use std::collections::hashmap::HashMap;

use uuid::Uuid;

use graphics::*;
use graphics::internal::{
    Rectangle,
    Vec2d,
};
use graphics::vecmath::Scalar;

/// A sprite is a texture with some properties.
pub struct Sprite<I: ImageSize> {
    id: Uuid,

    visible: bool,

    anchor: Vec2d,

    position: Vec2d,
    rotation: Scalar,
    scale: Vec2d,

    flip_x: bool,
    flip_y: bool,

    opacity: f32,

    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, uint>,

    texture: Rc<I>,
}

impl<I: ImageSize> Sprite<I> {
    /// Crate sprite from a texture
    pub fn from_texture(texture: Rc<I>) -> Sprite<I> {
        Sprite {
            id: Uuid::new_v4(),

            visible: true,

            anchor: [0.5, 0.5],

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],

            flip_x: false,
            flip_y: false,

            opacity: 1.0,

            texture: texture,

            children: Vec::new(),
            children_index: HashMap::new(),
        }
    }

    /// Get the sprite's id
    #[inline(always)]
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Whether or not the sprite is visible
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Set the sprite's visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Get the sprite's anchor point
    ///
    /// The value is normalized. Default value is [0.5, 0.5] (the center of texture)
    #[inline(always)]
    pub fn anchor(&self) -> (Scalar, Scalar) {
        (self.anchor[0], self.anchor[1])
    }

    /// Set the sprite's anchor point
    #[inline(always)]
    pub fn set_anchor(&mut self, x: Scalar, y: Scalar) {
        self.anchor = [x, y];
    }

    /// Get the sprite's position
    #[inline(always)]
    pub fn position(&self) -> (Scalar, Scalar) {
        (self.position[0], self.position[1])
    }

    /// Set the sprite's position
    #[inline(always)]
    pub fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.position = [x, y];
    }

    /// Get the sprite's rotation (in degree)
    #[inline(always)]
    pub fn rotation(&self) -> Scalar {
        self.rotation
    }

    /// Set the sprite's rotation (in degree)
    #[inline(always)]
    pub fn set_rotation(&mut self, deg: Scalar) {
        self.rotation = deg;
    }

    /// Get the sprite's scale
    #[inline(always)]
    pub fn scale(&self) -> (Scalar, Scalar) {
        (self.scale[0], self.scale[1])
    }

    /// Set the sprite's scale
    #[inline(always)]
    pub fn set_scale(&mut self, sx: Scalar, sy: Scalar) {
        self.scale = [sx, sy];
    }

    /// Whether or not the sprite is flipped horizontally.
    ///
    /// It only flips the texture of the sprite,
    /// and not the texture of the sprite’s children.
    ///
    /// Also, flipping the texture doesn’t alter the `anchor`.
    ///
    /// If you want to flip the `anchor` too,
    /// and/or to flip the children too use: sprite.scale.x *= -1;
    #[inline(always)]
    pub fn flip_x(&self) -> bool {
        self.flip_x
    }

    /// Flip the sprite
    #[inline(always)]
    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }

    /// Whether or not the sprite is flipped vertically.
    ///
    /// It only flips the texture of the sprite,
    /// and not the texture of the sprite’s children.
    ///
    /// Also, flipping the texture doesn’t alter the `anchor`.
    ///
    /// If you want to flip the `anchor` too,
    /// and/or to flip the children too use: sprite.scale.y *= -1;
    #[inline(always)]
    pub fn flip_y(&self) -> bool {
        self.flip_y
    }

    /// Flip the sprite
    #[inline(always)]
    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }

    /// Get the sprite's opacity
    #[inline(always)]
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Set the sprite's opacity
    #[inline(always)]
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
    }

    /// Get the sprite's texture
    #[inline(always)]
    pub fn texture(&self) -> &Rc<I> {
        &self.texture
    }

    /// Set the sprite's texture
    #[inline(always)]
    pub fn set_texture(&mut self, texture: Rc<I>) {
        self.texture = texture;
    }

    /// Add a sprite as the child of this sprite, return the added sprite's id.
    pub fn add_child(&mut self, sprite: Sprite<I>) -> Uuid {
        let id = sprite.id();
        self.children.push(sprite);
        self.children_index.insert(id, self.children.len() - 1);
        id
    }

    /// Remove the child by `id` from this sprite's children or grandchild
    pub fn remove_child(&mut self, id: Uuid) -> Option<Sprite<I>> {
        match self.children_index.pop(&id) {
            Some(i) => {
                let removed = self.children.remove(i).unwrap();
                // Removing a element of vector will alter the index,
                // update the mapping from uuid to index.
                for index in range(i, self.children.len()) {
                    let uuid = self.children[index].id();
                    self.children_index.insert(uuid, index);
                }
                Some(removed)
            },
            None => {
                for child in self.children.mut_iter() {
                    match child.remove_child(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }

    /// Find the child by `id` from this sprite's children or grandchild
    pub fn child(&self, id: Uuid) -> Option<&Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(&self.children[*i]) },
            None => {
                for child in self.children.iter() {
                    match child.child(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }

    /// Find the child by `id` from this sprite's children or grandchild, mutability
    pub fn child_mut(&mut self, id: Uuid) -> Option<&mut Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(self.children.get_mut(*i)) },
            None => {
                for child in self.children.mut_iter() {
                    match child.child_mut(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }

    /// Draw this sprite and its children
    pub fn draw<B: BackEnd<I>>(&self, c: &Context, b: &mut B) {
        if !self.visible {
            return;
        }

        let (w, h) = self.texture.get_size();
        let w = w as f64;
        let h = h as f64;
        let anchor = [self.anchor[0] * w, self.anchor[1] * h];

        let transformed = c.trans(self.position[0], self.position[1])
                           .rot_deg(self.rotation)
                           .scale(self.scale[0], self.scale[1]);

        let mut model = transformed.rect(-anchor[0],
                                         -anchor[1],
                                          w,
                                          h);

        if self.flip_x {
            model = model.trans(w - 2.0 * anchor[0], 0.0).flip_h();
        }

        if self.flip_y {
            model = model.trans(0.0, h - 2.0 * anchor[1]).flip_v();
        }

        // for debug: bounding_box
        //model.rgb(1.0, 0.0, 0.0).draw(b);

        model.rgba(1.0, 1.0, 1.0, self.opacity).image(&*self.texture).draw(b);

        // for debug: anchor point
        //c.trans(self.position[0], self.position[1]).rect(-5.0, -5.0, 10.0, 10.0).rgb(0.0, 0.0, 1.0).draw(b);

        for child in self.children.iter() {
            child.draw(&transformed, b);
        }
    }

    /// Get the sprite's bounding box
    pub fn bounding_box(&self) -> Rectangle {
        let (w, h) = self.texture.get_size();
        let w = w as f64 * self.scale[0];
        let h = h as f64 * self.scale[1];

        [
            self.position[0] - self.anchor[0] * w,
            self.position[1] - self.anchor[1] * h,
            w,
            h
        ]
    }
}

