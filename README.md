# rtw.tui
[![Crates.io](https://img.shields.io/crates/v/rtw-tui?style=flat-square)](https://crates.io/crates/rtw-tui)
[![Crates.io](https://img.shields.io/crates/d/rtw-tui?style=flat-square)](https://crates.io/crates/rtw-tui)
[![License](https://img.shields.io/badge/license-GPL-blue?style=flat-square)](./LICENSE)
[![Crates.io Size](https://img.shields.io/crates/size/rtw-tui)](https://crates.io/crates/rtw-tui)


This is a terminal user interface (TUI) for my raytracer, based off the book "Ray Tracing in One Weekend" by Peter Shirley.
It allows you to create and render simple scenes in your terminal.
## Features
- Create spheres of any size and position!
- Make your own materials, diffuse, metal, 
glass? we got it all!
- A curated selection of object types (sphere)
- Render your scene in some amount of time. 
- Loads of camera settings.
- Did I mention the spheres?

## Installation
To use rtw.tui, you need to have Rust installed on your system. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

There are two ways to install rtw.tui, crates.io or building from source.

### Crates.io
```bash
cargo install rtw-tui
```
### Building from Source
```bash
# Clone the repository
git clone https://github.com/jamdotjar/rtweekend-tui.git

# Change to the project directory
cd rtweekend-tui

# Install dependencies
cargo build
# do something with the binrary ig, or just 
cargo run
```

## Usage
How to use your project.
to open the TUI run the following command
```bash
rtwtui
```
This will open the main page, were you can create objects, materials, and render your scene. For each page, I will list the keybindings and what they do.
> ***IMPORTANT***: If you try to submit anything (render, material, object) with invalid inputs, nothing will hapen. You will have to fix the inputs before you can submit ( or cancel with `Esc` )
> ***ALSO IMPORTANT***: If your renders take an abnormally long time and result in a black screen, your camera is probably inside an object. This can ususally be fixed by just moving the camera back a bit more
**Main Page**
- `↑`/`↓` - Scroll object list
- `n` - Create a new object
- `m` - Create a new material
- `r` - Render the scene
- `q` - Quit

**Object Editor**
- `Tab`/`Shift+Tab` - Change inputs
- `←`/`→` - Change inputs
- `Type` - Input values
- `↑`/`↓` - Choose Material
- `Enter` - Save
- `Esc` - Cancel

**Material Editor**
- `Tab`/`Shift+Tab` - Change inputs
- `←`/`→` - Change inputs
- `Type` - Input color
- `↑`/`↓` - Cycle through material types
- `Enter` - Save
- `Esc` - Cancel

**Render Settings**
- `Tab`/`Shift+Tab` - Change inputs
- `←`/`→` - Change inputs
- `Type` - Input values
- `Enter` - Render scene (this might take a bit)
- `Esc` - Close

## Examples
Here's a a sphere!
![diffuse](https://github.com/user-attachments/assets/2d27cc85-140d-4c0a-9a8c-8ceae7918816)
here are the settings for this scene.
```
Sphere 1: radius 0.5, (1, 0, 0)
Sphere 2: radius 100.0, (1, -100.5, 0)

Camera: (-1, 0, 0), lookat(1, 0, 0), fov: 45
```
Here's a snowman I made with this:
![snowman](https://github.com/user-attachments/assets/90d3572f-5801-4713-9f73-87bfe9f4441d)
*I challenge you to try to make a snowman*
## Misc Info & Tips
- Y will always be "UP" in renders
- Lookat is the only way to set camera rotation, just coose a location and the camera will automatically rotate to face it.
- The more objects you add, the slower renders will be, so dont add 20 spheres and expect it to be fast.
- You don't need a lot of bounces for a good quality on most scenes (5-10 works fine on basic diffuse-only, for metal/glass feel free to bring it up a bit)
- Samples affect render time significantly more than bounces, but you should still have a fairly high sample count to avoid noise. For a clean render, anything above 100-150 is usually good.
- Images will be created whereever you run the tool, so if you want all your images in one folder run the tool from there. 
> this app creates portable pixelmap files (.ppm). These are not widely used, and not ideal for sharing due to their lack of compression. I'd suggest converting them to png or jpg if you want to store them longterm, as otherwise they can be space hogs.
> Here is a list of programs that could be used to view/convert PPM files:
> - GIMP (Mac/Windows/Linux) *might be a bit overkill*
> - Preview (Mac - builtin)
> - ImageMagick (Mac/Windows/Linux) *conversion only*
> - IrfanView (Windows)
> - [this website](https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html)
>
> This list isnt exhaustive, so check if your normal image viewer can be used first.

## Gallery
<img width="1470" alt="image" src="https://github.com/user-attachments/assets/d3e43ef9-998a-4a9f-aa35-caec97626f3a" />
<img width="1470" alt="image" src="https://github.com/user-attachments/assets/f3314d05-0579-48b4-afa1-294b05446b80" />
<img width="1468" alt="image" src="https://github.com/user-attachments/assets/4995b680-8460-49b1-a3a2-0a84ef86572b" />
<img width="1470" alt="image" src="https://github.com/user-attachments/assets/9102b2ae-b899-4bc4-8895-13407e242450" />
<img width="1470" alt="image" src="https://github.com/user-attachments/assets/31ead29d-a94e-4aaa-92a6-34983d6c6887" />


## License
This project is licensed under the GPL-3.0 License.
