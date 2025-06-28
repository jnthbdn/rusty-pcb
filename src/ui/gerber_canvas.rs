use std::cell::RefCell;

use iced::{
    event::Status,
    padding,
    widget::{
        canvas::{self, stroke, Cache, Fill, Frame, Path, Stroke},
        checkbox, column, row, Canvas, Column,
    },
    Color, Length, Point, Rectangle, Renderer, Theme, Vector,
};

use crate::{layer::layer::Layer, ui::message::CanvasLayer};

use super::message::{GerberCanvasMessage, Message};

const STEP_ZOOM_MENU: f32 = 0.5;
const MAX_SCALE: f32 = 100.0;
const MIN_SCALE: f32 = 0.01;
const ZOOM_OPTIMAL_MARGIN: f32 = 5.0;

#[derive(Debug, Default)]
pub struct GerberCanvasInternalState {
    last_mouse_pressed_position: Option<Point>,
}

#[derive(Debug)]
pub struct GerberCanvas {
    pub scale_speed: f32,
    pub translate_speed: f32,

    cache: Cache,
    translate: Vector,
    scale: f32,

    top_layer: Layer,
    bot_layer: Layer,
    drill_layer: Layer,
    outline_layer: Layer,

    canvas_size: RefCell<iced::Rectangle>,

    show_top_layer: bool,
    show_bot_layer: bool,
    show_drill_layer: bool,
    show_outline_layer: bool,
}

