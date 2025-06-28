use std::collections::HashMap;

use clipper2::{FillRule, Path, Paths, Point};
use gerber_parser::gerber_doc::GerberDoc;
use gerber_types::{Aperture, InterpolationMode};
use iced::widget::canvas::Frame;
use iced::widget::canvas::Path as IcedPath;
use iced::Color;
use iced::Point as IcedPoint;
use log::error;

use super::vec2::Vec2;

const CICRLE_RES: u32 = 100;

#[derive(Debug, Default, Clone)]
pub struct Layer {
    paths: Paths,
    color: Color,
}

impl Layer {
    pub fn from_gerber(gerber: &GerberDoc, color: Color) -> Self {
        let mut geos = Self::default();
        let apertures: &HashMap<i32, Aperture> = &gerber.apertures;

        let mut current_pos = Vec2::default();
        let mut current_aperture: Option<&Aperture> = None;
        let mut current_mode: InterpolationMode = InterpolationMode::Linear;

        geos.color = color;

        for cmd in &gerber.commands {
            match cmd {
                gerber_types::Command::FunctionCode(function_code) => match function_code {
                    gerber_types::FunctionCode::DCode(dcode) => match dcode {
                        gerber_types::DCode::SelectAperture(id) => {
                            if apertures.contains_key(id) {
                                current_aperture = Some(&apertures[id]);
                            } else {
                                error!("Unknown aperture #{id}");
                                current_aperture = None;
                            }
                        }
                        gerber_types::DCode::Operation(operation) => match operation {
                            gerber_types::Operation::Move(coord) => current_pos.set(coord),
                            gerber_types::Operation::Interpolate(
                                coordinates,
                                _coordinate_offset,
                            ) => match current_mode {
                                InterpolationMode::Linear => {
                                    let rect: Paths = Self::line_to_rect_path(
                                        current_pos,
                                        coordinates.into(),
                                        Self::thickness_aperture(&current_aperture),
                                    )
                                    .into();
                                    match rect
                                        .to_clipper_subject()
                                        .add_clip(Self::aperture_path(
                                            &current_aperture.unwrap(),
                                            current_pos,
                                        ))
                                        .add_clip(Self::aperture_path(
                                            &current_aperture.unwrap(),
                                            coordinates.into(),
                                        ))
                                        .union(FillRule::default())
                                    {
                                        Ok(paths) => geos.union(paths),
                                        Err(e) => {
                                            error!("Failed to create trace path. Error: {e}")
                                        }
                                    }
                                    current_pos.set(coordinates);
                                }
                                InterpolationMode::ClockwiseCircular => todo!(),
                                InterpolationMode::CounterclockwiseCircular => todo!(),
                            },
                            gerber_types::Operation::Flash(coordinates) => {
                                match current_aperture {
                                    Some(aperture) => {
                                        geos.union(
                                            Self::aperture_path(aperture, coordinates.into())
                                                .into(),
                                        );
                                    }
                                    None => error!("No aperture selected for flash operation"),
                                };
                                current_pos.set(coordinates);
                            }
                        },
                    },
                    gerber_types::FunctionCode::GCode(gcode) => match gcode {
                        gerber_types::GCode::InterpolationMode(mode) => current_mode = mode.clone(),
                        gerber_types::GCode::RegionMode(_) => {
                            error!("Region mode command not supported...")
                        }
                        gerber_types::GCode::QuadrantMode(_) => {
                            error!("Quadrant Mode command not supported...")
                        }
                        gerber_types::GCode::Comment(_) => (),
                    },
                    gerber_types::FunctionCode::MCode(mcode) => match mcode {
                        gerber_types::MCode::EndOfFile => (),
                    },
                },
                gerber_types::Command::ExtendedCode(_extended_code) => {
                    error!("No exented code supported...");
                }
            }
        }

        geos
    }

