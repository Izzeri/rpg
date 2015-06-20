use sfml::graphics::{Font, Texture};
use iconset::Iconset;

pub struct GameSettingsManager {
    pub font: Font,
    pub default_font_size: u32,
    pub iconset: Iconset,
    pub window_skin: Texture,

}

impl GameSettingsManager {
    pub fn new(font_filename: &str, font_size: u32, iconset_filename: &str, window_skin_filename: &str) -> GameSettingsManager {
        GameSettingsManager {
            font: Font::new_from_file(font_filename).expect("Failed to load font file"),
            default_font_size: font_size,
            iconset: Iconset::new(Texture::new_from_file(iconset_filename).expect("Failed to load iconset texture"), 24),
            window_skin: Texture::new_from_file(window_skin_filename).expect("Failed to load window skin texture"),
        }
    }
}