impl GerberCanvas {
    pub fn view<'a>(&'a self) -> Column<'a, Message> {
        column![
            Canvas::new(self).width(Length::Fill).height(Length::Fill),
            "Layers",
            row![
                column![
                    checkbox("Bottom", self.show_bot_layer)
                        .on_toggle(|x| Message::GerberCanvas(GerberCanvasMessage::ShowBotLayer(x))),
                    checkbox("Top", self.show_top_layer)
                        .on_toggle(|x| Message::GerberCanvas(GerberCanvasMessage::ShowTopLayer(x))),
                ]
                .spacing(5),
                column![
                    checkbox("Drill", self.show_drill_layer).on_toggle(|x| Message::GerberCanvas(
                        GerberCanvasMessage::ShowDrillLayer(x)
                    )),
                    checkbox("Outline", self.show_outline_layer).on_toggle(|x| {
                        Message::GerberCanvas(GerberCanvasMessage::ShowOutlineLayer(x))
                    })
                ]
                .spacing(5)
            ]
            .spacing(20)
            .width(Length::Fill)
            .padding(padding::left(20))
        ]
        .spacing(5)
    }

    pub fn update(&mut self, message: GerberCanvasMessage) {
        match message {
            GerberCanvasMessage::ZoomIn => {
                let px = self.canvas_size.borrow().width / 2.0;
                let py = self.canvas_size.borrow().height / 2.0;

                self.zoom_on(STEP_ZOOM_MENU, (px, py).into());
                self.force_redraw();
            }
            GerberCanvasMessage::ZoomOut => {
                let px = self.canvas_size.borrow().width / 2.0;
                let py = -self.canvas_size.borrow().height / 2.0;

                self.zoom_on(-STEP_ZOOM_MENU, (px, py).into());
                self.force_redraw();
            }
            GerberCanvasMessage::ResetView => {
                self.reset_view();
                self.force_redraw();
            }
            GerberCanvasMessage::ZoomPointer(amount, pointer) => {
                self.zoom_on(amount, pointer);
            }
            GerberCanvasMessage::Translate { delta_x, delta_y } => {
                self.translate.x += delta_x;
                self.translate.y += delta_y;
                self.force_redraw();
            }
            GerberCanvasMessage::LoadLayer(canvas_layer, layer) => {
                match canvas_layer {
                    CanvasLayer::Top => {
                        self.show_top_layer = true;
                        self.top_layer = layer;
                    }
                    CanvasLayer::Bottom => {
                        self.show_bot_layer = true;
                        self.bot_layer = layer;
                    }
                    CanvasLayer::Drill => {
                        self.show_drill_layer = true;
                        self.drill_layer = layer;
                    }
                    CanvasLayer::Outline => {
                        self.show_outline_layer = true;
                        self.outline_layer = layer;
                    }
                };
                self.reset_view();
                self.force_redraw();
            }
            GerberCanvasMessage::ShowTopLayer(is_show) => {
                if self.top_layer.empty() {
                    self.show_top_layer = false;
                } else {
                    self.show_top_layer = is_show;
                    self.force_redraw();
                }
            }
            GerberCanvasMessage::ShowBotLayer(is_show) => {
                if self.bot_layer.empty() {
                    self.show_bot_layer = false;
                } else {
                    self.show_bot_layer = is_show;
                    self.force_redraw();
                }
            }
            GerberCanvasMessage::ShowDrillLayer(is_show) => {
                if self.drill_layer.empty() {
                    self.show_drill_layer = false;
                } else {
                    self.show_drill_layer = is_show;
                    self.force_redraw();
                }
            }
            GerberCanvasMessage::ShowOutlineLayer(is_show) => {
                if self.outline_layer.empty() {
                    self.show_outline_layer = false;
                } else {
                    self.show_outline_layer = is_show;
                    self.force_redraw();
                }
            }
            GerberCanvasMessage::ClearTopLayer => self.clear_top_layer(),
            GerberCanvasMessage::ClearBottomLayer => self.clear_bottom_layer(),
            GerberCanvasMessage::ClearDrillLayer => self.clear_drill_layer(),
            GerberCanvasMessage::ClearOutlineLayer => self.clear_outline_layer(),
        };
    }

    pub fn force_redraw(&mut self) {
        self.cache.clear();
    }

    pub fn reset_view(&mut self) {
        let path_bounds = self.bot_layer.get_bounds();
        let canvas_bounds = self.canvas_size.borrow();

        self.translate.x = 0.0;
        self.translate.y = 0.0;
        self.scale = 1.0;

        if canvas_bounds.width > 0.0 && canvas_bounds.height > 0.0 {
            let target_scale = (canvas_bounds.width / (path_bounds.width + ZOOM_OPTIMAL_MARGIN))
                .min(canvas_bounds.height / (path_bounds.height + ZOOM_OPTIMAL_MARGIN));

            self.scale = target_scale;
            self.translate.x = (canvas_bounds.width - (path_bounds.width * self.scale)) / 2.0
                - (path_bounds.x * self.scale);
            self.translate.y = -(canvas_bounds.height - (path_bounds.height * self.scale)) / 2.0
                + (path_bounds.y * self.scale);

            self.translate.y += self.canvas_size.borrow().height;
        }
    }

    pub fn zoom_on(&mut self, zoom_amout: f32, point: Point) {
        let old_scale = self.scale;
        self.scale = (self.scale + zoom_amout).clamp(MIN_SCALE, MAX_SCALE);
        let factor = self.scale / old_scale;

        self.translate.x = point.x - (point.x - self.translate.x) * factor;
        self.translate.y = point.y - (point.y - self.translate.y) * factor;
        self.force_redraw();
    }

    pub fn clear_top_layer(&mut self) {
        self.top_layer.clear();
        self.force_redraw();
    }

    pub fn clear_bottom_layer(&mut self) {
        self.bot_layer.clear();
        self.force_redraw();
    }

    pub fn clear_drill_layer(&mut self) {
        self.drill_layer.clear();
        self.force_redraw();
    }

    pub fn clear_outline_layer(&mut self) {
        self.outline_layer.clear();
        self.force_redraw();
    }

    fn show_axis(&self, bounds: &Rectangle, frame: &mut Frame) {
        frame.stroke(
            &Path::new(|b| {
                b.move_to((0.0, 0.0).into());
                b.line_to(((bounds.width - self.translate.x) / self.scale, 0.0).into());
            }),
            Stroke {
                style: stroke::Style::Solid(Color::from_rgb(1.0, 0.0, 0.0)),
                width: 1.0,
                ..Default::default()
            },
        );

        frame.stroke(
            &Path::new(|b| {
                b.move_to((0.0, 0.0).into());
                b.line_to((0.0, self.translate.y / self.scale + bounds.height).into());
            }),
            Stroke {
                style: stroke::Style::Solid(Color::from_rgb(0.0, 1.0, 0.0)),
                width: 1.0,
                ..Default::default()
            },
        );
    }
}

