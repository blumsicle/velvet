use nannou::prelude::*;

use crate::mouse::Mouse;

pub struct Strand {
    pub start: Vec2,
    pub direction: Vec2,
    pub magnitude: f32,
    pub color: Hsv,
    pub thickness: f32,
}

impl Strand {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        let color = Self::get_color(angle);
        Self {
            start: vec2(x, y),
            direction: vec2(angle.cos(), angle.sin()),
            magnitude: 5.0,
            color,
            thickness: 5.0,
        }
    }

    fn get_color(angle: f32) -> Hsv {
        let saturation = map_range(angle.abs(), 0.0, PI, 0.7, 0.9);
        let brightness = map_range(angle.abs(), 0.0, PI, 0.4, 0.7);
        hsv(0.98, saturation, 1.0 - brightness)
    }

    pub fn update(&mut self, mouse: &Mouse) {
        if mouse.pressed
            && mouse.direction != Vec2::ZERO
            && mouse.current.distance(self.start) < mouse.radius
        {
            if (mouse.current.x < self.start.x && mouse.direction.x > 0.0)
                || (mouse.current.x > self.start.x && mouse.direction.x < 0.0)
                || (mouse.current.y < self.start.y && mouse.direction.y > 0.0)
                || (mouse.current.y > self.start.y && mouse.direction.y < 0.0)
            {
                self.direction = self.direction.lerp(mouse.direction, 0.4);
                self.color = Self::get_color(self.direction.angle());
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let end = self.start + (self.direction * self.magnitude);
        draw.line()
            .points(self.start, end)
            .color(self.color)
            .weight(self.thickness)
            .caps_round();
    }
}
