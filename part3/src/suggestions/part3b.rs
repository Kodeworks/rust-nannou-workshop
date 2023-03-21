//Original code and license here: https://github.com/nannou-org/nannou/blob/master/generative_design/random_and_noise/m_1_5_03.rs
use nannou::prelude::*;

fn main() {
    //We don't initialize the window using `simple_window` here this time,
    // but in the `model` function instead.
    nannou::app(model).update(update).run();
}

struct Model {
    agent: Agent,
}

struct Agent {
    vector: Vec2, //The movemement vector, i.e. how fast and in what direction we travel
    position: Vec2, // The current position
}

impl Agent{
    fn new(win_rect: Rect) -> Self {
        let position = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),
            );
        Self{
            vector: vec2(1.0, 0.0),
            position,
        }
    }

    fn update(self: &mut Self) {
        self.position += self.vector; //add and assign the value to the position
    }

    fn display(self: &Self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(5.0)
            .color(BLACK);
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .view(view)
        .build()
        .unwrap();

    //Call the constructor with the window rect as argument
    let agent = Agent::new(app.window_rect());
    Model{
        agent
    }

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.agent.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    model.agent.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}

