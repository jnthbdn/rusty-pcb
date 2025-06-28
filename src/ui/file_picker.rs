use std::path::PathBuf;

use iced::{
    widget::{button, container, row, text, text_input, tooltip},
    Length, Task,
};
use log::info;

static mut LAST_FOLDER: Option<PathBuf> = None;

#[derive(Debug, Clone)]
pub enum FilePickerMessage {
    BrowseFile,
    ClearFile,
    OnFileDialogClose(Option<PathBuf>),
}

pub enum FilePickerAction {
    Run(Task<FilePickerMessage>),
    FileSelected(PathBuf),
    ClearFile,
    None,
}

pub struct FilePicker {
    file: PathBuf,
    is_enable: bool,
    filter_name: String,
    filter_pattern: Vec<String>,
}

impl FilePicker {
    pub fn new(file: Option<PathBuf>, filter_name: String, filter_pattern: Vec<String>) -> Self {
        Self {
            file: if let Some(file) = file {
                file
            } else {
                PathBuf::default()
            },
            is_enable: true,
            filter_name,
            filter_pattern,
        }
    }

    pub fn update(&mut self, msg: FilePickerMessage) -> FilePickerAction {
        match msg {
            FilePickerMessage::BrowseFile => FilePickerAction::Run(Task::perform(
                Self::pick_file(self.filter_name.clone(), self.filter_pattern.clone()),
                move |x| FilePickerMessage::OnFileDialogClose(x),
            )),
            FilePickerMessage::ClearFile => {
                self.file = PathBuf::new();
                FilePickerAction::ClearFile
            }
            FilePickerMessage::OnFileDialogClose(path_buf) => match path_buf {
                Some(path_buf) => {
                    self.file = path_buf.clone();
                    Self::set_last_path(path_buf.clone());

                    FilePickerAction::FileSelected(path_buf)
                }
                None => FilePickerAction::None,
            },
        }
    }

    pub fn view(&self) -> iced::Element<FilePickerMessage> {
        let mut btn_browse =
            button(text(if self.is_enable { "📂" } else { "🚫" }).shaping(text::Shaping::Advanced))
                .width(Length::Shrink)
                .height(Length::Shrink);

        if self.is_enable {
            btn_browse = btn_browse.on_press(FilePickerMessage::BrowseFile);
        }

        row![
            text_input("", self.file_to_string()).width(Length::Fill),
            btn_browse,
            tooltip(
                button(text("🗑️").shaping(text::Shaping::Advanced))
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .on_press(FilePickerMessage::ClearFile),
                container("Remove the layer file")
                    .padding(5)
                    .style(container::rounded_box),
                tooltip::Position::Bottom,
            )
        ]
        .spacing(4)
        .into()
    }

    pub fn enable(&mut self, is_enable: bool) {
        self.is_enable = is_enable;
    }

    fn file_to_string(&self) -> &str {
        self.file.to_str().unwrap_or("")
    }

    async fn pick_file(filter_name: String, filter_pattern: Vec<String>) -> Option<PathBuf> {
        info!("Open file explorer");
        let mut dialog = rfd::FileDialog::new()
            .add_filter(filter_name, &filter_pattern)
            .add_filter("All file", &["*"]);

        if let Some(path) = Self::get_last_path() {
            dialog = dialog.set_directory(path);
        }

        dialog.pick_file()
    }

    fn set_last_path(path: PathBuf) {
        unsafe {
            if let Some(new_path) = path.parent().map(|x| x.to_path_buf()) {
                LAST_FOLDER = Some(new_path);
            }
        }
    }

    fn get_last_path() -> Option<PathBuf> {
        #[allow(static_mut_refs)]
        unsafe {
            LAST_FOLDER.clone()
        }
    }
}
