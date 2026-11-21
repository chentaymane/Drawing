use raster::{Color, Image};
use rand::Rng;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug, Clone)]
pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn random(x: i32, y: i32) -> Self {
        let mut rng = rand::rng();
        Self(
            rng.random_range(0..x),
            rng.random_range(0..y)
        )
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, self.color());
    }

    fn color(&self) -> Color {
        let mut rng = rand::rng();
        let red = rng.random_range(0..255);
        let green = rng.random_range(0..255);
        let blue = rng.random_range(0..255);
        Color::rgb(red,green,blue)
    }
}

pub struct Line(
    pub Point,
    pub Point
);

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self(start.clone(), end.clone())
    }

    pub fn random(x: i32, y: i32) -> Self {
        Self::new(&Point::random(x, y), &Point::random(x, y))
    }

    fn draw_with_color(&self, image: &mut Image, color: Color) {
        let mut x = self.0.0;
        let mut y = self.0.1;

        let x2 = self.1.0;
        let y2 = self.1.1;

        let dx = (x2 - x).abs();
        let dy = -(y2 - y).abs();

        let sx = if x < x2 { 1 } else { -1 };
        let sy = if y < y2 { 1 } else { -1 };

        let mut error = dx + dy;

        loop {
            image.display(x, y, color.clone());

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * error;

            if e2 >= dy {
                error += dy;
                x += sx;
            }

            if e2 <= dx {
                error += dx;
                y += sy;
            }
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        self.draw_with_color(image, self.color());
    }

    fn color(&self) -> Color {
        let mut rng = rand::rng();
        let red = rng.random_range(0..255);
        let green = rng.random_range(0..255);
        let blue = rng.random_range(0..255);
        Color::rgb(red,green,blue)
    }
}

pub struct Triangle(
    pub Point,
    pub Point,
    pub Point
);

impl Triangle {
    pub fn new(x: &Point, y: &Point, z: &Point) -> Self {
        let xc: Point = (*x).clone();
        let yc: Point = (*y).clone();
        let zc: Point = (*z).clone();
        Self(xc, yc, zc)
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let lines = [
            Line(self.0.clone(), self.1.clone()),
            Line(self.1.clone(), self.2.clone()),
            Line(self.2.clone(), self.0.clone()),
        ];

        let color = self.color();
        for line in &lines {
            line.draw_with_color(image, color.clone());
        }
    }

    fn color(&self) -> Color {
        let mut rng = rand::rng();
        let red = rng.random_range(0..255);
        let green = rng.random_range(0..255);
        let blue = rng.random_range(0..255);
        Color::rgb(red,green,blue)
    }
}