impl Default for GerberCanvas {
    fn default() -> Self {
        Self {
            scale_speed: 0.3,
            translate_speed: 1.0,
            cache: Default::default(),
            scale: 1.0,
            translate: Default::default(),
            bot_layer: Default::default(),
            top_layer: Default::default(),
            drill_layer: Default::default(),
            outline_layer: Default::default(),
            canvas_size: RefCell::new(Rectangle::default()),
            show_top_layer: false,
            show_bot_layer: false,
            show_drill_layer: false,
            show_outline_layer: false,
        }
    }
}

impl canvas::Program<Message> for GerberCanvas {
    type State = GerberCanvasInternalState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            self.canvas_size.replace(bounds);

            frame.fill_rectangle(
                Point::ORIGIN,
                frame.size(),
                Fill {
                    style: canvas::Style::Solid(Color::BLACK),
                    ..Default::default()
                },
            );

            // ----- BEGIN DRAW CENTER CROSS -----
            // frame.translate(Vector {
            //     x: frame.center().x,
            //     y: frame.center().y,
            // });

            // let path = Path::new(|b| {
            //     b.move_to(Point::new(0.0, 1_000_0000.0));
            //     b.line_to(Point::new(0.0, -1_000_000.0));
            //     b.move_to(Point::new(1_000_000.0, 0.0));
            //     b.line_to(Point::new(-1_000_000.0, 0.0));
            // });

            // frame.stroke(
            //     &path,
            //     Stroke {
            //         style: stroke::Style::Solid(Color::from_rgb(1.0, 0.0, 0.0)),
            //         width: 1.0,
            //         line_cap: canvas::LineCap::Square,
            //         ..Default::default()
            //     },
            // );
            // frame.translate(Vector {
            //     x: -frame.center().x,
            //     y: -frame.center().y,
            // });

            // ----- END DRAW CENTER CROSS -----

            frame.translate(self.translate);
            frame.scale(self.scale);

            // Put origin bottom right
            // frame.translate(Vector::new(0.0, bounds.height / self.scale));
            frame.scale_nonuniform(Vector::new(1.0, -1.0));
            self.show_axis(&bounds, frame);

            // ----- BEGIN DRAW CIRCUIT -----

            if self.show_top_layer {
                self.top_layer.draw(frame);
            }

            if self.show_bot_layer {
                self.bot_layer.draw(frame);
            }

            if self.show_drill_layer {
                self.drill_layer.draw(frame);
            }

            if self.show_outline_layer {
                self.outline_layer.draw(frame);
            }

            // ----- END DRAW CIRCUIT -----
        });

        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> (Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(event) => {
                if let Some(cursor_pos) = cursor.position_in(bounds) {
                    match event {
                        iced::mouse::Event::CursorMoved { position } => {
                            match state.last_mouse_pressed_position {
                                Some(last_pos) => {
                                    let vect_move = Vector::new(
                                        position.x - last_pos.x,
                                        position.y - last_pos.y,
                                    );

                                    let modifier = self.translate_speed; // / self.scale;

                                    state.last_mouse_pressed_position = Some(position);

                                    return (
                                        Status::Captured,
                                        Some(Message::GerberCanvas(
                                            GerberCanvasMessage::Translate {
                                                delta_x: vect_move.x * modifier,
                                                delta_y: vect_move.y * modifier,
                                            },
                                        )),
                                    );
                                }
                                None => (),
                            }
                        }
                        iced::mouse::Event::ButtonPressed(button) => match button {
                            iced::mouse::Button::Left => match cursor {
                                iced::mouse::Cursor::Available(point) => {
                                    state.last_mouse_pressed_position = Some(point);
                                    return (Status::Captured, None);
                                }
                                _ => (),
                            },
                            _ => (),
                        },
                        iced::mouse::Event::ButtonReleased(button) => match button {
                            iced::mouse::Button::Left => state.last_mouse_pressed_position = None,
                            _ => (),
                        },
                        iced::mouse::Event::WheelScrolled { delta } => match delta {
                            iced::mouse::ScrollDelta::Lines { x: _, y }
                            | iced::mouse::ScrollDelta::Pixels { x: _, y } => {
                                return (
                                    Status::Captured,
                                    Some(Message::GerberCanvas(GerberCanvasMessage::ZoomPointer(
                                        y * self.scale_speed,
                                        cursor_pos,
                                    ))),
                                );
                            }
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        (Status::Ignored, None)
    }
}
