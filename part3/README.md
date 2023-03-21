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
    vector: Vec2, //The movemement vector, i.e. how fast and in what direction we travel
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

📜 Lets use a looping statement and create multiple agents and store them in a Vec.

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

## 🌊 Exercise 3-E: A mind of its own
Let make those agents more individual!

📜 Update the agent's constructor function with an angle arument.
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
We are now controlling the direction using the _angle_  and step_size rather than a Vec2.

</details>

## 🌊 Exercise 3-E: Every time you run away you take a piece of 🍖 with you
Let make sure those agent don't fly away into the oblivion.

📜 Update the agent's update function so that the position doesn't go out of the window.

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

