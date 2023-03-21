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

    draw.background()
        .color(CYAN);

    let circle_color = MAGENTA;

    let win = app.window_rect();

    let y_pos = map_range(app.time.sin(), -1.0, 1.0, win.bottom(), win.top());
    let x_pos = map_range(app.time.cos(), -1.0, 1.0, win.left(), win.right());

    draw.ellipse()
        .y(y_pos)
        .x(x_pos)
        .color(circle_color);

    draw.to_frame(app, &frame).unwrap();
}
