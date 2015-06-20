use std::str::FromStr;

use sfml::graphics::{Transformable, Text, Drawable, Color, RenderTarget, RenderStates};

use window::{WindowState, Window};
use game_settings_manager::GameSettingsManager;
use iconset::Iconset;

// A window designed to show text
// Supports a few escape codes to show other things, or change the text properties
pub struct TextWindow<'a> {
    window: Window<'a>,
    text: Text<'a>,
    current_text: String,
    target_text: String,
    current_character: usize,
    state: TextWindowState,
    current_pos_x: f32,
    current_pos_y: f32,
    iconset: &'a Iconset,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TextWindowState {
    Typing,
    Paused(u32), // Pause duration left in frames
    Done,
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TextCode {
    Icon(usize),
    ChangeColor(Color),
    ChangeSize(u32),
    Pause(u32),
    Unknown,
}

impl<'a> TextWindow<'a> {

    pub fn new(settings: &'a GameSettingsManager, text: &str, x: f32, y: f32, width: f32, height: f32) -> TextWindow<'a> {
        TextWindow {
            window: Window::new(&settings.window_skin, x, y, width, height),
            text: Text::new_init("", &settings.font, settings.default_font_size).unwrap(),
            current_text: "".to_string(),
            target_text: text.to_string(),
            current_character: 0,
            state: TextWindowState::Typing,
            current_pos_x: 0.0,
            current_pos_y: 0.0,
            iconset: &settings.iconset,
        }
    }

    pub fn update(&mut self) {
        // If window is open, do updates
        if self.window.get_state() == WindowState::Static && self.window.is_open() {
            match self.state {
                // Draw the next character
                TextWindowState::Typing => {
                    self.current_character += 1;
                    let character = self.target_text.chars().take(self.current_character).last().unwrap();

                    if character == '\\' {
                        let code = self.parse_escape_code();
                        self.execute_code(code);
                    } else if character == '\n' {
                        self.current_pos_y += self.text.get_character_size() as f32;
                        self.current_pos_x = 0.0;
                    } else {
                        // Draw the next character if there's no escape sequence
                        self.current_text.push(character);

                        self.text.set_string(&character.to_string());

                        self.text.set_position2f(self.current_pos_x, self.current_pos_y);
                        self.current_pos_x += self.text.get_local_bounds().width;

                        self.window.get_contents().draw(&self.text);
                    }


                    if self.current_character == self.target_text.len() {
                        self.state = TextWindowState::Done;
                    }
                },

                // Pause for a frame and decrease duration
                TextWindowState::Paused(duration) => {
                    if duration <= 0 {
                        self.state = TextWindowState::Typing;
                    } else {
                        self.state = TextWindowState::Paused(duration - 1);
                    }
                },

                // Do nothing
                TextWindowState::Done => { },
            }
        }

        self.window.update();
    }

    fn parse_escape_code(&mut self) -> TextCode {
        let mut code = self.target_text.chars().skip(self.current_character).take_while(|&c| c != ']' );
        let length = self.target_text.chars().skip(self.current_character).take_while(|&c| c != ']' ).collect::<String>().len() + 1;

        // Add the length of the whole escape sequence to to the current character to skip them from being printed
        self.current_character += length;

        let code_type = code.next().unwrap();

        let code_args = code.skip(1).collect::<String>();
        let code_args = code_args.split(',').collect::<Vec<_>>();

        let mut result = TextCode::Unknown;

        match code_type {
            // Draw icon
            'i' => {
                    let arg = usize::from_str(code_args[0]);

                    if arg.is_err() {
                        error!("Invalid number string {} in Icon text code, defaulting to 0", code_args[0]);
                        result = TextCode::Icon(0);
                    } else {
                        result = TextCode::Icon(arg.unwrap());
                    }
                },

            // Change text color
            'c' => {
                    let arg = u32::from_str(code_args[0]);

                    if arg.is_err() {
                        error!("Invalid number string {} in Color text code, defaulting to 0", code_args[0]);
                        result = TextCode::ChangeColor(self.window.get_color(0));
                    } else {
                        result = TextCode::ChangeColor(self.window.get_color(arg.unwrap()));
                    }
                },

            // Change text size
            's' => {
                    let arg = u32::from_str(code_args[0]);

                    if arg.is_err() {
                        error!("Invalid number string {} in Size text code, defaulting to 18", code_args[0]);
                        result = TextCode::ChangeSize(18);
                    } else {
                        result = TextCode::ChangeSize(arg.unwrap());
                    }
                },

            // Pause
            'p' => {
                    let arg = u32::from_str(code_args[0]);

                    if arg.is_err() {
                        error!("Invalid number string {} in Pause text code, defaulting to 0", code_args[0]);
                        result = TextCode::Pause(0);
                    } else {
                        result = TextCode::Pause(arg.unwrap());
                    }
                },

            // Invalid code
            _ => result = TextCode::Unknown,
        };

        return result;
    }

    fn execute_code(&mut self, code: TextCode) {
        match code {
            // Draw icon
            TextCode::Icon(id) => {
                let mut icon = self.iconset.get(id);
                icon.set_position2f(self.current_pos_x, self.current_pos_y);
                self.window.get_contents().draw(&icon);

                self.current_pos_x += icon.get_local_bounds().width;

                info!("Drawing icon #{:?}", id);
            },

            // Change text color
            TextCode::ChangeColor(color) => {
                self.text.set_color(&color);
                info!("Changing color to {:?}", color);
            },

            // Change text size
            TextCode::ChangeSize(size) => {
                self.text.set_character_size(size);
                info!("Changing size to {:?}", size);
            },

            // Pause
            TextCode::Pause(duration) => {
                self.state = TextWindowState::Paused(duration);

                info!("Pausing for {:?}", duration);
            },

            // Do nothing
            TextCode::Unknown => { },
        }
    }

    // Clear the textbox
    pub fn reset(&mut self) {
        self.state = TextWindowState::Typing;
        self.current_text = "".to_string();
        self.text.set_string(&self.current_text);
        self.current_character = 0;
        self.current_pos_x = 0.0;
        self.current_pos_y = 0.0;
        self.window.get_contents().clear(&Color::new_rgba(0, 0, 0, 0));
    }

    pub fn change_text(&mut self, new_text: &str) {
        self.reset();
        self.target_text = new_text.to_string();
    }

    /// Change the state (Opening, Static, Closing) of the window
    pub fn set_state(&mut self, new_state: WindowState) {
        self.window.set_state(new_state);
    }

    //

}

impl<'a> Drawable for TextWindow<'a> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, rs: &mut RenderStates) {
        target.draw_with_renderstates(&self.window, rs);
    }
}
