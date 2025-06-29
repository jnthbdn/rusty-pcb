use iced::{Element, Subscription};

use crate::AppDaemon;

pub trait BaseWindow<Message, Action> {
    fn new() -> Self;
    fn title(&self) -> String;
    fn subscription(&self) -> Subscription<Message>;
    fn update(&mut self, message: Message) -> Action;
    fn view(&self, parent: &AppDaemon) -> Element<'_, Message>;
}
