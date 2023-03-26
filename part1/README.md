# 🌐 Part 1: Basic graphics with circles and lines

## Getting up and running
We assume that you already have the Rust toolchain installed and got it working with your basic "Hello, World!" program, by following [these](https://www.rust-lang.org/learn/get-started) instructions.

Just to make sure let's take that "Hello, World!" a spin again:

1. Open this repo as a folder in VSCode.
2. Open a terminal window and run this command:
```
cargo run --bin hello
```

You should see something like this printed in the terminal window:
```
[eirik@kodeworks rust-nannou-workshop]$ cargo run --bin hello
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/hello`
Hello, World!
```

Also, make sure that you have the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) installed, which helps you a lot while coding.

>⚠️  There is also a deprecated extension called [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) that you **should not install**.⚠️

🎉  We have Rust installed and are ready to get those graphics onto the screen!

## Our first Nannou program
There are two types of Nannou setups: _sketches_ and _apps_.

* _sketches_ is the simplest to set up and are mostly used for quick or simple experiments, where you don't need much state management, interaction with MIDI, audio etc.
* _apps_ is what you would use for a more full fledged application. This is the type of setup we will use for this workshop.

If you want more explanations of the differences between _apps_ and _sketches_ you can read the Nannou guide chapter [Basics - Sketch vs App](https://guide.nannou.cc/tutorials/basics/sketch-vs-app.html).

The setup of a basic Nannou application involves building an application instance, initializing a model, creating a window, and attaching functions for updating the state and the view.
To make this ordeal a bit easier we have included a basic app at [/basic_app/src/main.rs](/basic_app/src/main.rs) for you in this repo in the `/basic_app` directory.

📎 _When writing file paths in these documents, a leading slash means the repo root, i.e. the `/basic_app` directory is stored on the same level as folders `/part1`, `/part2`, etc._

To run this basic app you can run this command in the terminal window:
```
cargo run --bin basic_app
```

The program will start compiling, and if you haven't compiled a Nannou program before it is going to download some crates and it may take some time.
But when it is done a window with a cyan background and a pink circle is going to pop up on the screen, hopefully looking something like this:

![](/images/hello.png)
<br/>
_The output from running_ <code>cargo run --bin basic_app</code>


If you are seeing a similar image in a window on your computer now, we have Nannou up and running.

🎉  We have compiled our first Nannou app!

---

We'll start going through the exercises now that we have set up the things we need to start experimenting.
If you want to know more about how a basic Nannou app is structured you can open the [/basic_app/src/main.rs](/basic_app/src/main.rs) file and read through the comments in that file.
It will give some insights into what goes where, and some tips about the syntax and structure.
You can now choose to follow the route step-by-step, jump ahead to whatever you find interesting in the workshop materials, or check out other examples that you find in the [Sources of Inspiration](/texts/sources_of_inspiration.md) document.

---

## 🌐 Exercises: Part 1
In these exercises we will gradually implement simple version of an computer art work from the 1960s by John Whitney, as seen in [this Youtube-video](https://www.youtube.com/watch?v=jIv-EcX9tUs)

We'll build it in byte sized pieces and pick up some Rust specific topics along the way.
If you find any of the exercises difficult, feel free to just look at the suggestion and move along.
You can always come back to the material at a later time if you feel like it.

We start with a basic app setup for a Nannou program.
In the exercises for Part 1, we will work with the file `part1/src/main.rs`.
This file is the same file as the `basic_app` we just tested, but without all the comments.
We have also removed the line where the windows size was set, so that we now get the default window size.
You can open this file in your editor and make the changes to that file as we go along.
To run your code you can run this command the repo root in a terminal window:
```
cargo run --bin part1
```

For each exercise there is a corresponding suggestion for solution in the `part1/src/suggestions` directory.
The suggestions for exercises are named after the exercise name.
So for exercise 1-A, the suggestion is found in the file `part1/src/suggestion/part1a.rs`.
To run the suggestion file you can run this in the terminal:
```
cargo run --bin part1a
```

Exercises 1.A to 1.I are based on basic knowledge of how to move objects to different positions on the window, set the colors, translating the drawing context, and how to rotate objects using The Sons of Trigonomic Thrønder, `sin()` and `cos()`.
If this is all too familiar to you, you can take the Rocket Option™, and land on Exercise 1.J, where we will start actually building the Whitney piece.

### Learning goals
After the exercises in Part 1 you will have a basic understanding of the following:
* 🎯 Minimal setup for Nannou app.
* 🎯 Draw a colored background
* 🎯 Draw ellipses, rectangles, triangles, and lines.
* 🎯 Specify and control the positions and colors for the shapes.
* 🎯 Understand the coordinate system for nannou; how position values correspond to location on the windows.
* 🎯 Specify relative positions for groups of shapes.
* 🎯 Animate parameters for the shapes relative to time.

<br/>

---

<br/>

### 🌐 Exercise 1-A: Change the colors 
We are going to set different colors for our background and our circle.
In Nannou there are multiple ways to define colors.
The easiest method is to use [_color constants_](/texts/cheat-sheet.md#color-constants)

📎 _The color constants link above points to the Cheat Sheet in [`texts/cheat-sheet.md`](/texts/cheat-sheet.md) This document is a selection of tips, snippets, and info that can be helpful during the workshop._

📜 Define one variable for the circle color and use that to set the color of the ellipse.In the suggestion files we will continue to use cyan for our background, and magenta for the ellipse, but you can choose any colors you like.

<details><summary>💡 This is how you define a variable in Rust:</summary>

```rust
let x = 42; //a immutable variable containing a
```

</details>

<br/>
<details><summary>🙈 Spoiler alert! Here is one way to do it:</summary>

```rust
fn view(app: &App, _model: &Model, frame: Frame){
    //[...snip...]
    draw.background()
        .color(CYAN);

    let circle_color = MAGENTA;
    draw.ellipse()
        .color(circle_color);
    //[...snip...]
}
```

📎 _The suggestions for solutions will only show the relevant changes to the files. Surrounding areas of the changes made are usually _snipped out_, which you can see with <code>//[...snip...]</code> comments. Often but not always we will include the function signature for the areas that were changed. To see the whole suggestions you can open the corresponding file from its `suggestion` directory._
</details>
<br/>

🎉  You have changed the color of a graphic object in Nannou!

---
<br/>

### 🌐 Exercise 1-B: 📜 Make the circle move up and down
You can decide how high and low the ball will move.
<details> <summary>💡 Tip: You can use the app time to move the circle. </summary>

```rust
app.time;       //Gives the elapsed time in seconds
app.time.sin();         // You can use the sine function to generate a looping value
                        //  between -1.0 and 1.0.
app.time.sin() * 100.0; //Multiplying the output from sin() maps the range 
                        // -1.0 - 1.0 to -100.0 - 100.0
```

</details>
<br/>

<details> <summary>💡 Tip: Methods for setting positions of drawings </summary>
The type that is returned from the <code>ellipse()</code> methods is an instance of <code>Drawing</code>.
In the type-hint that <code>rust-analyzer</code> gives it says that it is a <code>Drawing<Ellipse></code> i.e. it is a drawing-in-progress of a geometric _primitive_, specifically an ellipse in our case.

<br/>

📎 _You can of course use Nannou to draw other things than ellipses. In the cheat sheet is a [list of other primitives](/texts/cheat-sheet.md#drawing-primitive-types)_

```rust
draw.ellipse()
    .y(-100.0); // Set the Y coordinate to 100 pixels below the center of the screen.
```


</details>
<br/>


<details> <summary>💡 Tip: About screen coordinates </summary>
In Nannou, the window coordinates has origo, i.e. point (x: 0.0 y: 0.0), at the center of the window.
If a window has size 1280x720, the right edge has X coordinate at 640 and Y coordinate at -640.
</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
    draw.ellipse()
        .y(app.time.sin() * 200.0 )
        .color(circle_color);
```

</details>
<br/>
🎉  You have animated the position a circle in Nannou, in relation to time!

<br/>

---

<br/>

### 🌐 Exercise 1.C - Change the circle movement
📜 Make the circle move all the way from the top of the screen to the bottom.
<details> <summary>💡 Tip: You can use the data from the <code>App</code> instance to get the screen height. </summary>

```rust
let win = app.window_rect();
win.w(); //total width
win.h(); //total height
win.top(); //top of the window
win.botton(); //bottom of the window
win.right(); //right edge of the window
win.left(); //left edge of the window
```

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
fn view(app: &App, _model: &Model, frame: Frame){

    //[...snip...]

    let win = app.window_rect();
    //The output range from the `sin()` function is -1.0 - 1.0
    //Since the window coordinates for nannou has x:0.0,y:0.0 as the center of the window,
    // converting the range -1.0 to 1.0 to the full height of the window is as simple as
    // multiplying with half the window height.
    let y_pos = app.time.sin() * win.h() * 0.5;
    //Use the draw instance to draw an ellipse.
    draw.ellipse()
        .y(y_pos)
        .color(circle_color);

    //[...snip...]
}
```

</details>

<br/>

🎉  Your circle is now swinging from the bottom to the top of the window.

---

<br/>

### 🌐 Exercise 1.D - A smaller and more precisely positioned movement
📜 Convert the movement range to having the top at _2:3_ of the window height, and the bottom at _1:4_ of the screen height.

<details><summary>💡 There is a Nannou function that makes it really easy to convert from one number range to another: </summary>

```rust
let a = 0.5;
let b = map_range(a, 0.0, 1.0, 10.0, 20.0); // => 15.0
```

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
//Use the top and bottom values from the window rect to make it simpler to convert value
// ranges.
const TOP_RATIO: f32 = 2.0/3.0; // The ratio need only be calculate once, so we use const.
const BOTTOM_RATIO: f32 = 1.0/4.0; // If we had used `let top_ratio = 2.0/3.0` the Rust
                                   // compiler probably would have optimized this into a
                                   // constant anyway. The default immutability of variables in
                                   // Rust makes many it easier for the compiler to figure out
                                   // ...But hey, can't a programmer have a bit of fun with 
                                   //  unnecessary optimization from time to time..?
let y_top = map_range(TOP_RATIO, 0.0, 1.0, win.bottom(), win.top());
let y_bottom = map_range(BOTTOM_RATIO, 0.0, 1.0, win.bottom(), win.top());
let y_pos = map_range(app.time.sin(), -1.0, 1.0, y_bottom, y_top);
```

</details>

<br/>
🎉  Your now have ninja-level precision over the circle's vertical position.

🎉  You can map numbers from one number range to another, using the `map_range` function.

--- 

<br/>

### 🌐 Exercise 1.E - Full circle rotation
📜 Change the movement of the circle to do move in a circle around the center.
The circle should go to the edges of the window both on the left/right edges and the top/bottom edges.

<details><summary>💡 To travel in a circular motion we can combine <code>sin()</code> with its co(s)mpanion... They have a complex relationship. </summary>

```rust
//We can combine `sin` and `cos` to create movement in circles.
//This will rotate the circle in an even circle around origo, with a distance of 200.0 pixels.
draw.ellipse()
    .y(app.time.sin() * 200.0)
    .x(app.time.cos() * 200.0)
    .color(MAGENTA);
```


</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
let win = app.window_rect();

let y_pos = map_range(app.time.sin(), -1.0, 1.0, win.bottom(), win.top());
let x_pos = map_range(app.time.cos(), -1.0, 1.0, win.left(), win.right());

draw.ellipse()
    .y(y_pos)
    .x(x_pos)
    .color(circle_color);
```

</details>

<br/>
🎉  You can now rotate things in Nannou! That will sure come in handy later.

---

<br/>

### 🌐 Exercise 1.F - Double circle rotation
📜 Add another circle to the drawings.
The new circle should be half the size and travel twice as fast, in the opposite direction.
Choose another nice color for the second circle.
Both circles should touch the edge of the screen, i.e. nothing of the circles should go outside the window.

💡 You can find function for setting circle/ellipse properties, and much more, in the [cheat sheet](/texts/cheat-sheet.md#draw-circles-and-ellipses).

<details><summary>💡 What do you have to change for time to travel faster?...</summary>

The `app.time` part is what's "moving the time", before it is evaluated as argument for the `sin()` function.
Hence, if we multiply the time with 2.0, we travel twice as fast... through time.

</details>
<br/>

<details><summary>💡 What do you have to change to change direction of rotation?...</summary>

You can negate either one of the `sin()` or `cos()` parts to change the direction.

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
    let circle_radius_a = 50.0;
    let pos_a = vec2(
        map_range(app.time.sin(), -1.0, 1.0, 
                  win.left() + circle_radius_a,
                  win.right() - circle_radius_a),
        map_range(app.time.cos(), -1.0, 1.0, 
                  win.bottom() + circle_radius_a, 
                  win.top() - circle_radius_a));

    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        //Multiply the time before calling the sin() function doubles the frequency.
        map_range((2.0 * app.time).sin(), -1.0, 1.0, 
                  win.left() + circle_radius_b,
                  win.right() - circle_radius_b ),
        //Negating either the sin() or cos() part to make it travel in opposite direction
        map_range(-(2.0 * app.time).cos(), -1.0, 1.0, 
                  win.bottom() + circle_radius_b, 
                  win.top() - circle_radius_b ));

    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);
    draw.ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);
```

</details>
<br/>
🎉  You can now rotate multiple things in relation to the window dimensions!

---

<br/>

### 🌐 Exercise 1.G - Planet and a moon
We have two circles.

📜 In this exercise we will change the movement of circles `circle_a` and `circle_b` like this:

- Circle `circle_a` will move back and forth along the x axis, at the vertical center of the window.
  - This means that `circle_a` will go from the left to the right, while the y coordinate is 0.0.
- Circle `circle_b` will circle around `circle_a`

<details><summary>💡 We don't have to calculate that much. It is actually much easier to change the position of the drawing context itself: </summary>
We have already seen that we can change the position of an ellipse using the <code>xy()</code> function.
Well, the same type of function can be used for the <code>draw</code> instance as well:

```rust
let draw = draw.xy(vec2(-100.0, 20.0)); // set the position using a Vec2
let draw = draw.x_y(-100.0, 20.0); // set the position using x and y as separate values
```

More info on this in the [cheat sheet](/texts/cheat-sheet.md#move-scale-and-rotate-the-position-of-the-drawing-context)

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
    let circle_radius_a = 50.0;
    let pos_a = vec2( (app.time * 0.5).sin() * win.w() * 0.25, 0.0);

    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);

    //Instead of moving the next circle, we move the whole draw instance to a new position.
    //This effectively moves the center for the drawing to the position of circle `a`, thus drawing
    // circle `b` in relation to that point becomes much easier.
    let draw = draw.xy(pos_a);

    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        (3.0 * app.time).sin() * 100.0,
        (3.0 * app.time).cos() * 100.0,
        );

    draw.ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);
