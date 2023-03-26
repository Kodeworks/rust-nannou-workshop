# 🌊 Part 3: Perlin noise flow fields

This part is a gateway to the official examples in Nannou.
In the [Sources for inspiration and learning](/texts/sources_of_inspiration.md) document there are links to the three main for examples from the [Nannou repository](https://github.com/nannou-org/nannou).

We will in this part look at one of the examples from [Generative Design](https://github.com/nannou-org/nannou).
The example we will look at is [m_1_5_03](https://github.com/nannou-org/nannou/blob/master/generative_design/random_and_noise/m_1_5_03.rs).

📎 _The example already exists as a p5.js program [here](http://www.generative-gestaltung.de/2/sketches/?02_M/M_1_5_03) that you can test also. It can be nice to compare the JavaScript/p5.js to the Rust/Nannou implementation._

As with Part 2, this part will build the implementation in steps, but not in a very "exercisey" way.

We will work with the code in `part3/src/main.rs` and make our implementation there.
To run your code you can run this command in the terminal:

```
cargo run --bin part3
```

If you run the code now you will see a much simpler version, it is only on a black dot moving towards the right.

The suggestions for the exercises can be run with:
```
cargo run --bin part3a
```
Substituting the `a` with the letter of the exercise.

<br/>

---

<br/>

## 🌊 Exercise 3-A: A struct of its own implementation
Our starting point is a simpler program where we extend a line towards the right.

We are now using the `model` and `update` function to initialize and update our state.

📜 Let's put the circle into a separate struct called `Agent`, so that we can keep the data and behaviour defined in a separate entity.
Give the `Agent` data members for `vector` and `position`, and implement a `display` and `update` function for it.

The calling code in our program's `update` function will look like this:
```rust
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.agent.update();
}
```

and the calling code in the `view` function will look like this:
```rust 
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    model.agent.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
```


<details><summary> 🙈 An ellipse in its own abstraction </summary>

We now have two `structs`:
```rust
struct Model {
    agent: Agent,
}

struct Agent {
    vector: Vec2, //The movement vector, i.e. how fast and in what direction we travel
    position: Vec2, // The current position
}
```
Initialized like this:

```rust
    let agent = Agent{
        vector: vec2(1.0, 0.0), //moving one pixel to the right per frame
        position: vec2(0.0, 0.0),
    };

    Model{
        agent
    }
```

</details>

Running `cargo run --bin part3a` shows us a moving circle again, now in its own struct!
<br/>

---

<br/>

## 🌊 Exercise 3-B: Initialize Agent with constructor function
Instead of initializing the struct directly we would like a separate function that does the work for us.

📜 Implement a constructor for the `Agent` that initializes the position to a random position on the screen.

<details><summary> 🙈 Constructing struct instances: </summary>

```rust
impl Agent{
    //A constructor function does not have the `self` argument, i.e. it is called
    // by `Agent::new` syntax.
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

    //[...snip...]
}
```
And it is called like this:
```rust
    //Call the constructor with the window rect as argument
    let agent = Agent::new(app.window_rect());
```

</details>

Running `cargo run --bin part3b` shows us a moving circle again, now in its own struct!

<br/>

---

<br/>

## 🌊 Exercise 3-C: An army of `Agents`
If we want that cool particle agent army energy going, we have to create lots of them.
Now we will have to add one constructor call per agent we create.

📜 Let's use a looping statement and create multiple agents and store them in a Vec.

<details><summary>💡 This time we will use an Iterator over a range, instead of a `for` loop:</summary>

```rust
    let items = (0..50)
        .map(|_| Widget::new()) 
        .collect();
```
This is a cleaner pattern than if we e.g. were to create a mutable variable and push to the items vector in a `for` loop.

</details>

<details><summary> 🙈 Creating multiples using Iterators: </summary>

Out Model now uses a `Vec` with `Agent`s inside:
```rust
struct Model {
    agents: Vec<Agent>,
}
```

And we construct the agent army like this:
```rust
    let agent_count = 50;

    let agents = (0..agent_count)
        .map(|_| Agent::new(app.window_rect())) 
        .collect();
```

Drawing our agents is using an iterator:
```rust 
    model.agents.iter().for_each(|agent| {
        agent.display(&draw);
    });
```

Updating the agents also is done using an iterator over mutable references:
```rust 
fn update(_app: &App, model: &mut Model, _update: Update) {
    //We have to use a special iterator, iter_mut, because we mutating the data in each agent
    model.agents.iter_mut()
        .for_each(|agent| agent.update());
}
```

</details>

Running `cargo run --bin part3c` shows us an army of agents flying into the distant right, never to return.

<br/>

---

<br/>

## 🌊 Exercise 3-D: A mind of its own
Let make those agents more individual!

📜 Update the agent's constructor function with an angle argument.
Use the angle to update the agent's position.

<details><summary>💡 Out Agents update function now:</summary>

We also added a `step_size` member to our `Agent` struct.
The update now uses an angle in radians to update the position.
```rust 
    fn update(self: &mut Self) {
        self.vector.x += self.angle.cos() * self.step_size;
        self.vector.y += self.angle.sin() * self.step_size;
    }
```

</details>

<details><summary> 🙈 Agents in random flight directions </summary>
Out new constructing code:

```rust
    //maps the range over a closure function that returns instances of `Agent`
    let agents = (0..agent_count)
        .map(|_| {
            let angle = random_range(0.0, TAU);
            Agent::new(app.window_rect(), angle)
        })
        .collect();
```
We are now controlling the direction using the <code>angle</code> and <code>step_size</code> rather than a <code>Vec2</code>.

</details>
<br/>

---

<br/>

## 🌊 Exercise 3-E: Every time you run away you take a piece of 🍖 with you
Let make sure those agent don't fly away into the oblivion.

📜 Change the agent's update function so that the position doesn't go out of the window.

<details><summary> 🙈 <a href=https://www.youtube.com/watch?v=k35dUj5kG90>Stay foreveeer!</a> </summary>

Using this magic spell we create a trap for our agents: If they escape out of one edge, they reappear on the other side.
You can set the `agent_count` to a lower number to clearly see what happens.
```rust
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
```

</details>
<br/>

---

<br/>

## 🌊 Exercise 3-F: Perlin, Merlin, Schmerlin, the Invisible Hand to guide you
📜 Add Perlin noise to control the angle of the movement.
This one involves some steps, so you can just jump onto the suggestion directly.

<details><summary> 🙈 Adding Perlin noise </summary>

We'll give our `Model` some settings for the noise, and initialize it:
```rust
struct Model {
    agents: Vec<Agent>,
    noise_scale: f64,
    noise_strength: f64,
}
//[...snip...]
    //Initialize the new data fields
    Model{
        agents,
        noise_scale: 300.0,
        noise_strength: 10.0,
    }
//[...snip...]
```
Import the Perlin noise related stuff:
```rust
use nannou::noise::{NoiseFn, Perlin, Seedable};
```
And we add another update function to the Agent:
```rust 
    fn update_angle(&mut self, noise: Perlin, noise_scale: f64, noise_strength: f64) {

        let n = noise.get([
            self.vector.x as f64 / noise_scale,
            self.vector.y as f64 / noise_scale,
            self.noise_z,
        ]) * noise_strength;

        self.angle = n as f32;
    }
```

The Perlin noise instance is created in the `update` function, and our new `update_angle` function is called:
```rust
fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new().set_seed(10);

    model.agents.iter_mut()
        .for_each(|agent| {
            agent.update_angle(noise, model.noise_scale, model.noise_strength);
            agent.update();
        });
}
```

</details>
<br/>

---

<br/>

## 🌊 Exercise 3-G: Linefy it
To better be able to see the "forces" at play here it would be nice to have much much much more agents.
This time we will draw them as lines along the "flow path".

Switch the Agent display function with a line drawing one:
```rust
    fn display(&self, draw: &Draw, stroke_weight: f32, agent_alpha: f32) {
        draw.line()
            .start(self.vector_old)
            .end(self.vector)
            .rgba(0.0, 0.0, 0.0, agent_alpha)
            .stroke_weight(stroke_weight * self.step_size);
    }
```

We'll add the `stroke_weight` and `agent_alpha` data members to the `Model` definition and initialization, and get these in the `view` function.

The `Model`:
```rust 
struct Model {
    agents: Vec<Agent>,
    noise_scale: f64,
    noise_strength: f64,
    agent_alpha: f32,
    stroke_width: f32,
}
```

Out `Agent::display` function gets these:
```rust 
    fn display(&self, draw: &Draw, stroke_weight: f32, agent_alpha: f32) {
        draw.line()
            .start(self.vector_old)
            .end(self.vector)
            .rgba(0.0, 0.0, 0.0, agent_alpha)
            .stroke_weight(stroke_weight * self.step_size);
    }
```

And they used as arguments in the `view` function:

```rust 
fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new().set_seed(10);

    model.agents.iter_mut()
        .for_each(|agent| {
            agent.update_angle(noise, model.noise_scale, model.noise_strength);
            agent.update();
        });
}
```

<br/>

---

<br/>

## 🌊 Exercise 3-H: Get that field moving
Our results so far has a quite static flow field, which causes the particle agents to get stuck in a given path.
The Perlin noise we have is a 3-dimensional one, so we can use the z-dimension to change the field gradually.

📜 Add a `noise_z_velocity` to the `Model`, and use that to change the Perlin noise's 3rd dimension in the `Agent`'s update function.

<details><summary> 🙈 Moving that third dimension </summary>

Add it to our `Model`:
```rust 
struct Model {
    agents: Vec<Agent>,
    noise_scale: f64,
    noise_strength: f64,
    agent_alpha: f32,
    stroke_width: f32,
    noise_z_velocity: f64,
}
```
Init in to `0.01` in our `model` function:
```rust
    Model{
        agents,
        noise_scale: 300.0,
        noise_strength: 10.0,
        agent_alpha: 0.35,
        stroke_width: 0.3,
        noise_z_velocity: 0.01,
    }
```
Add it to the `Agents` update function call:
```rust 
fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new().set_seed(10);

    model.agents.iter_mut()
        .for_each(|agent| {
            agent.update_angle(noise, model.noise_scale, model.noise_strength);
            agent.update(model.noise_z_velocity);
        });
}
```
And change the Agent update function accordingly:
```rust
    fn update(&mut self, noise_z_velocity: f64) {
        self.noise_z += noise_z_velocity;
        //[...snip...]
    }
```
We already have hooked this up in our `update_angle` function:
```rust
    fn update_angle(&mut self, noise: Perlin, noise_scale: f64, noise_strength: f64) {

        let n = noise.get([
            self.vector.x as f64 / noise_scale,
            self.vector.y as f64 / noise_scale,
            self.noise_z, // moving da phase
        ]) * noise_strength;

        self.angle = n as f32;
    }
```

</details>

<br/>

---

<br/>

## 🌊 Exercise 3-I: Increase the fuzzyness
We want a bit more fuzzyness in the particle flow.
We have set them free to flow with the Perlinian forces at play, but still they lack some complexity.

📜 Add a random offset to each `Agent`'s `noize_z`

<details><summary> 🙈 Fuzzying up the flow </summary>

This is our `Agent` constructor now:
```rust 
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
            step_size: random_range(1.0, 5.0),
            win_rect,
            noise_z: random_range(0.0, noise_z),
        }
    }
    //[...snip...]
}
```

</details>

<br/>

---

<br/>

## 🌊 Exercise 3-J: Smooth as silk

To get that real nice flowing feeling we could add thousand upon thousand of additional particles, but our poor CPU will probably start sweating before we get that nice effect going.

A nicer solution is adding an alpha trail to the whole picture, similar to what we did in **Part 2**.

📜 Add an alpha overlay for every frame. The first, and only the first frame, should be colored with a white background. Add the `overlay_alpha` data member to the `Model`.

<details><summary> 🙈 Smoothin' in the cruisin' </summary>

Added the `overlay_alpha` data member to the `Model`:
```rust 
    Model{
        agents,
        noise_scale: 300.0,
        noise_strength: 10.0,
        agent_alpha: 0.35,
        overlay_alpha: 0.03,
        stroke_width: 0.3,
        noise_z_velocity: 0.01,
    }
```
Using this value as the alpha value for our background color:
```rust
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::Delete) {
        draw.background().color(WHITE);
    } else {
        draw.rect()
            .wh(app.window_rect().wh())
            .rgba(1.0, 1.0, 1.0, model.overlay_alpha);
    }

    model.agents.iter().for_each(|agent| {
        agent.display(&draw, model.stroke_width, model.agent_alpha);
    });

    draw.to_frame(app, &frame).unwrap();
}
```
We get the frame index with `frame.nth`, and it that is equal to 0 set wipe the screen with WHITE.
Notice that we also can now use the Delete key to reset the background.

</details>

<br/>

---

<br/>

Fantastic work! You made all the way to the end of the workshop material.

Now the only thing that remains is the future.

There are some things that we haven't implemented from the original example, but we have gotten the basic idea up and running.
Now you can if you like go ahead and modify, experiment and play with the code and see what else you can do with it.

Hope you had fun with the material, and that got inspired to learn more about both Rust and graphics programming.
