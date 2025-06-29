use std::sync::mpsc::{self, Receiver};

use iced::time::{self, Duration};
use iced::window::{self, Settings};
use iced::{
    widget::{center, column, container, mouse_area, opaque, row, stack, text},
    Color, Element, Length, Padding, Subscription,
};
use log::info;

use crate::app_logger::{AppLogger, LogType};
use crate::base_window::BaseWindow;
use crate::ui::message::{MainWindowAction, MainWindowMessage};
use crate::ui::widgets::main_window::app_menu_bar::AppMenuBar;
use crate::ui::widgets::main_window::gerber_canvas::GerberCanvas;
use crate::ui::widgets::main_window::log_console::LogConsole;
use crate::ui::widgets::main_window::tab_bar::TabBar;
use crate::{AppDaemon, VERSION_APP};

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub enum PcbSides {
    #[default]
    OneSide,
    TwoSide,
}

#[derive(Debug)]
pub struct MainWindow {
    tab_bar: TabBar,
    gerber_canvas: GerberCanvas,
    menu_bar: AppMenuBar,
    console: LogConsole,

    db_window_id: Option<window::Id>,

    show_loading: bool,
    log_receiver: Receiver<LogType>,
}

impl MainWindow {
    fn loading_screen(&self) -> Element<MainWindowMessage> {
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

impl BaseWindow<MainWindowMessage, MainWindowAction> for MainWindow {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<LogType>();

        AppLogger::init(tx, log::LevelFilter::Info).expect("Failed to initialize AppLogger");

        let result = Self {
            tab_bar: TabBar::default(),
            gerber_canvas: Default::default(),
            menu_bar: Default::default(),
            console: Default::default(),
            db_window_id: None,
            show_loading: Default::default(),
            log_receiver: rx,
        };

        info!("Application started !");
        info!("Version: {}", VERSION_APP);

        result
    }

    fn title(&self) -> String {
        "Rusty PCB".to_string()
    }

    fn subscription(&self) -> Subscription<MainWindowMessage> {
        time::every(Duration::from_millis(250)).map(|_| MainWindowMessage::ReadLogReceiver)
    }

    fn update(&mut self, message: MainWindowMessage) -> MainWindowAction {
        match message {
            MainWindowMessage::GerberCanvas(gerber_canvas_message) => {
                self.gerber_canvas.update(gerber_canvas_message);
                MainWindowAction::None
            }
            MainWindowMessage::TabBar(tab_bar_message) => {
                MainWindowAction::Run(self.tab_bar.update(tab_bar_message))
            }
            MainWindowMessage::ShowLoading => {
                self.show_loading = true;
                MainWindowAction::None
            }
            MainWindowMessage::HideLoading => {
                self.show_loading = false;
                MainWindowAction::None
            }
            MainWindowMessage::ReadLogReceiver => {
                for msg in self.log_receiver.try_iter() {
                    match msg {
                        LogType::Info(msg) => self.console.log_message(&msg),
                        LogType::Warning(msg) => self.console.log_warning(&msg),
                        LogType::Error(msg) => self.console.log_error(&msg),
                    };
                }
                MainWindowAction::None
            }
            MainWindowMessage::ChangeTheme(theme) => MainWindowAction::ChangeTheme(theme),
            MainWindowMessage::OpenToolDB => {
                if self.db_window_id.is_none() {
                    let (id, task) = window::open(Settings::default());
                    self.db_window_id = Some(id);
                    MainWindowAction::Run(task.discard())
                } else {
                    MainWindowAction::Run(window::gain_focus(self.db_window_id.clone().unwrap()))
                }
            }
        }
    }

    fn view(&self, parent: &AppDaemon) -> Element<'_, MainWindowMessage> {
        let content = column![
            column![
                self.menu_bar.view(&parent.get_current_theme()),
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
}
