use nannou::prelude::*;

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

    draw.background()
        .color(BLACK);

    let a = vec2(0.0, win.top() * 0.5);
    let b = vec2(0.0, win.bottom() * 0.5);

    draw.ellipse()
        .xy(a)
        .radius(10.0)
        .color(PINK);

    draw.ellipse()
        .xy(b)
        .radius(10.0)
        .color(PINK);

    draw.to_frame(app, &frame).unwrap();
}

