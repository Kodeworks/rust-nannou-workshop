use nannou::prelude::*;
use common::Osc;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
struct Model { }

fn model(_app: &App) -> Model {
    Model { }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background()
        .color(rgb(0.1, 0.1, 0.2));

    let win = app.window_rect();

    let t = app.time; //Make a short hand variable for time
    
    let radius = 20.0;

    draw.text(format!("Seconds: {}", fmod(app.time, 10.0) ).as_str())
        .x_y(100.0, win.top() - 10.0)
        .color(WHITE);

    let y = map_range(0.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::sin")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::sin(t, freq, 0.0) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let y = map_range(1.0/10.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::cos")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::cos(t, freq, 0.0) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let y = map_range(2.0/10.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::saw")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::saw(t, freq, 0.0) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let y = map_range(3.0/10.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::tri")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::tri(t, freq, 0.0) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let y = map_range(4.0/10.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::square")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::square(t, freq, 0.0) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let y = map_range(5.0/10.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::pwm width: 0.75")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::pwm(t, freq, 0.0, 0.75) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let y = map_range(6.0/10.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::step num_steps: 10")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::step(t, freq, 0.0, 10) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let y = map_range(7.0/10.0, 0.0, 1.0, win.top() - radius, win.bottom() + radius);
    let freq = 10.0.recip();
    draw.text("Osc::step_smooth num_steps: 10, smoothness: 0.5")
        .x_y(win.left() + 100.0, y)
        .color(WHITE);

    draw.ellipse()
        .y(y)
        .x(Osc::step_smooth(t, freq, 0.0, 10, 0.5) * 300.0)
        .radius(20.0)
        .color(WHITE);

    let travel_radius = 100.0;
    let xy = vec2(
        map_range(0.0/4.0, 0.0, 1.0, win.left() + travel_radius, win.right() - travel_radius),
        win.bottom() + travel_radius);

    draw.text("sin_cos")
        .xy(xy)
        .color(WHITE);

    let xy = vec2(
        map_range(0.0/4.0, 0.0, 1.0, win.left() + travel_radius, win.right() - travel_radius),
        win.bottom() + travel_radius);

    draw.rect()
        .xy(xy)
        .wh(vec2(travel_radius, travel_radius) * 2.0)
        .no_fill()
        .stroke(WHITE)
        .stroke_weight(2.0);

    draw.xy(xy)
        .ellipse()
        .radius(radius)
        .xy(Osc::sin_cos(t, freq, 0.0) * (travel_radius - radius))
        .color(WHITE);

    let xy = vec2(
        map_range(1.0/4.0, 0.0, 1.0, win.left() + travel_radius, win.right() - travel_radius),
        win.bottom() + travel_radius);

    draw.text("diamond")
        .xy(xy)
        .color(WHITE);

    draw.rect()
        .xy(xy)
        .wh(vec2(travel_radius, travel_radius) * 2.0)
        .no_fill()
        .stroke(WHITE)
        .stroke_weight(2.0);

    draw.xy(xy)
        .ellipse()
        .radius(radius)
        .xy(Osc::diamond(t, freq, 0.0) * (travel_radius - radius))
        .color(WHITE);

    let xy = vec2(
        map_range(2.0/4.0, 0.0, 1.0, win.left() + travel_radius, win.right() - travel_radius),
        win.bottom() + travel_radius);

    draw.text("zigzag")
        .xy(xy)
        .color(WHITE);

    draw.rect()
        .xy(xy)
        .wh(vec2(travel_radius, travel_radius) * 2.0)
        .no_fill()
        .stroke(WHITE)
        .stroke_weight(2.0);

    draw.xy(xy)
        .ellipse()
        .radius(radius)
        .xy(Osc::zigzag(t, freq, 0.0) * (travel_radius - radius))
        .color(WHITE);

    let xy = vec2(
        map_range(3.0/4.0, 0.0, 1.0, win.left() + travel_radius, win.right() - travel_radius),
        win.bottom() + travel_radius);

    draw.text("rectangle")
        .xy(xy)
        .color(WHITE);

    draw.rect()
        .xy(xy)
        .wh(vec2(travel_radius, travel_radius) * 2.0)
        .no_fill()
        .stroke(WHITE)
        .stroke_weight(2.0);

    draw.xy(xy)
        .ellipse()
        .radius(radius)
        .xy(Osc::rectangle(t, freq, 0.0) * (travel_radius - radius))
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
