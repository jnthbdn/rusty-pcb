use iced::Task;
use iced_aw::Tabs;

use crate::ui::{
    message::{Message, TabBarMessage},
    tabs::{drilling::Drilling, files::Files, milling::Milling},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabBarId {
    Files,
    Milling,
    Drilling,
}

pub struct TabBar {
    files: Files,
    milling: Milling,
    drilling: Drilling,

    active_tab: TabBarId,
}

impl TabBar {
    pub fn update(&mut self, message: TabBarMessage) -> Task<Message> {
        match message {
            TabBarMessage::TabSelected(tab_bar_id) => {
                self.active_tab = tab_bar_id;
                Task::none()
            }
            TabBarMessage::TabFileMessage(tab_file_message) => self.files.update(tab_file_message),
        }
    }

    pub fn view(&self) -> Tabs<Message, TabBarId> {
        Tabs::new(|id| Message::TabBar(TabBarMessage::TabSelected(id)))
            .push(TabBarId::Files, self.files.tab_label(), self.files.view())
            .push(
                TabBarId::Milling,
                self.milling.tab_label(),
                self.milling.view(),
            )
            .push(
                TabBarId::Drilling,
                self.drilling.tab_label(),
                self.drilling.view(),
            )
            .set_active_tab(&self.active_tab)
            .tab_bar_position(iced_aw::TabBarPosition::Top)
            .tab_label_padding(2)
            .tab_bar_height(iced::Length::Shrink)
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self {
            files: Files::default(),
            milling: Milling::default(),
            drilling: Drilling::default(),
            active_tab: TabBarId::Files,
        }
    }
}
