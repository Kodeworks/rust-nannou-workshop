//Original code and license here: https://github.com/nannou-org/nannou/blob/master/generative_design/random_and_noise/m_1_5_03.rs
use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin, Seedable};

fn main() {
    //We don't initialize the window using `simple_window` here this time,
    // but in the `model` function instead.
    nannou::app(model).update(update).run();
}

struct Model {
    agents: Vec<Agent>,
    noise_scale: f64,
    noise_strength: f64,
    agent_alpha: f32,
    stroke_width: f32,
}

struct Agent {
    vector: Vec2,
    vector_old: Vec2,
    angle: f32,
    step_size: f32,
    win_rect: Rect,
    noise_z: f64,
}

impl Agent{
    fn new(win_rect: Rect, noise_z: f64) -> Self {
        let vector = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),
            );
        Self{
            vector,
            vector_old: vector,
            angle: 0.0,
            step_size: 3.0,
            win_rect,
            noise_z,
        }
    }

    fn update(self: &mut Self) {
        self.vector_old = self.vector;

        self.vector.x += self.angle.cos() * self.step_size;
        self.vector.y += self.angle.sin() * self.step_size;

        if self.vector.x < self.win_rect.left() - 10.0 {
            self.vector.x = self.win_rect.right() + 10.0;
            self.vector_old.x = self.vector.x;
        }
        if self.vector.x > self.win_rect.right() + 10.0 {
            self.vector.x = self.win_rect.left() - 10.0;
            self.vector_old.x = self.vector.x;
        }
        if self.vector.y < self.win_rect.bottom() - 10.0 {
            self.vector.y = self.win_rect.top() + 10.0;
            self.vector_old.y = self.vector.y;
        }
        if self.vector.y > self.win_rect.top() + 10.0 {
            self.vector.y = self.win_rect.bottom() - 10.0;
            self.vector_old.y = self.vector.y;
        }
    }

    fn update_angle(&mut self, noise: Perlin, noise_scale: f64, noise_strength: f64) {

        let n = noise.get([
            self.vector.x as f64 / noise_scale,
            self.vector.y as f64 / noise_scale,
            self.noise_z,
        ]) * noise_strength;

        self.angle = n as f32;
    }

    fn display(&self, draw: &Draw, stroke_weight: f32, agent_alpha: f32) {
        draw.line()
            .start(self.vector_old)
            .end(self.vector)
            .rgba(0.0, 0.0, 0.0, agent_alpha)
            .stroke_weight(stroke_weight * self.step_size);
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .view(view)
        .build()
        .unwrap();


    let noise_z_range = 0.4;
    let agent_count = 1000;
    let agents = (0..agent_count)
        .map(|_| {
            Agent::new(app.window_rect(), noise_z_range)
        })
        .collect();

    Model{
        agents,
        noise_scale: 300.0,
        noise_strength: 10.0,
        agent_alpha: 0.35,
        stroke_width: 0.3,
    }

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new().set_seed(10);

    model.agents.iter_mut()
        .for_each(|agent| {
            agent.update_angle(noise, model.noise_scale, model.noise_strength);
            agent.update();
        });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    model.agents.iter().for_each(|agent| {
        agent.display(&draw, model.stroke_width, model.agent_alpha);
    });

    draw.to_frame(app, &frame).unwrap();
}

