use crate::structs::{object::Object, particle::Particle};
use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Runtime<'a> {
    pub particles: HashMap<&'a str, Particle>,
    pub objects: HashMap<&'a str, Object>,
    pub k_friction: f64,
    pub s_friction: f64,
    pub gravity: f64,
    pub time: f64,
    pub steps: f64,
    pub ui: Ui<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
            objects: HashMap::new(),
            k_friction: 0.0,
            s_friction: 0.0,
            gravity: 10.0,
            time: 0.0,
            steps: 1.0,
            ui: Ui::default(),
        }
    }

    pub fn init_ui(&self) {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Torqous",
            native_options,
            //TODO: Somwhow pass runtime through this
            Box::new(|cc| Box::new(Ui::new(cc))),
        )
        .unwrap();
    }

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

    pub fn set_k_friction(&mut self, k_friction: f64) -> &mut Self {
        self.k_friction = k_friction;

        self
    }

    pub fn set_s_friction(&mut self, s_friction: f64) -> &mut Self {
        self.s_friction = s_friction;

        if self.k_friction == 0.0 {
            self.k_friction = s_friction;
        }

        self
    }

    pub fn set_gravity(&mut self, gravity: f64) -> &mut Self {
        self.gravity = gravity;

        self
    }

    pub fn run(self, time: f64) -> Self {
        self.run_for(time, |a| a)
    }

    pub fn run_for<F>(mut self, time: f64, f: F) -> Self
    where
        F: Fn(Runtime<'a>) -> Runtime<'a>,
    {
        while self.time != time {
            self = f(self);

            let mut particles = HashMap::new();
            for (name, mut particle) in self.particles {
                particles.insert(name, *particle.calculate(self.steps));
            }
            self.particles = particles;

            let mut objects = HashMap::new();
            for (name, mut object) in self.objects {
                object.gravity(self.gravity);
                object.friction(self.s_friction, self.k_friction, self.gravity);
                objects.insert(name, *object.calculate(self.steps));
            }
            self.objects = objects;

            self.time += self.steps;
        }
        self
    }
}

#[derive(Debug, Clone)]
pub struct Ui<'a> {
    pub runtimes: Vec<Runtime<'a>>,
}

impl<'a> Ui<'a> {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self { runtimes: vec![] }
    }

    pub fn default() -> Self {
        Self { runtimes: vec![] }
    }
}

impl<'a> eframe::App for Ui<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.runtimes[0].gravity.to_string());
        });
    }
}
