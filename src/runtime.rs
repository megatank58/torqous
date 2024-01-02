use crate::structs::{object::Object, particle::Particle};
use eframe::egui;
use egui::{Color32, Rect, Stroke, pos2};
use std::{collections::HashMap, f32::INFINITY};

#[derive(Debug, Clone)]
pub struct Runtime<'a> {
    pub particles: HashMap<&'a str, Particle>,
    pub objects: HashMap<&'a str, Object>,
    pub k_friction: f32,
    pub s_friction: f32,
    pub gravity: f32,
    internal_time: f32,
    pub end_time: f32,
    pub steps: f32,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
            objects: HashMap::new(),
            k_friction: 0.0,
            s_friction: 0.0,
            gravity: 10.0,
            internal_time: 0.0,
            end_time: 0.0,
            steps: 1.0,
        }
    }

    pub fn init_ui(&self) {}

    pub fn get_particle(&mut self, name: &'a str) -> Option<&Particle> {
        self.particles.get(&name)
    }

    pub fn get_object(&mut self, name: &'a str) -> Option<&Object> {
        self.objects.get(&name)
    }

    pub fn set_particle(&mut self, name: &'a str, particle: Particle) -> &mut Self {
        self.particles.insert(name, particle);

        self
    }

    pub fn set_object(&mut self, name: &'a str, object: Object) -> &mut Self {
        self.objects.insert(name, object);

        self
    }
}

impl<'a> eframe::App for Runtime<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.internal_time < self.end_time {
            let mut particles = HashMap::new();
            for (name, particle) in &mut self.particles {
                particles.insert(*name, *particle.calculate(self.steps));
            }
            self.particles = particles;

            let mut objects = HashMap::new();
            for (name, object) in &mut self.objects {
                object.gravity(self.gravity);
                object.friction(self.s_friction, self.k_friction, self.gravity);
                objects.insert(*name, *object.calculate(self.steps));
            }
            self.objects = objects;

            self.internal_time += self.steps;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let y = ui
                .label("Time".to_string() + &self.internal_time.to_string())
                .rect
                .bottom();
            let painter = ui.painter_at(Rect::everything_below(y));
            let stroke = Stroke::new(30.0, Color32::GREEN);

            painter.line_segment(
                [pos2(-INFINITY, 0.0), pos2(INFINITY, 0.0)],
                stroke,
            );

            for (name, object) in &self.objects {
                let pos = object.particle.position;
                match object.shape {
                    crate::structs::object::ShapeType::Circle(r) => {
                        painter.circle(pos2(pos.x, pos.y), r, Color32::GREEN, stroke);
                    }
                    crate::structs::object::ShapeType::Triangle(_, _, _) => todo!(),
                    crate::structs::object::ShapeType::Quadrilateral(_, _, _, _) => todo!(),
                }
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(
            (self.steps * 1000.0) as u64,
        ));
    }
}
