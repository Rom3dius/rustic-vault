use iced::widget::{button, column, container, text, tooltip, Column};
use iced::{Element, Length, Theme};

/// Create a section card with a title, optional subtitle, and content.
pub fn card<'a, M: 'a + Clone>(
    title: &str,
    subtitle: Option<&str>,
    content: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    let mut col = Column::new()
        .spacing(8)
        .push(text(title.to_string()).size(18));

    if let Some(sub) = subtitle {
        col = col.push(text(sub.to_string()).size(13).color([0.6, 0.6, 0.6]));
    }

    col = col.push(content);

    container(col)
        .padding(16)
        .width(Length::Fill)
        .style(container::rounded_box)
        .into()
}

/// Create a labeled form field with inline help text below.
pub fn form_field<'a, M: 'a + Clone>(
    label: &str,
    help: &str,
    input: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    column![
        text(label.to_string()).size(14),
        input.into(),
        text(help.to_string()).size(12).color([0.5, 0.5, 0.5]),
    ]
    .spacing(4)
    .into()
}

/// Create a button with a tooltip.
pub fn tooltip_button<'a, M: 'a + Clone>(
    label: &str,
    tip: &str,
    on_press: M,
) -> Element<'a, M> {
    tooltip(
        button(text(label.to_string()).size(14)).on_press(on_press),
        text(tip.to_string()).size(12),
        tooltip::Position::Bottom,
    )
    .gap(4)
    .into()
}

/// Create a danger-styled button (for destructive actions).
#[allow(dead_code)]
pub fn danger_button<'a, M: 'a + Clone>(label: &str, on_press: M) -> Element<'a, M> {
    button(text(label.to_string()).size(14).color([1.0, 0.3, 0.3]))
        .on_press(on_press)
        .style(button::secondary)
        .into()
}

/// A status badge showing a label with a colored background.
#[allow(dead_code)]
pub fn status_badge<'a, M: 'a>(label: &str) -> Element<'a, M> {
    container(text(label.to_string()).size(12))
        .padding([2, 8])
        .style(container::rounded_box)
        .into()
}

/// A sidebar entry (profile name + optional subtitle).
pub fn sidebar_entry<'a, M: 'a + Clone>(
    name: &str,
    subtitle: Option<&str>,
    selected: bool,
    on_click: M,
) -> Element<'a, M> {
    let mut col = Column::new().push(text(name.to_string()).size(14));
    if let Some(sub) = subtitle {
        col = col.push(text(sub.to_string()).size(11).color([0.5, 0.5, 0.5]));
    }

    let btn = button(col.spacing(2).width(Length::Fill))
        .on_press(on_click)
        .width(Length::Fill)
        .style(if selected {
            button::primary
        } else {
            button::secondary
        });

    btn.into()
}

/// Placeholder function to map a theme name to an iced Theme.
pub fn theme_from_name(name: &str) -> Theme {
    match name {
        "Light" => Theme::Light,
        "Dark" => Theme::Dark,
        "Dracula" => Theme::Dracula,
        "Nord" => Theme::Nord,
        "SolarizedLight" => Theme::SolarizedLight,
        "SolarizedDark" => Theme::SolarizedDark,
        "GruvboxLight" => Theme::GruvboxLight,
        "GruvboxDark" => Theme::GruvboxDark,
        "CatppuccinLatte" => Theme::CatppuccinLatte,
        "CatppuccinFrappe" => Theme::CatppuccinFrappe,
        "CatppuccinMacchiato" => Theme::CatppuccinMacchiato,
        "CatppuccinMocha" => Theme::CatppuccinMocha,
        "TokyoNight" => Theme::TokyoNight,
        "TokyoNightStorm" => Theme::TokyoNightStorm,
        "TokyoNightLight" => Theme::TokyoNightLight,
        "KanagawaWave" => Theme::KanagawaWave,
        "KanagawaDragon" => Theme::KanagawaDragon,
        "KanagawaLotus" => Theme::KanagawaLotus,
        "Moonfly" => Theme::Moonfly,
        "Nightfly" => Theme::Nightfly,
        "Oxocarbon" => Theme::Oxocarbon,
        "Ferra" => Theme::Ferra,
        _ => Theme::Dark,
    }
}

/// List of available theme names.
pub const THEME_NAMES: &[&str] = &[
    "Light",
    "Dark",
    "Dracula",
    "Nord",
    "SolarizedLight",
    "SolarizedDark",
    "GruvboxLight",
    "GruvboxDark",
    "CatppuccinLatte",
    "CatppuccinFrappe",
    "CatppuccinMacchiato",
    "CatppuccinMocha",
    "TokyoNight",
    "TokyoNightStorm",
    "TokyoNightLight",
    "KanagawaWave",
    "KanagawaDragon",
    "KanagawaLotus",
    "Moonfly",
    "Nightfly",
    "Oxocarbon",
    "Ferra",
];
