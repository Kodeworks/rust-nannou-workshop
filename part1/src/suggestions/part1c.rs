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
    //The output range from the `sin()` function is -1.0 - 1.0
    //Since the window coordinates for nannou has x:0.0,y:0.0 as the center of the window,
    // converting the range -1.0 to 1.0 to the full height of the window is as simple as
    // multiplying with half the window height.
    let y_pos = app.time.sin() * win.h() * 0.5;
    //Use the draw instance to draw an ellipse.
    draw.ellipse()
        .y(y_pos)
        .color(circle_color);

    draw.to_frame(app, &frame).unwrap();
}