```

</details>

<br/>
🎉 Good job! You can now rotate a thing relative to another things' position.

---

<br/>

### 🌐 Exercise 1.H - A building where the wild pixels live

Wouldn't it be cool if we had a small skyscraper on the ground? 🤔

📜 Let's add a building with 4 floors, two width units on each floor.
Start by making a rectangle for the building side.
A simple rectangle should be good.
How do we draw rectangles in Nannou you say?

<details><summary>💡 Let's see if the docs can help us...  </summary>
⚠️  🤓 Warning: Longer text about reading the Rust docfiles <a href=/texts/reading-docfiles.md>here</a>⚠️

<br/>

This text goes into detail on how you can navigate the documentation for nannou to find out what methods are defined for the different Primitives in Nannou.
This may be more interesting for you if you are eager to learn more about concepts in the Rust language and the documentation system.
If that is not interesting for you, feel free to jump ahead for the next tip that uncovers the magical methods for drawing rectangles.

</details>
<br/>

<details><summary>💡 How to draw a rectangle  </summary>
In the <code>Drawing</code> module we have the method <code>rect()</code> that returns a drawing-in-progress-of-a-rectangle, or <code>Drawing&ltRect&gt</code> as it called in Rust-speak.
The <code>Drawing&ltRect&gt</code> support a lot of interesting methods, so here is a short selection.

📎 _If you want to dig deeper into how to find these methods in the documentation and source yourself, read the text linked to in the previous tip if you haven't already done so. There is also some info about this listed in the [Cheat Sheet](/texts/cheat-sheet.md#shapes)_

```rust
.xy(); // set position using a Vec2
.x_y(); // set position using x and y as separate arguments
.wh(); // set the width and height using a Vec2
.w_h(); // set the width and height as separate argument
.rotate(); // rotate by radians
.rgb(); // color in RGB
.no_fill(); // don't fill the rectangle with a color
.stroke_color(); // set the color for the stroke
```

</details>
<br/>

Let's place the building rectangle at the center bottom of the window.

<details><summary>🙈 Here is one way to do it:  </summary>

```rust
//Draw the basic structure of the building
let unit_size = 50.0;
draw.rect()
    .x_y(0.0, win.bottom())
    .height(unit_size * 4.0) //4 floors
    .width(unit_size * 2.0) // 2 windows per floor
    .color(GREY);
