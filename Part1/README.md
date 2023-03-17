# 🌐 Part 1: Basic graphics with circles and lines

## Getting up and running
We assume that you already have the Rust toolchain installed and got it working with your basic "Hello, World!" program, by following [these](https://www.rust-lang.org/learn/get-started) instructions.

Just to make sure let's get that "Hello, World!" a spin:

1. Open this repo as a folder in VSCode.
2. Open a terminal window and run this command:
```
cargo run
```

You should see someting like this printed in the terminal window:
```
[eirik@kodeworks rust-nannou-workshop]$ cargo run
   Compiling rust-nannou-workshop v0.1.0 (/home/eirik/git/kodeworks/rust-nannou-workshop)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/rust-nannou-workshop`
Hello, world!
```

Also, make sure that you have the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) installed, which helps you a lot while coding.

(⚠️  There is also deprecated extension called [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) that you **should not install**.⚠️  )

🎉  We have Rust installed and are ready to get those graphics onto the screen!

## Our first Nannou program
The setup of a basic Nannou application involves building an application instance, initializing a model, creating a window, and attaching functions for updating the state and the view.
To make this ordeal a bit easier we have included a basic app at [/examples/basic_app.rs](/examples/basic_app.rs) for you in this repo in the `/examples` directory.

📎 _When writing file paths in these documents, a leading slash means the repo root, i.e. the examples directory is stored on the same level as folders `/Part1`, `/Part2`, etc._

To run this example you run this command in the terminal window:
```
cargo run --example basic_app
```

The program will start compiling, and if you haven't compiled a Nannou program before it is going to download some crates and it may take some time.
But when it is done a window with a cyan background and a pink circle is going to pop up on the screen, hopefully looking something like this:
![el](/images/hello.png)

![el](/images/hello.png){align=center}

<figure>
<div align="center">
<img src="/images/hello.png">
<figcaption style="text-align: center;font-style:italic">The output from running <code>cargo run --example basic_app</code>
</div>
</figure>


If you are seeing a similar image in a window on your computer now, we have Nannou up and running.

🎉  We have compiled our first Nannou app!

We'll start going through the exercises, now that we have set up the things we need to start experimenting.
You can now choose to follow the route step-by-step, jump ahead to whatever you find interesting in the workshop materials, or check out other examples that you find in the [Sources of Inspiration](/texts/sources_of_inspiration.md) document.

## 🌐 Exercises: Part 1
In these exercises we will gradually implement simple version of an computer art work from the 1960s by John Whitney, as seen here:

<p align="center">
<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/jIv-EcX9tUs?start=102" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
</p>

We'll build in byte sized pieces and try to pick up some Rust specific topics along the way.




