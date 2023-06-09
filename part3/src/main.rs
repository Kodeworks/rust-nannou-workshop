//Original code and license here: https://github.com/nannou-org/nannou/blob/master/generative_design/random_and_noise/m_1_5_03.rs
use nannou::prelude::*;

fn main() {
    //We don't initialize the window using `simple_window` here this time,
    // but in the `model` function instead.
    nannou::app(model).update(update).run();
}

struct Model {
    vector: Vec2, //The movemement vector, i.e. how fast and in what direction we travel
    position: Vec2, // The current position
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .view(view)
        .build()
        .unwrap();


    Model{
        vector: vec2(1.0, 0.0), //moving one pixel to the right per frame
        position: vec2(0.0, 0.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.position += model.vector; //add and assign the value to the position
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.ellipse()
        .xy(model.position)
        .radius(5.0)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

