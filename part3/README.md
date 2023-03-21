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
