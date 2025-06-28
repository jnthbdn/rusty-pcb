use std::sync::mpsc::{self, Receiver};

use iced::time::{self, Duration};
use iced::{
    widget::{center, column, container, mouse_area, opaque, row, stack, text},
    Color, Element, Length, Padding, Subscription, Task, Theme,
};

use log::info;
use ui::{app_menu_bar::AppMenuBar, gerber_canvas::GerberCanvas, message::Message};

use crate::{
    app_logger::{AppLogger, LogType},
    ui::{log_console::LogConsole, tab_bar::TabBar},
};

mod app_logger;
mod layer;
mod ui;

pub const VERSION_APP: &str = env!("BUILD_VERSION");

pub fn main() -> iced::Result {
    iced::application("Rusty PCB", MainWindow::update, MainWindow::view)
        .subscription(MainWindow::subscription)
        .theme(MainWindow::theme)
        .run()
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
enum PcbSides {
    #[default]
    OneSide,
    TwoSide,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum AppTheme {
    #[default]
    Dark,
    Light,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

struct MainWindow {
    tab_bar: TabBar,
    gerber_canvas: GerberCanvas,
    menu_bar: AppMenuBar,
    console: LogConsole,

    show_loading: bool,
    log_receiver: Receiver<LogType>,
    theme: AppTheme,
}

impl MainWindow {
    fn theme(&self) -> Theme {
        match self.theme {
            AppTheme::Dark => Theme::Dark,
            AppTheme::Light => Theme::Light,
            AppTheme::Dracula => Theme::Dracula,
            AppTheme::Nord => Theme::Nord,
            AppTheme::SolarizedLight => Theme::SolarizedLight,
            AppTheme::SolarizedDark => Theme::SolarizedDark,
            AppTheme::GruvboxLight => Theme::GruvboxLight,
            AppTheme::GruvboxDark => Theme::GruvboxDark,
            AppTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            AppTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            AppTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            AppTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            AppTheme::TokyoNight => Theme::TokyoNight,
            AppTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            AppTheme::TokyoNightLight => Theme::TokyoNightLight,
            AppTheme::KanagawaWave => Theme::KanagawaWave,
            AppTheme::KanagawaDragon => Theme::KanagawaDragon,
            AppTheme::KanagawaLotus => Theme::KanagawaLotus,
            AppTheme::Moonfly => Theme::Moonfly,
            AppTheme::Nightfly => Theme::Nightfly,
            AppTheme::Oxocarbon => Theme::Oxocarbon,
            AppTheme::Ferra => Theme::Ferra,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(1000)).map(|_| Message::ReadLogReceiver)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GerberCanvas(gerber_canvas_message) => {
                self.gerber_canvas.update(gerber_canvas_message);
                Task::none()
            }
            Message::TabBar(tab_bar_message) => self.tab_bar.update(tab_bar_message),
            Message::ShowLoading => {
                self.show_loading = true;
                Task::none()
            }
            Message::HideLoading => {
                self.show_loading = false;
                Task::none()
            }
            Message::ReadLogReceiver => {
                for msg in self.log_receiver.try_iter() {
                    match msg {
                        LogType::Info(msg) => self.console.log_message(&msg),
                        LogType::Warning(msg) => self.console.log_warning(&msg),
                        LogType::Error(msg) => self.console.log_error(&msg),
                    };
                }
                Task::none()
            }
            Message::ChangeTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            column![
                self.menu_bar.view(&self.theme),
                row![
                    self.tab_bar.view().width(Length::FillPortion(6)),
                    self.gerber_canvas.view().width(Length::FillPortion(3))
                ]
                .padding(Padding::new(5.0).top(10))
                .spacing(5),
            ]
            .height(Length::FillPortion(7)),
            self.console
                .view()
                .height(Length::FillPortion(3))
                .width(Length::Fill)
        ]
        .padding(5);

        if self.show_loading {
            stack![content, self.loading_screen()].into()
        } else {
            content.into()
        }
    }

    fn loading_screen(&self) -> Element<Message> {
        opaque(mouse_area(
            center(opaque(text!("Loading... please wait"))).style(|_theme| container::Style {
                background: Some(
                    Color {
                        a: 0.8,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }),
        ))
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel::<LogType>();

        AppLogger::init(tx, log::LevelFilter::Info).expect("Failed to initialize AppLogger");

        let result = Self {
            tab_bar: TabBar::default(),
            gerber_canvas: Default::default(),
            menu_bar: Default::default(),
            console: Default::default(),
            show_loading: Default::default(),
            log_receiver: rx,
            theme: Default::default(),
        };

        info!("Application started !");
        info!("Version: {}", VERSION_APP);

        result
    }
}
