use chrono::Utc;
use mouse::Mouse;
use nannou::{
    noise::{NoiseFn, OpenSimplex, Seedable},
    prelude::*,
};
use strand::Strand;

mod mouse;
mod strand;

struct Model {
    mouse: Mouse,
    strands: Vec<Strand>,
}

fn model(app: &App) -> Model {
    let id = app
        .new_window()
        .title("velvet")
        .size(800, 800)
        .view(view)
        .mouse_pressed(pressed)
        .mouse_released(released)
        .build()
        .unwrap();

    app.window(id).unwrap().set_cursor_visible(false);

    let mouse = Mouse::new(app.mouse.x, app.mouse.y);
    let bounds = app.window_rect().pad(mouse.radius * 2.0);
    let mut strands = Vec::new();

    let seed = Utc::now().timestamp() as u32;
    let noise = OpenSimplex::new().set_seed(seed);
    println!("seed: {}", noise.seed());

    for y in (bounds.bottom() as isize..bounds.top() as isize).step_by(12) {
        for x in (bounds.left() as isize..bounds.right() as isize).step_by(12) {
            let x = x as f32 + (random_f32() - 0.5) * 10.0;
            let y = y as f32 + (random_f32() - 0.5) * 10.0;
            let start = vec2(x, y);
            let scale = 0.003;

            // NOTE: As far as I can tell, OpenSimplex returns a number between -0.5 and 0.5
            let angle = noise.get([start.x as f64 * scale, start.y as f64 * scale]) as f32 * 2.0;

            strands.push(Strand::new(start.x, start.y, angle * PI));
        }
    }

    Model { mouse, strands }
}

fn pressed(_app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.mouse.pressed = true;
    }
}

fn released(_app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.mouse.pressed = false;
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.mouse.update(app.mouse.x, app.mouse.y);

    for strand in &mut model.strands {
        strand.update(&model.mouse);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(0.94, 0.9, 0.1));

    for strand in &model.strands {
        strand.draw(&draw);
    }

    model.mouse.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
