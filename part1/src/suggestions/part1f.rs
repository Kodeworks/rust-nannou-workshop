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

    let win = app.window_rect();

    let circle_radius_a = 50.0;
    let pos_a = vec2(
        map_range(app.time.sin(), -1.0, 1.0, 
                  win.left() + circle_radius_a,
                  win.right() - circle_radius_a),
        map_range(app.time.cos(), -1.0, 1.0, 
                  win.bottom() + circle_radius_a, 
                  win.top() - circle_radius_a));

    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        //Multiply the time before calling the sin() function doubles the frequency.
        map_range((2.0 * app.time).sin(), -1.0, 1.0, 
                  win.left() + circle_radius_b,
                  win.right() - circle_radius_b ),
        //Negating either the sin() or cos() part to make it travel in opposite direction
        map_range(-(2.0 * app.time).cos(), -1.0, 1.0, 
                  win.bottom() + circle_radius_b, 
                  win.top() - circle_radius_b ));

    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);
    draw.ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);

    draw.to_frame(app, &frame).unwrap();
}
