use sfml::graphics::{Drawable, Transformable, Sprite, RenderTexture, Texture, Color, RenderTarget, IntRect, RenderStates};
use sfml::system::vector2::Vector2f;

const CONTENT_PADDING: u32 = 5;

#[derive(PartialEq, Clone, Copy)]
pub enum WindowState {
    Opening,
    Static,
    Closing,
}

pub struct Window<'a> {
    skin: &'a Texture,
    window: RenderTexture,
    contents: RenderTexture,
    size: Vector2f,
    position: Vector2f,
    content_position: Vector2f,
    visible: bool,
    window_opacity: f32,
    content_opacity: f32,
    openness: f32,
    state: WindowState,
}


impl<'a> Window<'a> {
    /// Returns a new Window
    pub fn new(skin: &'a Texture, x: f32, y: f32, width: f32, height: f32) -> Window<'a> {
        let mut window = Window {
            skin: skin,
            window: RenderTexture::new(width as u32, height as u32, false).unwrap(),
            contents: RenderTexture::new(width as u32 - CONTENT_PADDING * 2, height as u32 - CONTENT_PADDING * 2, false).unwrap(),
            size: Vector2f::new(width, height),
            position: Vector2f::new(x, y),
            content_position: Vector2f::new(4.0, 4.0),
            visible: true,
            window_opacity: 1.0,
            content_opacity: 1.0,
            openness: 0.0,
            state: WindowState::Opening,
        };

        window.produce_window();

        return window;
    }

    /// Calculates and produces the window texture
    fn produce_window(&mut self) {
        let mut top_left_corner = Sprite::new_with_texture(self.skin).unwrap();
        top_left_corner.set_texture_rect(&IntRect::new(64, 0, 16, 16));
        top_left_corner.set_position2f(0.0, 0.0);

        let mut top_right_corner = Sprite::new_with_texture(self.skin).unwrap();
        top_right_corner.set_texture_rect(&IntRect::new(112, 0, 16, 16));
        top_right_corner.set_position2f(self.size.x - 16.0, 0.0);

        let mut bottom_left_corner = Sprite::new_with_texture(self.skin).unwrap();
        bottom_left_corner.set_texture_rect(&IntRect::new(64, 48, 16, 16));
        bottom_left_corner.set_position2f(0.0, self.size.y - 16.0);

        let mut bottom_right_corner = Sprite::new_with_texture(self.skin).unwrap();
        bottom_right_corner.set_texture_rect(&IntRect::new(112, 48, 16, 16));
        bottom_right_corner.set_position2f(self.size.x - 16.0, self.size.y - 16.0);

        let mut background = Sprite::new_with_texture(self.skin).unwrap();
        background.set_texture_rect(&IntRect::new(0, 0, 64, 64));
        background.set_position2f(CONTENT_PADDING as f32 / 2.0, CONTENT_PADDING as f32 / 2.0);
        background.scale2f((self.size.x - CONTENT_PADDING as f32) / 64.0, (self.size.y - CONTENT_PADDING as f32) / 64.0);

        let mut top_edge = Sprite::new_with_texture(self.skin).unwrap();
        top_edge.set_texture_rect(&IntRect::new(80, 0, 32, 16));
        top_edge.set_position2f(16.0, 0.0);
        top_edge.scale2f((self.size.x - 32.0) / 32.0, 1.0);

        let mut bottom_edge = Sprite::new_with_texture(self.skin).unwrap();
        bottom_edge.set_texture_rect(&IntRect::new(80, 48, 32, 16));
        bottom_edge.set_position2f(16.0, self.size.y - 16.0);
        bottom_edge.scale2f((self.size.x - 32.0) / 32.0, 1.0);

        let mut left_edge = Sprite::new_with_texture(self.skin).unwrap();
        left_edge.set_texture_rect(&IntRect::new(64, 16, 16, 32));
        left_edge.set_position2f(0.0, 16.0);
        left_edge.scale2f(1.0, (self.size.y - 32.0) / 32.0);

        let mut right_edge = Sprite::new_with_texture(self.skin).unwrap();
        right_edge.set_texture_rect(&IntRect::new(112, 16, 16, 32));
        right_edge.set_position2f(self.size.x - 16.0, 16.0);
        right_edge.scale2f(1.0, (self.size.y - 32.0) / 32.0);

        self.window.draw(&background);

        self.window.draw(&top_left_corner);
        self.window.draw(&top_right_corner);
        self.window.draw(&bottom_left_corner);
        self.window.draw(&bottom_right_corner);

        self.window.draw(&top_edge);
        self.window.draw(&bottom_edge);
        self.window.draw(&left_edge);
        self.window.draw(&right_edge);
    }

    /// Returns a reference to the RenderTexture of the contents of the window
    pub fn get_contents(&mut self) -> &mut RenderTexture {
        &mut self.contents
    }

    /// Update the graphic of the window
    pub fn update(&mut self) {
        // Animate opening
        if self.state == WindowState::Opening {
            if self.openness < 1.0 {
                self.openness += 0.2;
            }

            if self.openness >= 1.0 {
                self.openness = 1.0;
                self.state = WindowState::Static;
            }
        }

        // Animate closing
        if self.state == WindowState::Closing {
            if self.openness > 0.0 {
                self.openness -= 0.2;
            }

            if self.openness <= 0.0 {
                self.openness = 0.0;
                self.state = WindowState::Static;
            }
        }
    }

    /// Change the state (Opening, Static, Closing) of the window
    pub fn set_state(&mut self, new_state: WindowState) {
        self.state = new_state;
    }

    /// Visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn get_state(&self) -> WindowState {
        self.state
    }

