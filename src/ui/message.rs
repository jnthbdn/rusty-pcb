use std::fmt::Debug;

use iced::{advanced::graphics::core::window, Point, Task};

use crate::{
    layer::layer::Layer,
    ui::widgets::main_window::{tab_bar::TabBarId, tabs::files::TabFileMessage},
    AppTheme,
};

#[derive(Debug, Clone)]
pub enum CanvasLayer {
    Top,
    Bottom,
    Drill,
    Outline,
}

#[derive(Debug)]
pub enum AppMessage {
    MainWindow(MainWindowMessage),
    WindowClosed(window::Id),
}

#[derive(Debug, Clone)]
pub enum MainWindowMessage {
    ShowLoading,
    HideLoading,
    ReadLogReceiver,
    ChangeTheme(AppTheme),

    OpenToolDB,

    GerberCanvas(GerberCanvasMessage),
    TabBar(TabBarMessage),
}

pub enum MainWindowAction {
    Run(Task<MainWindowMessage>),
    None,

    ChangeTheme(AppTheme),
}

impl Debug for MainWindowAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Run(_) => write!(f, "Run"),
            Self::None => write!(f, "None"),
            Self::ChangeTheme(arg0) => f.debug_tuple("ChangeTheme").field(arg0).finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GerberCanvasMessage {
    ResetView,
    Translate { delta_x: f32, delta_y: f32 },
    ZoomPointer(f32, Point),
    ZoomIn,
    ZoomOut,

    LoadLayer(CanvasLayer, Layer),

    ShowTopLayer(bool),
    ShowBotLayer(bool),
    ShowDrillLayer(bool),
    ShowOutlineLayer(bool),

    ClearTopLayer,
    ClearBottomLayer,
    ClearDrillLayer,
    ClearOutlineLayer,
}

#[derive(Debug, Clone)]
pub enum TabBarMessage {
    TabSelected(TabBarId),
    TabFileMessage(TabFileMessage),
}
