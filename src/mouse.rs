use nannou::prelude::*;

pub struct Mouse {
    pub previous: Vec2,
    pub current: Vec2,
    pub direction: Vec2,
    pub radius: f32,
    pub pressed: bool,
}

impl Mouse {
    pub fn new(x: f32, y: f32) -> Self {
        let pos = vec2(x, y);
        Self {
            previous: pos,
            current: pos,
            direction: Vec2::ZERO,
            radius: 30.0,
            pressed: false,
        }
    }

    pub fn update(&mut self, x: f32, y: f32) {
        self.previous = self.current;
        self.current = vec2(x, y);
        self.direction = (self.current - self.previous).normalize_or_zero();
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.current)
            .radius(self.radius)
            .hsva(0.33, 0.4, 0.4, 0.2);
    }
}
