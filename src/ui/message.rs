use iced::Point;

use crate::{
    layer::layer::Layer,
    ui::{tab_bar::TabBarId, tabs::files::TabFileMessage},
    AppTheme,
};

#[derive(Debug, Clone)]
pub enum CanvasLayer {
    Top,
    Bottom,
    Drill,
    Outline,
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowLoading,
    HideLoading,
    ReadLogReceiver,
    ChangeTheme(AppTheme),

    GerberCanvas(GerberCanvasMessage),
    TabBar(TabBarMessage),
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
