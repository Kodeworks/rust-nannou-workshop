use nannou::prelude::*;
use common::Osc;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn draw_line_trail(time: f32, draw: &Draw, num_lines: i32, speed: f32, tail_length: f32, color: Srgb)  {
    for i in 0..num_lines { // 0..10 is the notation for a range in Rust 
        let tt = time + (i as f32 * tail_length); // offset the time with the scaled index value
        let aa = vec2(
            Osc::parametric(tt, 0.25 * speed, 0.0, 1.02, 0.5),
            Osc::parametric(tt, 0.3 * speed, Osc::PHASE90, 1.87, 0.3),
            );
        let bb = vec2(
            Osc::parametric(tt, 0.35 * speed, 0.0, 1.32, 0.9),
            Osc::parametric(tt, 0.5 * speed, Osc::PHASE90, 0.8, 0.1),
            );
        let a = aa * 300.0;
        let b = bb * 300.0;

        draw.line()
            .points(a, b)
            .stroke_weight(1.5)
            .color(color);
    }
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    let _win = app.window_rect();
    let t = app.time;

    draw.background()
        .color(BLACK);

    //We have to call .into_format() on the color constant to convert it into a f32 based color.
    draw_line_trail(t, &draw, 10, 0.5, 0.1, PINK.into_format());
    draw_line_trail(t + 100.0, &draw, 13, 0.4, 0.15, rgb(0.1, 0.1, 0.9));

    draw.to_frame(app, &frame).unwrap();
}

