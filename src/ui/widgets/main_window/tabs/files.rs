use std::{fs::File, io::BufReader, path::PathBuf};

use gerber_parser::parser::parse_gerber;
use iced::{
    padding,
    widget::{column, horizontal_rule, radio, row, vertical_space},
    Color, Element, Task,
};
use iced_aw::TabLabel;
use log::{error, info, warn};

use crate::{
    layer::layer::Layer,
    ui::{
        main_window::PcbSides,
        message::{CanvasLayer, GerberCanvasMessage, MainWindowMessage, TabBarMessage},
        widgets::file_picker::{FilePicker, FilePickerAction, FilePickerMessage},
    },
};

#[derive(Debug, Clone)]
pub enum TabFileMessage {
    PcbTypeChange(PcbSides),
    FilePickerMessage(CanvasLayer, FilePickerMessage),
}

#[derive(Debug)]
pub struct Files {
    pcb_sides: PcbSides,

    top_file_picker: FilePicker,
    bot_file_picker: FilePicker,
    drill_file_picker: FilePicker,
    outline_file_picker: FilePicker,
}

impl Files {
    pub fn tab_label(&self) -> TabLabel {
        TabLabel::Text("Files".to_string())
    }

    pub fn update(&mut self, message: TabFileMessage) -> Task<MainWindowMessage> {
        match message {
            TabFileMessage::PcbTypeChange(pcb_sides) => {
                self.pcb_sides = pcb_sides;

                self.top_file_picker
                    .enable(self.pcb_sides == PcbSides::TwoSide);

                Task::none()
            }

            TabFileMessage::FilePickerMessage(canvas_layer, file_picker_message) => {
                let picker: &mut FilePicker = match canvas_layer {
                    CanvasLayer::Top => &mut self.top_file_picker,
                    CanvasLayer::Bottom => &mut self.bot_file_picker,
                    CanvasLayer::Drill => &mut self.drill_file_picker,
                    CanvasLayer::Outline => &mut self.outline_file_picker,
                };

                match picker.update(file_picker_message) {
                    FilePickerAction::Run(task) => task.map(move |x| {
                        MainWindowMessage::TabBar(TabBarMessage::TabFileMessage(
                            TabFileMessage::FilePickerMessage(canvas_layer.clone(), x),
                        ))
                    }),
                    FilePickerAction::None => Task::none(),
                    FilePickerAction::FileSelected(path_buf) => {
                        Task::done(MainWindowMessage::ShowLoading)
                            .chain(
                                Task::future(Self::load_file(path_buf, canvas_layer.clone())).then(
                                    move |x| match x {
                                        Some(new_layer) => {
                                            info!(
                                                "Load new file to {:?} layer",
                                                canvas_layer.clone()
                                            );
                                            Task::done(MainWindowMessage::GerberCanvas(
                                                GerberCanvasMessage::LoadLayer(
                                                    canvas_layer.clone(),
                                                    new_layer,
                                                ),
                                            ))
                                        }
                                        None => {
                                            error!("Failed to load file");
                                            Task::none()
                                        }
                                    },
                                ),
                            )
                            .chain(Task::done(MainWindowMessage::HideLoading))
                    }
                    FilePickerAction::ClearFile => {
                        warn!("Clear {:?} layer", canvas_layer);
                        match &canvas_layer {
                            CanvasLayer::Top => Task::done(MainWindowMessage::GerberCanvas(
                                GerberCanvasMessage::ClearTopLayer,
                            )),
                            CanvasLayer::Bottom => Task::done(MainWindowMessage::GerberCanvas(
                                GerberCanvasMessage::ClearBottomLayer,
                            )),
                            CanvasLayer::Drill => Task::done(MainWindowMessage::GerberCanvas(
                                GerberCanvasMessage::ClearDrillLayer,
                            )),
                            CanvasLayer::Outline => Task::done(MainWindowMessage::GerberCanvas(
                                GerberCanvasMessage::ClearOutlineLayer,
                            )),
                        }
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<MainWindowMessage> {
        column![
            "PCB type",
            row![
                radio("1-side", PcbSides::OneSide, Some(self.pcb_sides), |id| {
                    MainWindowMessage::TabBar(TabBarMessage::TabFileMessage(
                        TabFileMessage::PcbTypeChange(id),
                    ))
                }),
                radio("2-sides", PcbSides::TwoSide, Some(self.pcb_sides), |id| {
                    MainWindowMessage::TabBar(TabBarMessage::TabFileMessage(
                        TabFileMessage::PcbTypeChange(id),
                    ))
                })
            ]
            .spacing(10)
            .padding(padding::left(20)),
            horizontal_rule(3),
            "Top file",
            self.top_file_picker
                .view()
                .map(
                    move |x| MainWindowMessage::TabBar(TabBarMessage::TabFileMessage(
                        TabFileMessage::FilePickerMessage(CanvasLayer::Top, x)
                    ))
                ),
            vertical_space().height(5),
            "Bottom file",
            self.bot_file_picker
                .view()
                .map(
                    move |x| MainWindowMessage::TabBar(TabBarMessage::TabFileMessage(
                        TabFileMessage::FilePickerMessage(CanvasLayer::Bottom, x)
                    ))
                ),
            vertical_space().height(5),
            "Drill file",
            self.drill_file_picker
                .view()
                .map(
                    move |x| MainWindowMessage::TabBar(TabBarMessage::TabFileMessage(
                        TabFileMessage::FilePickerMessage(CanvasLayer::Drill, x)
                    ))
                ),
            vertical_space().height(5),
            "Outline file",
            self.outline_file_picker
                .view()
                .map(
                    move |x| MainWindowMessage::TabBar(TabBarMessage::TabFileMessage(
                        TabFileMessage::FilePickerMessage(CanvasLayer::Outline, x)
                    ))
                ),
        ]
        .spacing(5)
        .into()
    }

    async fn load_file(file_path: PathBuf, layer: CanvasLayer) -> Option<Layer> {
        info!("Parsing {}", file_path.to_str().unwrap_or("unknown"));
        let reader = BufReader::new(File::open(&file_path).ok()?);
        match layer {
            CanvasLayer::Top => Some(Layer::from_gerber(
                &parse_gerber(reader),
                Color::from_rgb(0.0, 0.0, 1.0),
            )),
            CanvasLayer::Bottom => Some(Layer::from_gerber(
                &parse_gerber(reader),
                Color::from_rgb(1.0, 0.0, 0.0),
            )),
            CanvasLayer::Outline => Some(Layer::from_gerber(
                &parse_gerber(reader),
                Color::from_rgb(0.0, 1.0, 0.0),
            )),
            CanvasLayer::Drill => None,
        }
    }

    fn vec_str(vec: Vec<&str>) -> Vec<String> {
        vec.iter().map(ToString::to_string).collect()
    }
}

impl Default for Files {
    fn default() -> Self {
        let mut result = Self {
            pcb_sides: Default::default(),
            top_file_picker: FilePicker::new(
                None,
                "Gerber File".to_string(),
                Self::vec_str(vec!["gtl", "GTL", "gbr", "GBR"]),
            ),
            bot_file_picker: FilePicker::new(
                None,
                "Gerber File".to_string(),
                Self::vec_str(vec!["gbl", "GBL", "gbr", "GBR"]),
            ),
            drill_file_picker: FilePicker::new(
                None,
                "Excellon file".to_string(),
                Self::vec_str(vec!["drl", "DRL", "txt", "TXT"]),
            ),
            outline_file_picker: FilePicker::new(
                None,
                "Gerber File".to_string(),
                Self::vec_str(vec!["gko", "GKO", "gbr", "GBR"]),
            ),
        };

        result.top_file_picker.enable(false);

        result
    }
}
