# Rusty Christmas Tree

This is code that draws on the LED Christmas Tree made by
[@aidancrowther](https://github.com/aidancrowther/). You can see his 3D design
files and Pi Pico setup code on his project
[OpenPixelTree](https://github.com/aidancrowther/OpenPixelTree).

<p align="center">
    <img src="https://media.discordapp.net/attachments/444005079410802699/923308267143303208/unknown.png" width="500" />
</p>

## Adding your own renderer

If any of this doesn't make sense, write an issue and I'll try to make it more
clear 👍

This should get you started with making something to display on the tree. There
is a Nannou visualizer included so you can see what it will look like before
making a PR. You'll be able to run this with `make visualize` from the root.

To write your own code to run on the tree, you need to implement a "renderer". A
renderer is just a Rust module that implements a draw function, and returns a
`TreeCanvas`. You can find the renderers in [this
folder](https://github.com/AngelOnFira/rusty-christmas-tree/tree/main/tree-writer/src/renderers).
Here is an example of what you'll implement:

```rust
pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    for y in 0..75 {
        for x in 0..20 {
            let this_pixel = Pixel {
                r: (
                    (tick as f64) // Start by converting the tick to a 64 bit float
                    .sin() // The sin will be between -1 and 1
                    .abs() // Get the absolute value so we are between 0 and 1
                    * 150.0 // Multiply by 150 to get a number between 0 and 150
                    + 100.0
                    // ^^ Add 100 to get a number between 100 and 250
                ) as u8, // Convert the float to an 8 bit integer
                g: 0,
                b: 0,
            };
            canvas.set_pixel(x, y, this_pixel)
        }
    }
    canvas
}
```

To add a new renderer, start by duplicating the `template` folder and giving it a new
name. You'll then have to add code in several different places in the project:

- First, add your module
  [here](https://github.com/AngelOnFira/rusty-christmas-tree/blob/main/tree-writer/src/renderers/mod.rs#L5).
- Second, add a new enum variant
  [here](https://github.com/AngelOnFira/rusty-christmas-tree/blob/main/tree-data-schema/src/lib.rs#L7).
- Next, we'll have to add this new variant to several match statements
  [here](https://github.com/AngelOnFira/rusty-christmas-tree/blob/main/tree-data-schema/src/lib.rs#L17),
  [here](https://github.com/AngelOnFira/rusty-christmas-tree/blob/main/tree-writer/src/renderers/mod.rs#L13),
  and
  [here](https://github.com/AngelOnFira/rusty-christmas-tree/blob/main/tree-writer/src/main.rs#L54).
- Finally, set the default vizualizer renderer to your new renderer
  [here](https://github.com/AngelOnFira/rusty-christmas-tree/blob/main/tree-visualizer/src/main.rs#L38).
  
Hopefully I get some time to fix this eventually, but I don't know how right
now. Now, you can start working in the `mod.rs` file in the new renderer folder.

At this point, you should also be able to run the visualizer and see your
renderer in action.

```bash
make visualize
```

Once you have something cool, make a pull request!

<p align="center">
    <img src="images/mario.gif" width="500" />
</p>

### Other Make Commands

```
build: build tree-writer for the Raspberry Pi (requires Docker and cargo-cross)
deploy: scp the binary to the Pi
run: (build + deploy) build tree-writer for the Raspberry Pi and scp the binary to the Pi
visualize: run the Nannou visualizer
setup-web: install the prerequisites for the frontend
frontend: serve the frontend
frontend-release: serve the frontend in release mode
```

## Architecture

### Physical Tree

The physical tree is running on a Raspberry Pi Pico. There are 20 (actually 19)
strips running, creating 19x75 LED grid. You send data to the LED strings using
`spidev`, and send 4500 8-bit numbers for each frame. 20 frames can be drawn per
second. The light indexes on the tree are as follows:

```
3 4 9
2 5 8
1 6 7
```

### tree-writer

This crate is where the different "renderers" are implemented. A renderer is
just a module that implements a draw function, and returns a `TreeCanvas`. In
this function, you can set any `Pixel{r: u8, g: u8, b: u8}` on the `TreeCanvas`.

### tree-visualizer

This crate uses [Nannou] to visualize different `renderers`. It renders at the
same speed (I think:tm:) and orientation that will be displayed on the tree.

### tree-backend

### tree-frontend

### tree-script

This was an attempt to use the Mun scripting language as the backend for drawing
to the canvas. Currently, this isn't working, but if an MVP is made, then it
could be easier to write with, and hot reloadable as well.

[Nannou]: https://github.com/nannou-org/nannou

<p align="center">
    <img src="images/snow.gif" width="500" />
</p>