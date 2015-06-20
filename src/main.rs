
#[macro_use]
extern crate log;
extern crate fern;
extern crate sfml;

mod iconset;
mod window;
mod text_window;
mod game_settings_manager;
mod database_items;

use sfml::system::{Clock};
use sfml::graphics::{Drawable, RenderStates, Text, RenderWindow, Color, RenderTarget};
use sfml::window::{VideoMode, ContextSettings, event, window_style, Key};

use game_settings_manager::GameSettingsManager;


const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 600;

fn main() {
    // Create a basic logger configuration
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg, level, _location| {
            // This format just displays [{level}] {message}
            format!("[{}] {}", level, msg)
        }),
        // Output to stdout
        output: vec![fern::OutputConfig::stdout()],
        // Only log messages Info and above
        level: log::LogLevelFilter::Info,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }

    let settings = ContextSettings::default();
    let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_HEIGHT, WINDOW_WIDTH, 32), "SFML", window_style::CLOSE, &settings).unwrap();
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);

    let settings = GameSettingsManager::new("assets/fonts/sansation.ttf", 18, "assets/textures/icons.png", "assets/textures/window_skin.png");

    let msg = "Hello my name is \\p[30]L\\p[30]i\\p[30]l\\p[30]l\\p[30]y\nI like \\i[70]\\p[10]\\i[71]\\p[10]\\i[72]";

    let mut text_window = text_window::TextWindow::new(&settings, msg, 0.0, 50.0, 500.0, 200.0);

    let mut clock = Clock::new();

    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    Key::Escape => {
                        window.close();
                        break;
                    },
                    Key::A => {
                        text_window.reset();
                        text_window.set_state(window::WindowState::Closing);
                    },
                    Key::S => {
                        text_window.set_state(window::WindowState::Opening);
                    },
                    Key::D => {
                        text_window.change_text("Test 2");
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        // FPS counter
        let delta_time = clock.restart().as_seconds();
        let fps = 1.0 / delta_time;
        let fps_text = Text::new_init(&fps.to_string(), &settings.font, 24).unwrap();


        // Display things on screen
        window.clear(&Color::black());

        // Update and draw textbox
        text_window.update();
        window.draw(&text_window);

        // Draw FPS counter
        window.draw_text(&fps_text, &mut RenderStates::default());

        window.display();
    }
}
