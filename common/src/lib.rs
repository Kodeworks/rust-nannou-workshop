use nannou::prelude::*;

pub struct Osc;

pub fn unit_to_range(value: f32, min: f32, max: f32) -> f32 {
    value * 0.5 * (max + min)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn multismoothstep(num_steps: i32, x: f32, smoothness: f32) -> f32 {
    let n = num_steps as f32;
    let xn = x * n;
    let step_index = (xn as i32).min(num_steps - 1);
    let remainder = xn.fract();
    let next_delta = ((step_index + 1) % num_steps) - step_index;
    let steepness = abs(1.0 - clamp(smoothness, 0.0, 1.0)) * 0.5;
    let smoothing = smoothstep(steepness, abs(1.0 - steepness), remainder) * next_delta as f32;
    (step_index as f32 + smoothing) / (n-1.0)
}

impl Osc {
    pub const PHASE90: f32 = 0.5 * std::f32::consts::PI;
    pub const PHASE180: f32 = std::f32::consts::PI;
    pub const PHASE270: f32 = 1.5 * std::f32::consts::PI;
    pub fn sin(time: f32, freq: f32, phase: f32) -> f32 {
        //Notice that there is not semicolon at the end of the last line in this 
        // function.
        //This is the syntax for what is returned from the function.
        //It is also possible to use `return` keyword, e.g. for early function returns.
        (time * TAU * freq + phase).sin()
    }

    pub fn cos(time: f32, freq: f32, phase: f32) -> f32 {
        (time * TAU * freq + phase).cos()
    }

    pub fn sin_cos(time: f32, freq: f32, phase: f32) -> Vec2 {
        let (x,y) = (time * TAU * freq + phase).sin_cos();
        vec2(x,y)
    }

    pub fn saw(time: f32, freq: f32, phase: f32) -> f32 {
        fmod( time * freq + phase, 1.0 ) * 2.0 - 1.0
    }

    pub fn tri(time: f32, freq: f32, phase: f32) -> f32 {
        let val = Self::saw(time, freq, phase);

        if val > 0.0 {
            abs(1.0 - val) * 2.0 - 1.0
        } else {
            val.mul_add(2.0, 1.0)
        }
    }

    pub fn diamond(time: f32, freq: f32, phase: f32) -> Vec2 {
        vec2(Self::tri(time, freq, phase),
            Self::tri(time, freq, phase + 0.25))
    }

    pub fn zigzag(time: f32, freq: f32, phase: f32) -> Vec2 {
        vec2(Self::saw(time, freq, phase),
            Self::saw(time, freq, phase + 0.25))
    }

    pub fn square(time: f32, freq: f32, phase: f32) -> f32 {
        Self::pwm(time, freq, phase, 0.5)
    }

    pub fn rectangle(time: f32, freq: f32, phase: f32) -> Vec2 {
        vec2(Self::square(time, freq, phase),
            Self::square(time, freq, phase + 0.25))
    }

    pub fn pwm(time: f32, freq: f32, phase: f32, width: f32) -> f32 {
        let val = Self::saw(time, freq, phase);
        let threshold = width.mul_add(2.0,  -1.0);
        if val < threshold {
            -1.0
        } else {
            1.0
        }
    }

    pub fn step(time: f32, freq: f32, phase: f32, num_steps: i32) -> f32 {
        Self::step_smooth(time, freq, phase, num_steps, 0.0)
    }

    pub fn step_smooth(time: f32, freq: f32, phase: f32, num_steps: i32, smoothness: f32) -> f32 {
        let val = Self::saw(time, freq, phase).mul_add(0.5, 0.5);
        multismoothstep(num_steps, val, smoothness).mul_add(2.0, -1.0)
    }

    pub fn parametric(time: f32, freq: f32, phase: f32, freq_ratio: f32, mul_ratio: f32) -> f32 {
        let a = mul_ratio;
        let b = abs(1.0 - mul_ratio);
        (Self::sin(time, freq, phase) * a) + (Self::sin(time, freq * freq_ratio, phase) * b)
    }
}
