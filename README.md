# Rusty Christmas Tree

This is code that draws on the LED Christmas Tree made by
[@aidancrowther](https://github.com/aidancrowther/). You can see his 3D design
files and Pi Pico setup code on his project
[OpenPixelTree](https://github.com/aidancrowther/OpenPixelTree).

<p align="center">
    <img src="https://media.discordapp.net/attachments/444005079410802699/923308267143303208/unknown.png" width="500" />
</p>

## Adding your own renderer

Write your own code to run on the tree, you need to implement a renderer. You
can find the renderers in [this
folder](https://github.com/AngelOnFira/rusty-christmas-tree/tree/main/tree-writer/src/renderers).
To add a new one, start by duplicating the `template` folder and giving it a new
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
  