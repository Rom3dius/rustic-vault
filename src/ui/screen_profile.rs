use iced::widget::{button, column, container, row, rule, scrollable, space, text, text_input};
use iced::{Element, Length};

use crate::app::{App, Message};
use crate::ui::help;
use crate::ui::widgets::{card, form_field};

pub fn view(app: &App) -> Element<'_, Message> {
    let profile = match &app.editor_profile {
        Some(p) => p,
        None => {
            return container(text("No profile loaded"))
                .center(Length::Fill)
                .into();
        }
    };

    let mut content = column![
        row![
            text("Profile Editor").size(24),
            space::horizontal(),
            button(text("Cancel").size(14)).on_press(Message::ProfileCancel),
            button(text("Save").size(14))
                .on_press(Message::ProfileSave)
                .style(button::primary),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
        rule::horizontal(1),
    ]
    .spacing(12)
    .padding(24);

    // Name
    content = content.push(form_field(
        "Profile Name",
        help::PROFILE,
        text_input("Profile name...", &profile.name).on_input(Message::ProfileNameChanged),
    ));

    // Source folders
    let mut folders_col = column![].spacing(4);
    for (i, path) in profile.paths.iter().enumerate() {
        folders_col = folders_col.push(
            row![
                text(path.display().to_string()).size(13).width(Length::Fill),
                button(text("x").size(12))
                    .on_press(Message::ProfileRemoveFolder(i))
                    .style(button::secondary),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center),
        );
    }
    folders_col = folders_col.push(
        button(text("+ Add Folder").size(13))
            .on_press(Message::ProfileAddFolder)
            .style(button::secondary),
    );

    content = content.push(card(
        "Source Folders",
        Some("Folders to include in backups"),
        folders_col,
    ));

    // Exclude patterns
    content = content.push(form_field(
        "Exclude Patterns",
        help::EXCLUDE_PATTERNS,
        text_input("node_modules, .cache, *.tmp", &app.editor_excludes_text)
            .on_input(Message::ProfileExcludesChanged),
    ));

    // Tags
    content = content.push(form_field(
        "Tags",
        help::TAGS,
        text_input("laptop, documents", &app.editor_tags_text).on_input(Message::ProfileTagsChanged),
    ));

    // Retention policy
    let retention = &profile.retention;
    let summary = retention.summary();

    let retention_section = column![
        text(help::RETENTION_POLICY)
            .size(12)
            .color([0.5, 0.5, 0.5]),
        form_field(
            "Keep Last",
            help::KEEP_LAST,
            text_input("5", &app.editor_keep_last).on_input(Message::ProfileKeepLastChanged),
        ),
        form_field(
            "Keep Daily",
            help::KEEP_DAILY,
            text_input("7", &app.editor_keep_daily).on_input(Message::ProfileKeepDailyChanged),
        ),
        form_field(
            "Keep Weekly",
            help::KEEP_WEEKLY,
            text_input("4", &app.editor_keep_weekly).on_input(Message::ProfileKeepWeeklyChanged),
        ),
        form_field(
            "Keep Monthly",
            help::KEEP_MONTHLY,
            text_input("12", &app.editor_keep_monthly)
                .on_input(Message::ProfileKeepMonthlyChanged),
        ),
        text(summary).size(13).color([0.4, 0.8, 0.4]),
    ]
    .spacing(8);

    content = content.push(card(
        "Retention Policy",
        Some("How long to keep old snapshots"),
        retention_section,
    ));

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
