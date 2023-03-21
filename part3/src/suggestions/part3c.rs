//Original code and license here: https://github.com/nannou-org/nannou/blob/master/generative_design/random_and_noise/m_1_5_03.rs
use nannou::prelude::*;

fn main() {
    //We don't initialize the window using `simple_window` here this time,
    // but in the `model` function instead.
    nannou::app(model).update(update).run();
}

struct Model {
    agents: Vec<Agent>,
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

    let agent_count = 50;

    //maps the range over a closure function that returns instances of `Agent`
    let agents = (0..agent_count)
        .map(|_| Agent::new(app.window_rect()))
        .collect();

    Model{
        agents
    }

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //We have to use a speciaf iterator, iter_mut, because we mutating the data in each agent
    model.agents.iter_mut()
        .for_each(|agent| agent.update());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    model.agents.iter().for_each(|agent| {
        agent.display(&draw);
    });

    draw.to_frame(app, &frame).unwrap();
}

