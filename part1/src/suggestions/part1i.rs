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

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time;

    draw.background()
        .color(BLACK);

    let a = Osc::sin_cos(t, 0.25, 0.0) * 100.0 + vec2(0.0, win.top() * 0.5);
    let b = Osc::sin_cos(t, 0.3, 0.0) * 100.0 + vec2(0.0, win.bottom() * 0.5);

    draw.ellipse()
        .xy(a)
        .radius(10.0)
        .color(PINK);

    draw.ellipse()
        .xy(b)
        .radius(10.0)
        .color(PINK);

    draw.line()
        .points(a, b)
        .stroke_weight(1.5)
        .color(PINK);

    draw.to_frame(app, &frame).unwrap();
}