    pub fn draw(&self, frame: &mut Frame) {
        for path in self.paths.iter() {
            let mut first_point: Option<IcedPoint> = None;
            let iced_path = IcedPath::new(|b| {
                for pts in path {
                    if first_point.is_none() {
                        b.move_to(IcedPoint::new(pts.x() as f32, pts.y() as f32));
                        first_point = Some(IcedPoint::new(pts.x() as f32, pts.y() as f32));
                    } else {
                        b.line_to(IcedPoint::new(pts.x() as f32, pts.y() as f32));
                    }
                }

                if let Some(pt) = first_point {
                    b.line_to(pt);
                }
            });

            frame.stroke(
                &iced_path,
                iced::widget::canvas::Stroke {
                    style: iced::widget::canvas::stroke::Style::Solid(self.color),
                    width: 1.0,
                    ..Default::default()
                },
            );
        }
    }

    pub fn get_bounds(&self) -> iced::Rectangle {
        if self.paths.len() == 0 {
            iced::Rectangle::new((0.0, 0.0).into(), (0.0, 0.0).into())
        } else {
            let bounds = self.paths.bounds();

            iced::Rectangle::new(
                (bounds.min.x() as f32, bounds.min.y() as f32).into(),
                (bounds.size().x() as f32, bounds.size().y() as f32).into(),
            )
        }
    }

    pub fn empty(&self) -> bool {
        self.paths.len() == 0
    }

    pub fn clear(&mut self) {
        self.paths = Paths::default();
    }

    fn thickness_aperture(aperture: &Option<&Aperture>) -> f64 {
        match aperture {
            Some(x) => match x {
                Aperture::Circle(circle) => circle.diameter,
                Aperture::Rectangle(rect) => rect.x.max(rect.y),
                Aperture::Obround(rect) => rect.x.max(rect.y),
                Aperture::Polygon(_polygon) => todo!(),
                Aperture::Other(_) => todo!(),
            },
            None => {
                error!("No aperture for thickness");
                1.0
            }
        }
    }

    fn line_to_rect_path(from: Vec2, to: Vec2, thickness: f64) -> Path {
        let mut vec = Vec2::new_normalize(to.x - from.x, to.y - from.y);
        vec.mult(thickness / 2.0);

        let norm = Vec2::new(-vec.y, vec.x);
        let mut inv_norm = norm.clone();
        inv_norm.inv();

        let v: Vec<Point> = vec![
            (from + norm).into(),
            (to + norm).into(),
            (to + inv_norm).into(),
            (from + inv_norm).into(),
        ];

        v.into()
    }

    fn create_circle_path(center: Vec2, radius: f64, resolution: u32) -> Path {
        let mut p: Vec<Point> = Vec::with_capacity(resolution as usize);
        let step = (360.0 / resolution as f64).to_radians();

        for i in 0..resolution {
            let angle = step * (i as f64);
            p.push(
                (
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                )
                    .into(),
            );
        }

        p.into()
    }

    fn aperture_path(aperture: &Aperture, center: Vec2) -> Path {
        match aperture {
            Aperture::Circle(circle) => {
                Self::create_circle_path(center, circle.diameter / 2.0, CICRLE_RES)
            }
            Aperture::Rectangle(rect) => {
                let origin = Vec2::new(center.x - rect.x / 2.0, center.y - rect.y / 2.0);
                vec![
                    (origin.x, origin.y),
                    (origin.x + rect.x, origin.y),
                    (origin.x + rect.x, origin.y + rect.y),
                    (origin.x, origin.y + rect.y),
                ]
                .into()
            }
            Aperture::Obround(rect) => {
                if rect.x == rect.y {
                    Self::create_circle_path(center, rect.x / 2.0, CICRLE_RES)
                } else {
                    todo!()
                }
            }
            Aperture::Polygon(_polygon) => todo!(),
            Aperture::Other(_) => {
                error!("Unsupported \"Other\" aperture type");
                Path::default()
            }
        }
    }

    fn union(&mut self, paths: Paths) {
        match self
            .paths
            .to_clipper_subject()
            .add_clip(paths)
            .union(FillRule::default())
        {
            Ok(new) => self.paths = new,
            Err(e) => error!("Failed to union paths. Error: {e}"),
        }
    }
}