```

</details>
<br/>
🎉 Great stuff, you've built a building, using pixels!

---

<br/>

##### 🪲 BUG ALERT! 🪲
if you used the suggestions so far, running the code above gives a strange result: The building moves along with `circle_a`.
A moving building...clearly that's not what we wanted.

📜 Can you figure out why this happened?

<details><summary>💡 Here is a hint about what going on:  </summary>

What happened to the drawing context when we changed its position using `.xy(pos_a)`?

Have a look at the method signature:

```rust
pub fn xy(&self, v: Vec2) -> Self
```

and the call site:

```rust
let draw = draw.xy(pos_a);
```

</details>
<br/>


<details><summary>💡 Here is another hint on how to fix it: </summary>
The result of <code>draw.xy(pos_a)</code> is assigned to our <code>draw</code> variable, which caused lasting change to the position of the drawing context.

We need to make this temporary.

</details>
<br/>


<details><summary>🙈  Here is a way to fix it: </summary>

```rust
pub fn xy(&self, v: Vec2) -> Self
```

The method signature for `.xy()` tells us that we call the object as a read-only _reference_, because of the first `&self` parameter.
This means that we cannot change the self object that the method is called on, but the `.xy()` method returns a new version with a new drawing context centered around `pos_a` in our case.
The `-> Self` part tells you that what gets returned from the function is an _owned type_, it has no `&`.
You can safely assume that a new version of the `Draw` type is returned, since there is no way for an owned instance of the original `Draw` instance to enter the function.
When we use this new version of `Draw` to assign to the `draw` variable, the old `draw` instance is replaced.

```rust
let draw = draw.xy(pos_a);
```

If we don't want to replace the existing `Draw` instance we can temporarily move the center to another position by calling the `.xy()` method inline, right before we start drawing our rectangle:

```rust
    draw.xy(pos_a) // move the draw context to a different position temporarily
        .ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);

    //Draw the basic structure of the building
    let unit_size = 50.0;
    draw.rect()
        .x_y(0.0, win.bottom()) //This time the origin is in the center of the window
        .height(unit_size * 4.0) //4 floors
        .width(unit_size * 2.0) // 2 windows per floor
        .color(GREY);

    //Just a triangle to check that it draw smack in the center, i.e. it hasn't been
    // affected by changing the position of the drawing context in the rect drawing 
    // section above.
    draw.tri()
        .x_y(0.0, 0.0)
        .color(BLUE);
