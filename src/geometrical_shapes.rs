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
        let dx = self.0.0 - self.1.0;
        let dy = self.0.1 - self.1.1;

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
pub struct Rectangle<'a> (&'a Point, &'a Point);
impl<'a> Rectangle<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point) -> Self {
        Self(p1, p2)
    }
}

impl<'a> Drawable for Rectangle<'a> {
    fn draw(&self, image: &mut Image) {

        // dda_line(
        //     Point(self.0.0, self.0.1),
        //     Point(self.1.0, self.0.1),
        //     img,
        //     &color,
        // );
        // dda_line(
        //     Point(self.1.0, self.0.1),
        //     Point(self.1.0, self.1.1),
        //     img,
        //     &color,
        // );
        // dda_line(
        //     Point(self.1.0, self.1.1),
        //     Point(self.0.0, self.1.1),
        //     img,
        //     &color,
        // );
        // dda_line(
        //     Point(self.0.0, self.1.1),
        //     Point(self.0.0, self.0.1),
        //     img,
        //     &color,
        // );

        let rb_point = Point::new(self.0.0, self.0.1);
        let lt_point = Point::new(self.1.0, self.1.1);
        let lb_point = Point::new(self.1.0, self.0.1);
        let rt_point = Point::new(self.0.0, self.1.1);

        Point::new(self.0.0, self.0.1).draw(image);
        Point::new(self.1.0, self.1.1).draw(image);

        Line::new(rb_point.clone(), lt_point.clone()).draw(image);
        // Line::new(lb_point.clone(), rb_point.clone()).draw(image);
        // Line::new(lb_point.clone(), lt_point.clone()).draw(image);
        Line::new(lt_point.clone(), rt_point.clone()).draw(image);
    }
}