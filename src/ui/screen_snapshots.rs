use iced::widget::{button, column, container, row, rule, scrollable, space, text};
use iced::{Element, Length};

use crate::app::{App, Message};
use crate::ui::help;
use crate::ui::widgets::{card, tooltip_button};

pub fn view(app: &App) -> Element<'_, Message> {
    let mut content = column![
        row![
            text("Snapshots").size(24),
            space::horizontal(),
            button(text("Home").size(14)).on_press(Message::GoHome),
            button(text("Refresh").size(14)).on_press(Message::RefreshSnapshots),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
        text(help::SNAPSHOT).size(13).color([0.6, 0.6, 0.6]),
        rule::horizontal(1),
    ]
    .spacing(12)
    .padding(24);

    if app.snapshots_loading {
        content = content.push(text("Loading snapshots...").size(14));
    } else if app.snapshots.is_empty() {
        content = content.push(
            container(
                text("No snapshots yet. Run a backup to create your first snapshot.")
                    .size(14)
                    .color([0.5, 0.5, 0.5]),
            )
            .padding(20),
        );
    } else {
        for snap in &app.snapshots {
            let tags_text = if snap.tags.is_empty() {
                String::new()
            } else {
                format!("Tags: {}", snap.tags.join(", "))
            };

            let paths_text = snap.paths.join(", ");
            let size_text = snap
                .summary_size
                .map(|s| bytesize::ByteSize(s).to_string())
                .unwrap_or_else(|| "—".to_string());

            let snap_id = snap.id.clone();
            let snap_id2 = snap.id.clone();

            let snap_card = card(
                &format!("{} — {}", snap.short_id, snap.time),
                Some(&snap.hostname),
                column![
                    text(format!("Paths: {paths_text}")).size(12),
                    text(tags_text).size(12),
                    text(format!("Size: {size_text}")).size(12),
                    row![
                        tooltip_button(
                            "Restore",
                            help::RESTORE,
                            Message::SnapshotRestore(snap_id),
                        ),
                        tooltip_button(
                            "Delete",
                            help::DELETE_SNAPSHOT_TOOLTIP,
                            Message::SnapshotDelete(snap_id2),
                        ),
                    ]
                    .spacing(8),
                ]
                .spacing(4),
            );

            content = content.push(snap_card);
        }

        // Forget & Prune button
        content = content.push(rule::horizontal(1));
        content = content.push(tooltip_button(
            "Apply Retention & Prune",
            help::RETENTION_PRUNE_TOOLTIP,
            Message::SnapshotForgetPrune,
        ));
        content = content.push(
            text(help::PRUNE)
                .size(12)
                .color([0.5, 0.5, 0.5]),
        );
    }

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