```

</details>
<br/>
🎉 Congratulations! You have fixed a very nasty building-moving bug.

🎉 You understand the concept of moving the position of the `Draw` context both temporarily and ...not-so-temporarily.

---

<br/>

##### 🪳 BUG ALERT!! 🪳
📜 We have another bug in our code! 😱
You may have noticed that half of the building is outside the window.
This is because the position of the rectangle is defined in relation to its center.
So we could offset the position of the rectangle by half the height of the rectangle, but there is another more expressive way to place the rectangle: Using the [`nannou::geom::Rect`](https://docs.rs/nannou/0.18.1/nannou/geom/struct.Rect.html) module.
This module offers a lot of methods relating to describing the geometrical properties of a rectangle.


<details><summary>💡 But didn't we just use the <code>rect</code> thing for just drawing? surely, the <code>rect</code> thing can't do a lot of other fancy geometrical tricks as well...? : </summary>

In programming, a general rule is that you try to keep your abstraction as well defined according to a given purpose as possible.
It would not be a good design if we designed a rectangle abstraction that defines all its geometrical properties as well as how it is going to draw itself on the screen, and send an email to your followers on social media while doing so.
That would be to break the [single-responsibility principle](https://en.wikipedia.org/wiki/single-responsibility_principle).

You can rest assured (but possibly confused) that the <code>rect</code> we use for drawing and <code>rect</code> used for geometrical properties are not the same.
They live in different places in the _namespace_ of nannou.

- The drawing type is `nannou::draw::primitive::rect::Rect`
- The geometric type is `nannou::geom::Rect`

The nodes in the namespace are separated by double colons, `::`.

</details>
<br/>


<details><summary>💡 Placing a <code>nannou::geom::Rect</code> </summary>
Here is a selection of <a href=https://docs.rs/nannou/0.18.1/nannou/geom/struct.Rect.html><code>nannou::geom::Rect</code></a> methods:

```rust
use nannou::geom::Rect; //Bring it into current namespace so that only need `Rect`
let r = Rect::from_x_y_w_h(10.0, -100.0, 100.0, 100.0);
let r2 = Rect::from_w_h(50.0, 50.0).mid_top_of(r); // place second rect on the middle of the top of r
r2.wh(); //get the width and height as a Vec2
r2.xy(); //get the position as a Vec2
```

</details>
<br/>


<details><summary>🙈  Here is a way to fix the second bug, using <code>nannou::geom::Rect</code> </summary>

```rust
//Draw the basic structure of the building
let unit_size = 50.0;
let num_floors = 4;
let windows_per_floor = 2;
//Convert the i32 primitive type to f32 using the
// `as` keyword. This way of converting types is only
// applicable for primitive types such as f32, u32
// etc.
let building_rect = Rect::from_w_h(
    unit_size * windows_per_floor as f32, 
    unit_size * num_floors as f32
    )
    .mid_bottom_of(win);

