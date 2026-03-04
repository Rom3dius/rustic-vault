use iced::widget::{button, column, container, row, rule, scrollable, space, text, text_input};
use iced::{Element, Length};

use crate::app::{App, Message};
use crate::ui::widgets::card;

/// Repo selector / manager screen.
pub fn view_selector(app: &App) -> Element<'_, Message> {
    let mut content = column![
        text("Rustic Vault").size(28),
        text("Select or add a repository").size(14).color([0.6, 0.6, 0.6]),
        rule::horizontal(1),
    ]
    .spacing(12)
    .padding(24)
    .max_width(700);

    // List existing repos
    if app.config.repos.is_empty() {
        content = content.push(
            text("No repositories configured yet. Add one below.")
                .size(14)
                .color([0.5, 0.5, 0.5]),
        );
    } else {
        for repo in &app.config.repos {
            let is_current = app.config.current_repo.as_deref() == Some(&repo.id);
            let label = if is_current {
                format!("{} (current)", repo.name)
            } else {
                repo.name.clone()
            };

            let repo_id_select = repo.id.clone();
            let repo_id_remove = repo.id.clone();

            let repo_card = card(
                &label,
                Some(&repo.repo_path.display().to_string()),
                row![
                    button(text("Select").size(13))
                        .on_press(Message::RepoSelected(repo_id_select))
                        .style(if is_current {
                            button::primary
                        } else {
                            button::secondary
                        }),
                    space::horizontal(),
                    button(text("Remove").size(13).color([1.0, 0.3, 0.3]))
                        .on_press(Message::RepoRemove(repo_id_remove))
                        .style(button::text),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            );

            content = content.push(repo_card);
        }
    }

    // Add repo form
    content = content.push(rule::horizontal(1));
    content = content.push(text("Add Repository").size(20));

    let add_form = column![
        text_input("Repository name...", &app.repo_add_name)
            .on_input(Message::RepoAddNameChanged)
            .width(Length::Fill),
        row![
            text_input("Repository path...", &app.repo_add_path)
                .on_input(Message::RepoAddPathChanged)
                .width(Length::Fill),
            button(text("Browse").size(13))
                .on_press(Message::RepoAddBrowse)
                .style(button::secondary),
        ]
        .spacing(8),
        {
            let can_add =
                !app.repo_add_name.trim().is_empty() && !app.repo_add_path.trim().is_empty();
            let mut btn =
                button(text("Add Repository").size(14)).style(button::primary);
            if can_add {
                btn = btn.on_press(Message::RepoAddConfirm);
            }
            btn
        },
    ]
    .spacing(8);

    content = content.push(add_form);

    container(scrollable(content))
        .center_x(Length::Fill)
        .height(Length::Fill)
        .into()
}
