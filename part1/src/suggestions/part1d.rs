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

    //Use the top and bottom values from the window rect to make it simpler to convert value
    // ranges.
    const TOP_RATIO: f32 = 2.0/3.0; // The ratio need only be calculate once, so we use const.
    const BOTTOM_RATIO: f32 = 1.0/4.0; // If we had used `let top_ratio = 2.0/3.0` the Rust
                                       // compiler probably would have optimized this into a
                                       // constant anyway. The default immutability of variables in
                                       // Rust makes many it easier for the compiler to figure out
                                       // ...But hey, can't a programmer have a bit of fun with 
                                       //  unecessary optimization from time to time..?
    let y_top = map_range(TOP_RATIO, 0.0, 1.0, win.bottom(), win.top());
    let y_bottom = map_range(BOTTOM_RATIO, 0.0, 1.0, win.bottom(), win.top());
    let y_pos = map_range(app.time.sin(), -1.0, 1.0, y_bottom, y_top);

    draw.ellipse()
        .y(y_pos)
        .color(circle_color);

    draw.to_frame(app, &frame).unwrap();
}