draw.rect()
    .xy(building_rect.xy()) // use the rect properties directly on the drawing
    .wh(building_rect.wh())
    .color(GREY);
```

</details>
<br/>
🎉 You have more insight into placing rectangles, using the <code>nannou::geom::Rect</code> API.

🎉 You have seen `Rect`'s from different namespaces doing what each of them do best.

🎉 You have seen conversion between primitive values using the `as` keyword.

🎉 You are becoming a danger to those poor building-placement bugs!

---

<br/>

Ok, so we have come a good way along now.
But what do all these circles and rectangles have to do with the John Whitney piece we are aiming to implement?

Fear not, because we are now going to start getting those dancin' lines onto the window.💃🕺

<br/>

---


### 🌐 Exercise 1.I - Oscillators
The code we have used for changing a scalar value in relation to the application time is the following:

```rust
(app.time * 2.0 + (0.5 * PI)).sin();
```

This expression has started to become used repeatedly around our code, and we get a feeling that it could be nice to create an abstraction for this.
It would be an advantage to encapsulate this idea in a separate function that abstracts the idea of a sine oscillator more clearly, with the added advantage of being easier to reuse and to combine with other oscillators.

We have made some oscillator abstractions for controlling values for you to use in the following exercises.
These are found in the [`common/src/lib.rs`](/common/src/lib.rs) file.

You can see different oscillators in action by running the [`common/src/osc_functions.rs`](/common/src/osc_functions.rs) file with this command:

```
cargo run --bin osc_functions
```

📜 Using the file [`/part1/src/whitney.rs`](/part1/src/whitney.rs) as a starting point, make the ellipse travel in circles using the `Osc::sin_cos` function.
Also, add a line between the two ellipses. Another thing you can try is to give the two oscillator slightly different frequencies, so that go in and out of phase with eachother.

You run the code with the command: 
```rust
cargo run --bin whitney
```

<details><summary>💡 Importing a module </summary>
We are going to use the <code>Osc</code> struct from the <code>common</code> module in our workspace.
To import the <code>common</code> crate in out workbench you use the <code>use</code> keyword:

```rust
use common::Osc;
```

We can now use the `Osc` functions with e.g. `Osc::sin`.
</details>
<br/>


<details><summary>💡 Rotating oscillator </summary>
As we know from earlier, the combination of <code>sin</code> and <code>cos</code> gives us a circular movement.
The <code>Osc</code> struct has a method called <code>sin_cos</code> that returns a <code>Vec2</code> which is nice, because you can just put the returned value into any <code>.xy()</code> method.

```rust
.xy(Osc::sin_cos(t, 0.25, 0.0) * 100.0) // multiplying the whole vector with 100.0 makes it move
                                        // in a radius of 100.0 around its origo
