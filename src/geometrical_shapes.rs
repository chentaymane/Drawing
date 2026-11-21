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

pub struct Rectangle(
    pub Point,
    pub Point,
    pub Point,
    pub Point
);

impl Rectangle {
    pub fn new(pt1: &Point, pt2: &Point) -> Self {
        Self(
            Point(pt1.0, pt1.1),
            Point(pt2.0, pt1.1),
            Point(pt2.0, pt2.1),
            Point(pt1.0, pt2.1)
        )
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let lines = [
            Line(self.0.clone(), self.1.clone()),
            Line(self.1.clone(), self.2.clone()),
            Line(self.2.clone(), self.3.clone()),
            Line(self.3.clone(), self.0.clone()),
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

pub struct Circle(
    pub Point,
    pub usize
);

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self(center.clone(), radius.clone() as usize)
    }

    pub fn random(x: i32, y: i32) -> Self {
        let mut rng = rand::rng();
        Self::new(&Point::random(x, y), rng.random_range(1..x))
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let cx = self.0.0 as f64;
        let cy = self.0.1 as f64;
        let radius = self.1 as f64;

        let color = self.color();
        for degree in 0..3600 {
            let angle = (degree as f64 / 10.0).to_radians();

            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin();

            image.display(
                x.round() as i32,
                y.round() as i32,
                color.clone(),
            );
        }
    }

    fn color(&self) -> Color {
        let mut rng = rand::rng();
        let red = rng.random_range(0..=255);
        let green = rng.random_range(0..=255);
        let blue = rng.random_range(0..=255);
        Color::rgb(red,green,blue)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // 1. Test random Point bounds
    #[test]
    fn test_point_random_bounds() {
        for _ in 0..100 {
            let point = Point::random(100, 200);

            assert!(point.0 >= 0 && point.0 < 100);
            assert!(point.1 >= 0 && point.1 < 200);
        }
    }

    // 2. Test random Line bounds
    #[test]
    fn test_line_random_bounds() {
        for _ in 0..100 {
            let line = Line::random(100, 200);

            assert!(line.0.0 >= 0 && line.0.0 < 100);
            assert!(line.0.1 >= 0 && line.0.1 < 200);

            assert!(line.1.0 >= 0 && line.1.0 < 100);
            assert!(line.1.1 >= 0 && line.1.1 < 200);
        }
    }

    // 3. Test random Circle bounds
    #[test]
    fn test_circle_random_bounds() {
        for _ in 0..100 {
            let circle = Circle::random(100, 200);

            let x = circle.0.0;
            let y = circle.0.1;

            assert!(x >= 0 && x < 100);
            assert!(y >= 0 && y < 200);
        }
    }

    // 4. Test Rectangle with reversed points
    #[test]
    fn test_rectangle_reversed() {
        let rectangle = Rectangle::new(
            &Point::new(150, 300),
            &Point::new(50, 60),
        );

        assert_eq!(rectangle.0.0, 150);
        assert_eq!(rectangle.0.1, 300);

        assert_eq!(rectangle.1.0, 50);
        assert_eq!(rectangle.1.1, 300);

        assert_eq!(rectangle.2.0, 50);
        assert_eq!(rectangle.2.1, 60);

        assert_eq!(rectangle.3.0, 150);
        assert_eq!(rectangle.3.1, 60);
    }
}
