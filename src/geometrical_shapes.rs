use raster::{Color, Image};
use rand::Rng;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
} 

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        let r = rand::thread_rng().gen_range(0..=255);
        let g = rand::thread_rng().gen_range(0..=255);
        let b = rand::thread_rng().gen_range(0..=255);
        let a = rand::thread_rng().gen_range(0..=255);

        Color { r, g, b, a }
    }
}

// Point
#[derive (Clone)]
pub struct Point (i32, i32);
impl Point {
   pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::thread_rng().gen_range(0..=width);
        let y = rand::thread_rng().gen_range(0..=height);

        Point::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, self.color());
    }
}

// Line
pub struct Line (Point, Point);
impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self(p1, p2)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::thread_rng().gen_range(0..=width);
        let y = rand::thread_rng().gen_range(0..=height);
        let x2 = rand::thread_rng().gen_range(0..=width);
        let y2 = rand::thread_rng().gen_range(0..=height);

        Line::new(Point(x, y), Point(x2, y2))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = self.1.0 - self.0.0;
        let dy = self.1.1 - self.0.1;

        let step = dx.abs().max(dy.abs());

        let x_inc = dx as f64 / step as f64;
        let y_inc = dy as f64 / step as f64;

        let mut x = self.0.0 as f64;
        let mut y = self.0.1 as f64;
        let cl = Color::white();

        for _ in 0..=step {
            image.display(x.round() as i32, y.round() as i32, cl.clone());
            x += x_inc;
            y += y_inc;
        }
    }
}

// Rectangle
pub struct Rectangle ( Point, Point);
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_left = Point::new(self.0.0, self.1.1);
        let top_right = Point::new(self.1.0, self.1.1);
        let bottom_left = Point::new(self.0.0, self.0.1);
        let bottom_right = Point::new(self.1.0, self.0.1);
    
        Line::new(top_left.clone(), top_right.clone()).draw(image);
        Line::new(top_right.clone(), bottom_right.clone()).draw(image);
        Line::new(bottom_right.clone(), bottom_left.clone()).draw(image);
        Line::new(bottom_left.clone(), top_left.clone()).draw(image);
    }
}

// Triangle
pub struct Triangle (Point, Point, Point);
impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self(p1.clone(), p2.clone(), p3.clone())
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.0.clone(), self.1.clone()).draw(image);
        Line::new(self.1.clone(), self.2.clone()).draw(image);
        Line::new(self.2.clone(), self.0.clone()).draw(image);
    }
}

// Circle
pub struct Circle {
    point: Point,
    radius: i32
}
impl Circle {
    fn new(point: Point, radius: i32) -> Self {
        Self{
            point,
            radius
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::thread_rng().gen_range(0..=width);
        let y = rand::thread_rng().gen_range(0..=height);
        let radius = rand::thread_rng().gen_range(1..=((width+height)/2)/2);

        Circle::new(Point::new(x, y), radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let center_x = self.point.0;
        let center_y = self.point.1;
        let radius = self.radius;
        let color = self.color();
        
        let num_points = radius * 360;

        for i in 0..num_points {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / (num_points as f64);
            
            // Calculate point on circle
            let x = (center_x as f64 + radius as f64 * angle.cos()).round() as i32;
            let y = (center_y as f64 + radius as f64 * angle.sin()).round() as i32;
            
            image.display(x, y, color.clone());
        }
    }
}