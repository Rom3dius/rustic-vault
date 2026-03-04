use iced::widget::{button, column, container, pick_list, row, rule, scrollable, space, text, text_input};
use iced::{Element, Length};

use crate::app::{App, Message};
use crate::ui::help;
use crate::ui::widgets::{card, THEME_NAMES};

pub fn view(app: &App) -> Element<'_, Message> {
    let mut content = column![
        row![
            text("Settings").size(24),
            space::horizontal(),
            button(text("Home").size(14)).on_press(Message::GoHome),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
        rule::horizontal(1),
    ]
    .spacing(12)
    .padding(24);

    // Password section
    let save_label = if app.config.save_password {
        "[x] Save password to disk"
    } else {
        "[ ] Save password to disk"
    };

    let password_display = if app.password_visible {
        app.password.clone()
    } else {
        "••••••••••••".to_string()
    };

    let password_section = column![
        row![
            text(format!("Password: {password_display}")).size(14),
            button(
                text(if app.password_visible {
                    "Hide"
                } else {
                    "Show"
                })
                .size(13),
            )
            .on_press(Message::SettingsShowPassword)
            .style(button::secondary),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
        button(text(save_label).size(14))
            .on_press(Message::SettingsToggleSavePassword(!app.config.save_password))
            .style(button::text),
        text(help::SAVE_PASSWORD).size(12).color([0.5, 0.5, 0.5]),
        rule::horizontal(1),
        text(help::CHANGE_PASSWORD_HELP)
            .size(12)
            .color([0.5, 0.5, 0.5]),
        text_input("New password...", &app.settings_new_password)
            .on_input(Message::SettingsPasswordChanged)
            .secure(true),
        button(text("Change Password").size(14))
            .on_press(Message::SettingsChangePassword)
            .style(button::secondary),
    ]
    .spacing(8);

    content = content.push(card(
        "Password",
        Some("Manage your repository password"),
        password_section,
    ));

    // Theme section
    let theme_options: Vec<String> = THEME_NAMES.iter().map(|s| s.to_string()).collect();

    content = content.push(card(
        "Theme",
        Some("Choose your preferred color scheme"),
        pick_list(theme_options, Some(app.config.theme.clone()), Message::SettingsThemeSelected)
            .width(200),
    ));

    // Repository info section
    let repo_path = app.repo_path();
    let mut repo_info_col = column![
        text(format!("Path: {}", repo_path.display())).size(13),
    ]
    .spacing(4);

    if let Some(ref info) = app.repo_info {
        repo_info_col = repo_info_col
            .push(text(format!("Snapshots: {}", info.snapshot_count)).size(13));
    }

    content = content.push(card(
        "Repository",
        Some("Information about the backup repository"),
        repo_info_col,
    ));

    // About section
    content = content.push(card(
        "About",
        None,
        column![
            text("Rustic Vault v0.1.0").size(14),
            text("A GUI backup tool powered by rustic_core (restic-compatible).")
                .size(13)
                .color([0.6, 0.6, 0.6]),
            text("Built with Iced and Rust.")
                .size(13)
                .color([0.6, 0.6, 0.6]),
        ]
        .spacing(4),
    ));

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
