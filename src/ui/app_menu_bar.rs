use iced::{
    widget::{self, button, center, container, horizontal_rule, image::Handle, row, text, Image},
    Background, Color, Element, Length, Padding,
};
use iced_aw::{
    menu::{self, Item},
    menu_bar, menu_items, Menu,
};

use crate::{ui::message::GerberCanvasMessage, AppTheme};

use super::message::Message;

const IMG_ZOOM_IN: &[u8] = include_bytes!("../../img/zoom-in.png");
const IMG_ZOOM_OUT: &[u8] = include_bytes!("../../img/zoom-out.png");
const IMG_ZOOM_FIT: &[u8] = include_bytes!("../../img/zoom-fit.png");

#[derive(Debug, Default)]
pub struct AppMenuBar {}

impl AppMenuBar {
    pub fn view<'a>(&self, select_theme: &AppTheme) -> Element<'a, Message> {
        let menu_template = |items| Menu::new(items).max_width(200.0).offset(10.0).spacing(5);

        #[rustfmt::skip]
        let menu_bar = menu_bar!(
            (Self::menu_button("File"),
            menu_template(menu_items!(
                (Self::item_button("Save project"))
                (Self::item_button("Load project"))
            )))
            (Self::menu_button("View"),
            menu_template(menu_items!(
                (row!(
                        Self::item_button_image(Image::new(Handle::from_bytes(IMG_ZOOM_OUT))).on_press(Message::GerberCanvas(GerberCanvasMessage::ZoomOut)),
                        Self::item_button_image(Image::new(Handle::from_bytes(IMG_ZOOM_FIT))).on_press(Message::GerberCanvas(GerberCanvasMessage::ResetView)),
                        Self::item_button_image(Image::new(Handle::from_bytes(IMG_ZOOM_IN))).on_press(Message::GerberCanvas(GerberCanvasMessage::ZoomIn))
                    ).height(30).spacing(4)
                )
                (Self::item_sub("Theme"), menu_template(menu_items!(
                    (Self::item_button_radio("Dark", select_theme, AppTheme::Dark).on_press(Message::ChangeTheme(AppTheme::Dark)))
                    (Self::item_button_radio("Light", select_theme, AppTheme::Light).on_press(Message::ChangeTheme(AppTheme::Light)))
                    (horizontal_rule(4))
                    (Self::item_button_radio("Dracula", select_theme, AppTheme::Dracula).on_press(Message::ChangeTheme(AppTheme::Dracula)))
                    (Self::item_button_radio("Nord", select_theme, AppTheme::Nord).on_press(Message::ChangeTheme(AppTheme::Nord)))
                    (Self::item_button_radio("SolarizedLight", select_theme, AppTheme::SolarizedLight).on_press(Message::ChangeTheme(AppTheme::SolarizedLight)))
                    (Self::item_button_radio("SolarizedDark", select_theme, AppTheme::SolarizedDark).on_press(Message::ChangeTheme(AppTheme::SolarizedDark)))
                    (Self::item_button_radio("GruvboxLight", select_theme, AppTheme::GruvboxLight).on_press(Message::ChangeTheme(AppTheme::GruvboxLight)))
                    (Self::item_button_radio("GruvboxDark", select_theme, AppTheme::GruvboxDark).on_press(Message::ChangeTheme(AppTheme::GruvboxDark)))
                    (Self::item_button_radio("CatppuccinLatte", select_theme, AppTheme::CatppuccinLatte).on_press(Message::ChangeTheme(AppTheme::CatppuccinLatte)))
                    (Self::item_button_radio("CatppuccinFrappe", select_theme, AppTheme::CatppuccinFrappe).on_press(Message::ChangeTheme(AppTheme::CatppuccinFrappe)))
                    (Self::item_button_radio("CatppuccinMacchiato", select_theme, AppTheme::CatppuccinMacchiato).on_press(Message::ChangeTheme(AppTheme::CatppuccinMacchiato)))
                    (Self::item_button_radio("CatppuccinMocha", select_theme, AppTheme::CatppuccinMocha).on_press(Message::ChangeTheme(AppTheme::CatppuccinMocha)))
                    (Self::item_button_radio("TokyoNight", select_theme, AppTheme::TokyoNight).on_press(Message::ChangeTheme(AppTheme::TokyoNight)))
                    (Self::item_button_radio("TokyoNightStorm", select_theme, AppTheme::TokyoNightStorm).on_press(Message::ChangeTheme(AppTheme::TokyoNightStorm)))
                    (Self::item_button_radio("TokyoNightLight", select_theme, AppTheme::TokyoNightLight).on_press(Message::ChangeTheme(AppTheme::TokyoNightLight)))
                    (Self::item_button_radio("KanagawaWave", select_theme, AppTheme::KanagawaWave).on_press(Message::ChangeTheme(AppTheme::KanagawaWave)))
                    (Self::item_button_radio("KanagawaDragon", select_theme, AppTheme::KanagawaDragon).on_press(Message::ChangeTheme(AppTheme::KanagawaDragon)))
                    (Self::item_button_radio("KanagawaLotus", select_theme, AppTheme::KanagawaLotus).on_press(Message::ChangeTheme(AppTheme::KanagawaLotus)))
                    (Self::item_button_radio("Moonfly", select_theme, AppTheme::Moonfly).on_press(Message::ChangeTheme(AppTheme::Moonfly)))
                    (Self::item_button_radio("Nightfly", select_theme, AppTheme::Nightfly).on_press(Message::ChangeTheme(AppTheme::Nightfly)))
                    (Self::item_button_radio("Oxocarbon", select_theme, AppTheme::Oxocarbon).on_press(Message::ChangeTheme(AppTheme::Oxocarbon)))
                    (Self::item_button_radio("Ferra", select_theme, AppTheme::Ferra).on_press(Message::ChangeTheme(AppTheme::Ferra)))
                )))
            )))
            (Self::menu_button("Tools"),
            menu_template(menu_items!(
                (Self::item_button("Open Database"))
                (horizontal_rule(4))
                (Self::item_button("Import Database"))
                (Self::item_button("Export Database"))
            )))
            // (Self::menu_button("Demo"), menu_template(menu_items!(
            //     (Self::item_button("item_1"))
            //     (Self::item_button("item_2"))
            //     (Self::item_button("Sub Menu 1"), menu_template(menu_items!(
            //         (Self::item_button("item_1"))
            //         (Self::item_button("Sub Menu 2"), menu_template(menu_items!(
            //             (Self::item_button("item_1"))
            //             (Self::item_button("item_2"))
            //             (Self::item_button("item_3")))))
            //         (Self::item_button("item_2"))
            //         (Self::item_button("item_3")))))
            //     (Self::item_button("item_3"))
            // )))
        )
        .spacing(3.0)
        .padding(Padding::default().left(5))
        .width(Length::Fill)
        .style(|theme, status| menu::Style {
            bar_background: theme.extended_palette().background.weak.color.into(),
            ..menu::primary(theme, status)
        });

        menu_bar.into()
    }

