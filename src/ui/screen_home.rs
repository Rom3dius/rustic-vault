use iced::widget::{button, column, container, row, rule, scrollable, space, text, tooltip};
use iced::{Element, Length};

use crate::app::{App, Message};
use crate::ui::help;
use crate::ui::widgets::{card, sidebar_entry, tooltip_button};

/// First-run setup screen.
pub fn view_first_run(app: &App) -> Element<'_, Message> {
    let mut content = column![
        text("Welcome to Rustic Vault").size(28),
        text(help::WELCOME).size(14).color([0.7, 0.7, 0.7]),
        rule::horizontal(1),
        text("Repository Password").size(20),
        text(help::REPO_PASSWORD).size(13).color([0.6, 0.6, 0.6]),
    ]
    .spacing(12)
    .padding(24)
    .max_width(600);

    if app.first_run_custom_password {
        content = content.push(
            iced::widget::text_input("Enter your password...", &app.first_run_password)
                .on_input(Message::FirstRunPasswordChanged)
                .secure(true)
                .width(Length::Fill),
        );
        content = content.push(
            button(text("Use generated password instead").size(13))
                .on_press(Message::FirstRunToggleCustomPassword(false))
                .style(button::text),
        );
    } else {
        content = content.push(
            column![
                text("Generated password:").size(13),
                container(
                    text(&app.first_run_password)
                        .size(16)
                        .font(iced::Font::MONOSPACE)
                )
                .padding(8)
                .style(container::rounded_box),
                row![
                    button(text("Regenerate").size(13))
                        .on_press(Message::FirstRunGeneratePassword)
                        .style(button::secondary),
                    button(text("Use custom password").size(13))
                        .on_press(Message::FirstRunToggleCustomPassword(true))
                        .style(button::text),
                ]
                .spacing(8),
            ]
            .spacing(8),
        );
    }

    // Save password checkbox
    let save_label = if app.first_run_save_password {
        "[x] Save password to disk"
    } else {
        "[ ] Save password to disk"
    };
    content = content.push(
        column![
            button(text(save_label).size(14))
                .on_press(Message::FirstRunToggleSavePassword(!app.first_run_save_password))
                .style(button::text),
            text(help::SAVE_PASSWORD).size(12).color([0.5, 0.5, 0.5]),
        ]
        .spacing(4),
    );

    // Init button
    let can_init = !app.first_run_password.is_empty() && !app.busy;
    let mut init_btn = button(text("Initialize Repository").size(16)).style(button::primary);
    if can_init {
        init_btn = init_btn.on_press(Message::FirstRunInit);
    }
    content = content.push(init_btn);

    container(scrollable(content))
        .center_x(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Main home screen with sidebar + detail view.
pub fn view(app: &App) -> Element<'_, Message> {
    // Sidebar: profile list
    let mut sidebar = column![
        text("Profiles").size(18),
        tooltip(
            button(text("+ New Profile").size(14))
                .on_press(Message::GoProfileEditor(None))
                .width(Length::Fill),
            text(help::NEW_PROFILE_TOOLTIP).size(12),
            tooltip::Position::Right,
        ),
    ]
    .spacing(8)
    .padding(12)
    .width(220);

    for (i, profile) in app.profiles.iter().enumerate() {
        let last_backup = profile
            .last_backup
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string());
        let selected = app.selected_profile == Some(i);
        sidebar = sidebar.push(sidebar_entry(
            &profile.name,
            last_backup.as_deref(),
            selected,
            Message::SelectProfile(i),
        ));
    }

    let sidebar_scroll = scrollable(sidebar).height(Length::Fill);

    // Main area: selected profile details or placeholder
    let main_area = if let Some(idx) = app.selected_profile {
        if let Some(profile) = app.profiles.get(idx) {
            view_profile_detail(profile, idx)
        } else {
            placeholder()
        }
    } else {
        placeholder()
    };

    // Nav bar at top
    let nav = row![
        text("Rustic Vault").size(22),
        space::horizontal(),
        button(text("Snapshots").size(14)).on_press(Message::GoSnapshots),
        button(text("Settings").size(14)).on_press(Message::GoSettings),
    ]
    .spacing(8)
    .padding(12)
    .align_y(iced::Alignment::Center);

    let body = row![
        container(sidebar_scroll)
            .width(220)
            .height(Length::Fill)
            .style(container::rounded_box),
        container(scrollable(main_area))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16),
    ]
    .spacing(8);

    column![nav, rule::horizontal(1), body]
        .spacing(0)
        .height(Length::Fill)
        .into()
}

fn view_profile_detail(profile: &crate::core::profile::Profile, idx: usize) -> Element<'_, Message> {
    let paths_list: Element<Message> = if profile.paths.is_empty() {
        text("No folders configured").size(13).color([0.5, 0.5, 0.5]).into()
    } else {
        column(
            profile
                .paths
                .iter()
                .map(|p| text(p.display().to_string()).size(13).into())
                .collect::<Vec<Element<Message>>>(),
        )
        .spacing(2)
        .into()
    };

    let excludes_text = if profile.excludes.is_empty() {
        "None".to_string()
    } else {
        profile.excludes.join(", ")
    };

    let tags_text = if profile.tags.is_empty() {
        "None".to_string()
    } else {
        profile.tags.join(", ")
    };

    let retention_summary = profile.retention.summary();

    column![
        text(&profile.name).size(24),
        rule::horizontal(1),
        card(
            "Source Folders",
            Some("Folders included in backups"),
            paths_list,
        ),
        card(
            "Exclude Patterns",
            None,
            text(excludes_text).size(13),
        ),
        card("Tags", None, text(tags_text).size(13)),
        card(
            "Retention Policy",
            None,
            text(retention_summary).size(13),
        ),
        rule::horizontal(1),
        row![
            tooltip_button("Run Backup", help::RUN_BACKUP_TOOLTIP, Message::GoBackup(idx)),
            tooltip_button(
                "View Snapshots",
                help::VIEW_SNAPSHOTS_TOOLTIP,
                Message::GoSnapshots,
            ),
            button(text("Edit Profile").size(14))
                .on_press(Message::GoProfileEditor(Some(idx))),
            button(text("Delete").size(14).color([1.0, 0.3, 0.3]))
                .on_press(Message::DeleteProfile(idx))
                .style(button::secondary),
        ]
        .spacing(8),
    ]
    .spacing(12)
    .into()
}

fn placeholder<'a>() -> Element<'a, Message> {
    container(
        column![
            text("Select a profile").size(18).color([0.5, 0.5, 0.5]),
            text("Choose a profile from the sidebar, or create a new one.")
                .size(14)
                .color([0.4, 0.4, 0.4]),
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center),
    )
    .center(Length::Fill)
    .into()
}
