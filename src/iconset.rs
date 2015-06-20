use sfml::graphics::{Texture, Sprite, IntRect};
use sfml::system::{Vector2u};

pub struct Iconset {
    texture: Texture,
    icon_size: u32,
}

impl Iconset {
    pub fn new(texture: Texture, icon_size: u32) -> Iconset {
        Iconset {
            texture: texture,
            icon_size: icon_size,
        }
    }

    pub fn  get(&self, index: usize) -> Sprite {
        let Vector2u {x: w, y: h} = self.texture.get_size();
        let cols = w / self.icon_size;
        let rows = h / self.icon_size;
        let mut row = index / cols as usize;
        let mut col = index % cols as usize;

        // Default to (0, 0) if the index is bigger than the amount of icons in the iconset
        if index > rows as usize * cols as usize - 1 {
            error!("Illegal icon index {}, defaulting to 0", index);
            row = 0;
            col = 0;
        }

        let rect = IntRect::new(col as i32 * self.icon_size as i32, row as i32 * self.icon_size as i32, self.icon_size as i32, self.icon_size as i32);
        let mut sprite = Sprite::new_with_texture(&self.texture).unwrap();
        sprite.set_texture_rect(&rect);

        return sprite;
    }
}
