# Rust Drawer

Rust Drawer creates a 1000 × 1000 PNG with a random line, a single point, a rectangle, a triangle, and 49 circles. The rectangle and triangle stay in the same positions on every run. The other shapes are placed randomly, and each draw chooses a random RGB color.

![Example output showing colorful shape outlines on a black background](image.png)

## Run the example

From the project directory, use Cargo with a Rust toolchain that supports the 2024 edition.

```sh
cargo run
```

Open `image.png` in an image viewer. The program writes the file in the current working directory and replaces it on each run. Some circles appear as arcs because their outlines extend beyond the canvas.

## Draw your own shapes

The origin is the upper-left pixel. Increasing `x` moves right, and increasing `y` moves down. To try this example, replace the drawing statements between `Image::blank` and `raster::save` in [src/main.rs](src/main.rs). Keep its imports and the `Displayable` implementation.

```rust
let a = gs::Point::new(40, 60);
let b = gs::Point::new(180, 140);
let c = gs::Point::new(220, 40);

gs::Rectangle::new(&b, &a).draw(&mut image); // Opposite corners in either order
gs::Triangle::new(&a, &b, &c).draw(&mut image);
gs::Line::new(&a, &b).draw(&mut image);
gs::Circle::new(&c, 25).draw(&mut image);
a.draw(&mut image);
```

For random placement, pass the canvas width first and height second.

```rust
gs::Point::random(image.width, image.height).draw(&mut image);
gs::Line::random(image.width, image.height).draw(&mut image);
gs::Circle::random(image.width, image.height).draw(&mut image);
```

Random points, line endpoints, and circle centers use coordinates in `0..width` and `0..height`. Random circles choose a radius in `1..width`, so they require a width of at least two pixels. All three functions require positive dimensions.
