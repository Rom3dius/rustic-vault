use iced::widget::{button, column, container, progress_bar, rule, text};
use iced::{Element, Length};

use crate::app::{App, Message};

pub fn view(app: &App) -> Element<'_, Message> {
    let mut content = column![].spacing(12).padding(24);

    if app.backup_running {
        let profile_name = app
            .selected_profile
            .and_then(|i| app.profiles.get(i))
            .map(|p| p.name.as_str())
            .unwrap_or("Unknown");

        content = content
            .push(text(format!("Creating snapshot for '{profile_name}'...")).size(22))
            .push(text("Backup in progress. This may take a while depending on the amount of data.").size(14).color([0.6, 0.6, 0.6]))
            .push(progress_bar(0.0..=1.0, 0.5))
            .push(text("Processing files...").size(13).color([0.5, 0.5, 0.5]));
    } else if let Some(ref summary) = app.backup_summary {
        content = content
            .push(text("Backup Complete!").size(22))
            .push(rule::horizontal(1))
            .push(
                column![
                    text(format!("Snapshot ID: {}", &summary.snapshot_id[..8.min(summary.snapshot_id.len())])).size(14),
                    text(format!("Files new: {}", summary.files_new)).size(14),
                    text(format!("Files changed: {}", summary.files_changed)).size(14),
                    text(format!("Files unmodified: {}", summary.files_unmodified)).size(14),
                    text(format!(
                        "Data added: {}",
                        bytesize::ByteSize(summary.data_added)
                    ))
                    .size(14),
                    text(format!(
                        "Total processed: {}",
                        bytesize::ByteSize(summary.total_bytes_processed)
                    ))
                    .size(14),
                ]
                .spacing(4),
            )
            .push(rule::horizontal(1))
            .push(
                iced::widget::row![
                    button(text("View Snapshots").size(14)).on_press(Message::GoSnapshots),
                    button(text("Back to Home").size(14)).on_press(Message::GoHome),
                ]
                .spacing(8),
            );
    } else {
        content = content
            .push(text("No backup in progress").size(18))
            .push(button(text("Back").size(14)).on_press(Message::GoHome));
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