```

We can now use the <code>Osc</code> functions with e.g. <code>Osc::sin</code>.
</details>
<br/>

<details><summary>💡 Translating a circular movement </summary>
When you use the <code>+</code> operator on a <code>Vec2</code> you are actually moving the center position of the circular movement.

```rust
//Adding a `Vec2{x: 100.0, y: -100.0}` means shifting the center of rotation 100.0 pixels right
// and 100 pixels down.
.xy(Osc::sin_cos(t, 0.25, 0.0) * 100.0 + vec2(100.0, -100.0))
```

</details>
<br/>

<details><summary>🙈  Two ellipses moving in circles with a connecting line </summary>

```rust
    let a = Osc::sin_cos(t, 0.25, 0.0) * 100.0 + vec2(0.0, win.top() * 0.5);
    let b = Osc::sin_cos(t, 0.3, 0.0) * 100.0 + vec2(0.0, win.bottom() * 0.5);

    draw.ellipse()
        .xy(a)
        .radius(10.0)
        .color(PINK);

    draw.ellipse()
        .xy(b)
        .radius(10.0)
        .color(PINK);

    draw.line()
        .points(a, b)
        .stroke_weight(1.5)
        .color(PINK);
```

You can run the suggestion with:
```
cargo run --bin part1i
```

</details>
<br/>

🎉 You have imported a module to your code!

🎉 You have used a complex sine oscillator to make a circular movement!

<br/>

---

<br/>

### 🌐 Exercise 1.J - Going parametric

In the `Osc` struct we have the `parametric` oscillator that can create more complex movements that can have a bit more variation.
Let's us this to control the movement of the points:

```rust
    let aa = vec2(
        Osc::parametric(t, 0.25, 0.0, 1.02, 0.5),
        Osc::parametric(t, 0.3, Osc::PHASE90, 1.87, 0.3),
        );
    let bb = vec2(
        Osc::parametric(t, 0.35, 0.0, 1.32, 0.9),
        Osc::parametric(t, 0.5, Osc::PHASE90, 0.8, 0.1),
        );
    let a = aa * 300.0;
    let b = bb * 300.0;
