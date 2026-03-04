mod app;
mod core;
mod ui;

use std::path::PathBuf;

use iced::Size;

fn main() -> iced::Result {
    // Determine the base path (the rustic-vault directory)
    let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    let base_path = exe_path
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| {
            // Fallback: use the current working directory
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        });

    // Ensure profiles directory exists
    let profiles_dir = base_path.join("profiles");
    let _ = std::fs::create_dir_all(&profiles_dir);

    iced::application(
        {
            let base = base_path.clone();
            move || app::App::new(base.clone())
        },
        app::App::update,
        app::App::view,
    )
    .subscription(app::App::subscription)
    .theme(app::App::theme)
    .window_size(Size::new(1000.0, 700.0))
    .run()
}
