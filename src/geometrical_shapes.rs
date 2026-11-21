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