```

📜 Experiment with the settings for the parametric oscillators to se what makes for an interesting movement.
If you feel adventurous you can try to mix in, replace with, any combination of the functions from the `Osc` module.

<details><summary>🙈  The correct answer... </summary>
...is whatever you find the most interesting.
</details>
<br/>

🎉 You are making digital art!
<br/>

---

<br/>

### 🌐 Exercise 1.K - Tails of lines
In the Whitney piece we see that there are multiple lines following the same path.

Can you think of a way to get this effect in our code?

📜 Make a tail of for examples 10 lines, that follow after the first line.

<details><summary>💡 Making multiple instances </summary>
A good ole' <code>for</code> loop could do the trick.
The simplest way to loop over values in Rust is this:

```rust
for i in 0..10 {
  println!("index: {}", i);
}
```
You can use the index value to offset the time argument for the `Osc::parametric` function.
</details>
<br/>

<details><summary>🙈  Iteratin'... u hatin' </summary>

```rust 
    let speed = 0.5; // Added a main speed control variable
    for i in 0..10 { // 0..10 is the notation for a range in Rust 
        let tail_length = 0.1;
        let tt = t + (i as f32 * tail_length); // offset the time with the scaled index value
        let aa = vec2(
            Osc::parametric(tt, 0.25 * speed, 0.0, 1.02, 0.5),
            Osc::parametric(tt, 0.3 * speed, Osc::PHASE90, 1.87, 0.3),
            );
        let bb = vec2(
            Osc::parametric(tt, 0.35 * speed, 0.0, 1.32, 0.9),
            Osc::parametric(tt, 0.5 * speed, Osc::PHASE90, 0.8, 0.1),
            );
        let a = aa * 300.0;
        let b = bb * 300.0;

        draw.line()
            .points(a, b)
            .stroke_weight(1.5)
            .color(PINK);
    }
