use iced::{widget::text, Element};
use iced_aw::TabLabel;

use crate::ui::message::MainWindowMessage;

#[derive(Debug, Default)]
pub struct Drilling {}

impl Drilling {
    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text("Drilling".to_string())
    }

    pub fn view(&self) -> Element<MainWindowMessage> {
        text("Drilling panel").into()
    }
}
