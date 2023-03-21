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
}

struct Agent {
    vector: Vec2,
    angle: f32,
    step_size: f32,
    win_rect: Rect,
    noise_z: f64,
}

impl Agent{
    fn new(win_rect: Rect, noise_z: f64) -> Self {
        let position = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),
            );
        Self{
            vector: position,
            angle: 0.0,
            step_size: 3.0,
            win_rect,
            noise_z,
        }
    }

    fn update(self: &mut Self) {
        self.vector.x += self.angle.cos() * self.step_size;
        self.vector.y += self.angle.sin() * self.step_size;

        if self.vector.x < self.win_rect.left() - 10.0 {
            self.vector.x = self.win_rect.right() + 10.0;
        }
        if self.vector.x > self.win_rect.right() + 10.0 {
            self.vector.x = self.win_rect.left() - 10.0;
        }
        if self.vector.y < self.win_rect.bottom() - 10.0 {
            self.vector.y = self.win_rect.top() + 10.0;
        }
        if self.vector.y > self.win_rect.top() + 10.0 {
            self.vector.y = self.win_rect.bottom() - 10.0;
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

    fn display(self: &Self, draw: &Draw) {
        draw.ellipse()
            .xy(self.vector)
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


    let noise_z_range = 0.4;
    let agent_count = 50;
    let agents = (0..agent_count)
        .map(|_| {
            Agent::new(app.window_rect(), noise_z_range)
        })
        .collect();

    Model{
        agents,
        noise_scale: 300.0,
        noise_strength: 10.0,
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
        agent.display(&draw);
    });

    draw.to_frame(app, &frame).unwrap();
}

