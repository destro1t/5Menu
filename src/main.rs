mod config;
mod themes;
mod ui;

use iced::{Settings, window, Application, Font};
use std::default::Default;
use ui::MenuWindow;

fn main() -> iced::Result {
    env_logger::init();

    let config = config::Config::load().unwrap_or_default();
    
    let window_settings = window::Settings {
        size: (config.width, config.height),
        position: window::Position::Centered,
        min_size: None,
        max_size: None,
        visible: true,
        resizable: false,
        decorations: false,
        transparent: true,
        level: window::Level::AlwaysOnTop,
        platform_specific: window::PlatformSpecific::default(),
        icon: None,
    };

    let settings = Settings {
        window: window_settings,
        flags: (),
        default_font: Font::MONOSPACE,
        default_text_size: config.font_size as f32,
        antialiasing: true,
        ..Default::default()
    };

    MenuWindow::run(settings)?;
    Ok(())
}