    pub fn is_open(&self) -> bool {
        self.openness >= 1.0
    }

    pub fn get_color(&self, id: u32) -> Color {
        // There are 8 * 4 = 32 colors in the window skin
        if id >= 32 {
            error!("Illegal color ID {}, defaulting to 0", id);
            return self.get_color(0);
        }

        let row = id / 8;
        let col = id % 8;

        let pixel_x = 64 + col * 8;
        let pixel_y = 96 + row * 8;

        // Return color of the pixel
        self.skin.copy_to_image().unwrap().get_pixel(pixel_x, pixel_y)
    }
}

impl<'a> Drawable for Window<'a> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, rs: &mut RenderStates) {
        // Finalize drawings on the textures
        self.window.display();
        self.contents.display();

        let window_opacity = self.window_opacity * 255.0;
        let content_opacity = self.content_opacity * 255.0;

        let window_texture = &self.window.get_texture().unwrap();
        let mut window_sprite = Sprite::new_with_texture(&window_texture).unwrap();
        window_sprite.set_origin2f(self.size.x / 2.0, self.size.y / 2.0);
        window_sprite.set_position2f(self.size.x / 2.0 + self.position.x, self.size.y / 2.0 + self.position.y);
        window_sprite.set_scale2f(1.0, self.openness);
        window_sprite.set_color(&Color::new_rgba(255, 255, 255, window_opacity as u8));

        let content_texture = &self.contents.get_texture().unwrap();
        let mut content_sprite = Sprite::new_with_texture(&content_texture).unwrap();
        content_sprite.set_origin2f(self.size.x / 2.0, self.size.y / 2.0);
        content_sprite.set_position2f(self.size.x / 2.0 + self.content_position.x + self.position.x,
                                      self.size.y / 2.0 + self.content_position.y + self.position.y);
        content_sprite.set_scale2f(1.0, self.openness);
        content_sprite.set_color(&Color::new_rgba(255, 255, 255, content_opacity as u8));

        target.draw_with_renderstates(&window_sprite, rs);
        target.draw_with_renderstates(&content_sprite, rs);
    }
}
