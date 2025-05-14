mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rec_point1 = gs::Point::new(150, 150);
    let rec_point2 = gs::Point::new(50, 50);
    let rectangle = gs::Rectangle::new(&rec_point1, &rec_point2);
    rectangle.draw(&mut image);

    let triangle_p1 = gs::Point::new(500, 500);
    let triangle_p2 = gs::Point::new(250, 700);
    let triangle_p3 = gs::Point::new(700, 800);
    let triangle = gs::Triangle::new (
            &triangle_p1,
            &triangle_p2,
            &triangle_p3,
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}