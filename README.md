# Rust Drawer

Rust Drawer creates a 1000 × 1000 PNG with a random line, a single point, a rectangle, a triangle, and 49 circles. The rectangle and triangle stay in the same positions on every run. The other shapes are placed randomly, and each draw chooses a random RGB color.

![Example output showing colorful shape outlines on a black background](image.png)

## Run the example

From the project directory, use Cargo with a Rust toolchain that supports the 2024 edition.

```sh
cargo run
```

Open `image.png` in an image viewer. The program writes the file in the current working directory and replaces it on each run. Some circles appear as arcs because their outlines extend beyond the canvas.
