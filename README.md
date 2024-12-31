# rtw.tui

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
cargo install rtw.tui
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
Here's a snowman I made with this:
![snowman](https://github.com/user-attachments/assets/90d3572f-5801-4713-9f73-87bfe9f4441d)
and a sphere!
![diffuse](https://github.com/user-attachments/assets/2d27cc85-140d-4c0a-9a8c-8ceae7918816)

## License
This project is licensed under the GPL-3.0 License.
