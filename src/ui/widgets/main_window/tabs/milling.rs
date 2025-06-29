use iced::{widget::text, Element};
use iced_aw::TabLabel;

use crate::ui::message::MainWindowMessage;

#[derive(Debug, Default)]
pub struct Milling {}

impl Milling {
    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text("Milling".to_string())
    }

    pub fn view(&self) -> Element<MainWindowMessage> {
        text("Milling panel").into()
    }
}