```


</details>
<br/>

🎉 You are using a for-loop to create multiple lines that move along the same path!

<br/>

---

<br/>

### 🌐 Exercise 1.L - Functions
In the trend of making multiple instances, we would like to have two trails of moving lines.

📜 Modify the code so that we can draw two different trails of lines.
Make the line trails have different movement paths and color.

<details><summary>💡 Before we fire up the copy-pastamachine..🏜  Let's try to keep it <a href=https://en.wikipedia.org/wiki/Don%27t_repeat_yourself>DRY </a></summary>

This is a good chance to make the drawing of line trails into a separate function, so that we can call it with varying arguments.

May I suggest a function signature for you:
```rust
fn draw_line_trail(time: f32, draw: &Draw, num_lines: i32, speed: f32, tail_length: f32, color: Srgb)  {...}
```

You can ...try to call it with this code (and see what happens...):
```rust
    draw_line_trail(t, &draw, 10, 0.5, 0.1, PINK);
```

</details>
<br/>

<details><summary>💡 Is the computer saying no to your color <code>PINK</code>?</summary>
The Rust compiler is as may have heard, quite strict.
The color the <code>Srgb</code> type expects is based on <code>f32</code> types, whereas the type that is returned from a color constant such as <code>PINK</code> is <code>Rgb<Srgb, u8></code>.
The <code>u8</code> is the relevant part of this.
But it is easy to convert between color type and color spaces in Nannou:

```rust
    //We have to call .into_format() on the color constant to convert it into a f32 based color.
    draw_line_trail(t, &draw, 10, 0.5, 0.1, PINK.into_format());
```

For more info about colors in Nannou there is stuff to read in the [cheat sheet](/texts/cheat-sheet.md#colors).

</details>
<br/>

<details><summary>🙈  A suggestion for function and calling code. </summary>
The function can be something like this:

```rust
fn draw_line_trail(time: f32, draw: &Draw, num_lines: i32, speed: f32, tail_length: f32, color: Srgb)  {
    for i in 0..num_lines { // 0..10 is the notation for a range in Rust 
        let tt = time + (i as f32 * tail_length); // offset the time with the scaled index value
        let aa = vec2(
            Osc::parametric(tt, 0.25 * speed, 0.0, 1.02, 0.5),
            Osc::parametric(tt, 0.3 * speed, Osc::PHASE90, 1.87, 0.3),
            );
        let bb = vec2(
            Osc::parametric(tt, 0.35 * speed, 0.0, 1.32, 0.9),
            Osc::parametric(tt, 0.5 * speed, Osc::PHASE90, 0.8, 0.1),
            );
        let a = aa * 300.0;
        let b = bb * 300.0;

        draw.line()
            .points(a, b)
            .stroke_weight(1.5)
            .color(color);
    }
}
```
And you could call it like this:
```rust
    //We have to call .into_format() on the color constant to convert it into a f32 based color.
    draw_line_trail(t, &draw, 10, 0.5, 0.1, PINK.into_format());
    draw_line_trail(t + 100.0, &draw, 13, 0.4, 0.15, rgb(0.1, 0.1, 0.9));
```

</details>
<br/>

🎉 You are using your own specially made function to draw a trailing lines duet!

<br/>
    
---
    
<br/>
Good job! Take a break and a snack, you deserve it.
    
Next up: [**Part 2: Veiled kaleidoscope**](/part2/README.md) 🚂