    fn item_button_radio<'a, V>(label: &str, value: &V, selected: V) -> widget::Button<'a, Message>
    where
        V: Eq + Copy,
    {
        button(
            text(format!(
                "{}{}",
                if *value == selected { "✓ " } else { "  " },
                label
            ))
            .shaping(text::Shaping::Advanced),
        )
        .width(Length::Fill)
        .padding([4, 8])
    }

    fn item_button<'a>(content: impl Into<Element<'a, Message>>) -> widget::Button<'a, Message> {
        button(content).width(Length::Fill).padding([4, 8])
    }

    fn item_sub<'a>(label: &'a str) -> widget::Container<'a, Message> {
        container(row![
            text(label).width(Length::Fill),
            text(">").width(Length::Shrink)
        ])
        .width(Length::Fill)
        .padding([4, 8])
        .style(|theme| {
            let button_style = button::primary(theme, button::Status::Active);
            container::Style {
                background: button_style.background,
                text_color: Some(button_style.text_color),
                ..Default::default()
            }
        })
    }

    fn item_button_image<'a>(img: widget::Image) -> widget::Button<'a, Message> {
        Self::item_button(center(img))
    }

    fn menu_button<'a>(content: impl Into<Element<'a, Message>>) -> widget::Button<'a, Message> {
        button(content).style(|theme, status| button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: theme.extended_palette().background.base.text,
            ..button::primary(theme, status)
        })
    }
}
