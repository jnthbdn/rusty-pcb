use std::marker::PhantomData;

use iced::widget::center;
use iced::window;
use iced::window::Settings;
use iced::Element;
use iced::{Subscription, Task, Theme};
use log::error;

use crate::base_window::BaseWindow;
use crate::ui::main_window::MainWindow;
use crate::ui::message::MainWindowAction;
use crate::ui::message::{AppMessage, MainWindowMessage};

mod app_logger;
mod base_window;
mod layer;
mod ui;

pub const VERSION_APP: &str = env!("BUILD_VERSION");

pub fn main() -> iced::Result {
    iced::daemon(AppDaemon::title, AppDaemon::update, AppDaemon::view)
        .theme(AppDaemon::theme)
        .subscription(AppDaemon::subscription)
        .run_with(AppDaemon::new)
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

#[derive(Debug)]
struct AppWindow<MSG, ACT, W: BaseWindow<MSG, ACT>> {
    id: window::Id,
    win: W,

    msg: PhantomData<MSG>,
    act: PhantomData<ACT>,
}

impl<M, A, W: BaseWindow<M, A>> AppWindow<M, A, W> {
    pub fn new(id: window::Id, win: W) -> Self {
        Self {
            id,
            win,
            msg: PhantomData,
            act: PhantomData,
        }
    }
}

#[derive(Debug)]
struct AppDaemon {
    main_win: AppWindow<MainWindowMessage, MainWindowAction, MainWindow>,
    db_tool_win: Option<AppWindow<MainWindowMessage, MainWindowAction, MainWindow>>,
    theme: AppTheme,
}

impl AppDaemon {
    fn new() -> (Self, Task<AppMessage>) {
        let (win_id, win_task) = window::open(Settings::default());
        let s: AppDaemon = Self {
            main_win: AppWindow::new(win_id, MainWindow::new()),
            db_tool_win: None,
            theme: Default::default(),
        };

        (s, win_task.discard())
    }

    fn title(&self, window_id: window::Id) -> String {
        if window_id == self.main_win.id {
            self.main_win.win.title()
        } else {
            "===ERROR UNKNOWN WINDOW===".to_string()
        }
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::MainWindow(main_window_message) => {
                match self.main_win.win.update(main_window_message) {
                    MainWindowAction::Run(task) => task.map(AppMessage::MainWindow),
                    MainWindowAction::None => Task::none(),
                    MainWindowAction::ChangeTheme(app_theme) => {
                        self.theme = app_theme;
                        Task::none()
                    }
                }
            }
            AppMessage::WindowClosed(id) => {
                if id == self.main_win.id {
                    iced::exit()
                } else {
                    if let Some(win) = self.db_tool_win.take() {
                        window::close(win.id)
                    } else {
                        Task::none()
                    }
                }
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<'_, AppMessage> {
        if window_id == self.main_win.id {
            self.main_win.win.view(self).map(AppMessage::MainWindow)
        } else {
            error!("Unknown windows id '{}'.", window_id);
            center("Bad view").into()
        }
    }

    fn theme(&self, _window_id: window::Id) -> Theme {
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

    fn subscription(&self) -> Subscription<AppMessage> {
        Subscription::batch([
            window::close_events().map(AppMessage::WindowClosed),
            self.main_win.win.subscription().map(AppMessage::MainWindow),
        ])
    }

    pub fn get_current_theme(&self) -> AppTheme {
        self.theme
    }
}
