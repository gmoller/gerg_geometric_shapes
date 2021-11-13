use std::ops::Neg;
use glam::Vec2;

use crate::{Vec2Extensions, circle::Circle, line_segment::LineSegment, rectangle::Rectangle};

pub struct OrientedRectangle {
    rectangle: Rectangle,
    rotation_in_degrees: f32
}
impl OrientedRectangle {
    pub fn new(rectangle: Rectangle, rotation_in_degrees: f32)-> OrientedRectangle {
        OrientedRectangle { rectangle, rotation_in_degrees }
    }
    pub fn center(&self) -> Vec2 {
        self.rectangle.center()
    }
    pub fn size(&self) -> Vec2 {
        self.rectangle.size()
    }
    pub fn half_extend(&self) -> Vec2 {
        self.rectangle.size() * 0.5
    }
    pub fn width(&self) -> f32 {
        self.rectangle.width()
    }
    pub fn height(&self) -> f32 {
        self.rectangle.height()
    }
    pub fn rotation_in_degrees(&self) -> f32 {
        self.rotation_in_degrees
    }
    pub fn get_corner(&self, number: u32) -> Vec2 {
        let corner_number = number % 4;

        let c = match corner_number {
            0 => Vec2::new(-self.half_extend().x, self.half_extend().y),
            1 => self.half_extend(),
            2 => Vec2::new(self.half_extend().x, -self.half_extend().y),
            3 => self.half_extend().neg(),
            _ => panic!("Unknown corner number.")
        };

        let c = c.rotate_vector(self.rotation_in_degrees());
        let result = c + self.center();

        result
    }
    pub fn get_edge(&self, number: u32) -> LineSegment {
        let edge_number = number % 4;

        let mut a = self.half_extend();
        let mut b = self.half_extend();
        
        if edge_number == 0 {
            a.x = -a.x;
        } else if edge_number == 1 {
            b.y = -b.y;
        } else if edge_number == 2 {
            a.y = -a.y;
            b = b.neg();
        } else if edge_number == 3 {
            a = a.neg();
            b.x = -b.x;
        } else {
            panic!("Unknown edge number.");
        };

        let a = a.rotate_vector(self.rotation_in_degrees());
        let a = a + self.center();

        let b = b.rotate_vector(self.rotation_in_degrees());
        let b = b + self.center();

        let result = LineSegment::new(a, b);

        result
    }
    pub fn has_separating_axis(&self, axis: &LineSegment) -> bool {
        let r_edge_0 = self.get_edge(0);
        let r_edge_2 = self.get_edge(2);
        let n = axis.point1() - axis.point2();
        let axis_range = axis.project(n);
        let r0_range = r_edge_0.project(n);
        let r2_range = r_edge_2.project(n);
        let r_projection = r0_range.hull(r2_range);
        let result = !axis_range.intersects(r_projection);

        result
    }
    pub fn rectangle_hull(&self) -> Rectangle {
        let result = Rectangle::new(self.center(), Vec2::new(0.0, 0.0));

        let corner = self.get_corner(0);
        let result = result.enlarge_to_point(corner);

        let corner = self.get_corner(1);
        let result = result.enlarge_to_point(corner);

        let corner = self.get_corner(2);
        let result = result.enlarge_to_point(corner);

        let corner = self.get_corner(3);
        let result = result.enlarge_to_point(corner);

        result
    }
    pub fn circle_hull(&self) -> Circle {
        let result = Circle::new(self.center(), self.half_extend().length());

        result
    }
    pub fn to_local_rectangle(&self) -> Rectangle {
        let result = Rectangle::new(self.size() * 0.5, self.size());

        result
    }
}

//--------------------------------------------------
#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::{line_segment::LineSegment, oriented_rectangle::OrientedRectangle, rectangle::Rectangle};

    #[test]
    fn oriented_rectangle_can_be_created() {
        let center = Vec2::new(6.0, 4.0);
        let size = Vec2::new(3.0, 2.0);
        let rotation = 30.0;
        let rectangle = Rectangle::new(center, size);
        let oriented_rectangle = OrientedRectangle::new(rectangle, rotation);

        assert_eq!(oriented_rectangle.center(), center);
        assert_eq!(oriented_rectangle.size(), size);
        assert_eq!(oriented_rectangle.width(), 3.0);
        assert_eq!(oriented_rectangle.height(), 2.0);
        assert_eq!(oriented_rectangle.rotation_in_degrees(), 30.0);
        assert_eq!(oriented_rectangle.half_extend(), Vec2::new(1.5, 1.0));
    }

    #[test]
    fn get_corner() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rotation = 0.0;
        let rectangle = Rectangle::new(center, size);
        let oriented_rectangle = OrientedRectangle::new(rectangle, rotation);

        assert_eq!(oriented_rectangle.get_corner(0), Vec2::new(3.0, 5.0));
        assert_eq!(oriented_rectangle.get_corner(1), Vec2::new(5.0, 5.0));
        assert_eq!(oriented_rectangle.get_corner(2), Vec2::new(5.0, 3.0));
        assert_eq!(oriented_rectangle.get_corner(3), Vec2::new(3.0, 3.0));
    }

    #[test]
    fn get_edge() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rotation = 0.0;
        let rectangle = Rectangle::new(center, size);
        let oriented_rectangle = OrientedRectangle::new(rectangle, rotation);

        assert_eq!(oriented_rectangle.get_edge(0).point1(), Vec2::new(3.0, 5.0));
        assert_eq!(oriented_rectangle.get_edge(0).point2(), Vec2::new(5.0, 5.0));

        assert_eq!(oriented_rectangle.get_edge(1).point1(), Vec2::new(5.0, 5.0));
        assert_eq!(oriented_rectangle.get_edge(1).point2(), Vec2::new(5.0, 3.0));

        assert_eq!(oriented_rectangle.get_edge(2).point1(), Vec2::new(5.0, 3.0));
        assert_eq!(oriented_rectangle.get_edge(2).point2(), Vec2::new(3.0, 3.0));

        assert_eq!(oriented_rectangle.get_edge(3).point1(), Vec2::new(3.0, 3.0));
        assert_eq!(oriented_rectangle.get_edge(3).point2(), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn has_separating_axis() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rotation = 0.0;
        let rectangle = Rectangle::new(center, size);
        let oriented_rectangle = OrientedRectangle::new(rectangle, rotation);

        let line_segment1 = LineSegment::new(Vec2::new(6.0, 6.0), Vec2::new(8.0, 6.0));
        let line_segment2 = LineSegment::new(Vec2::new(3.0, 6.0), Vec2::new(4.0, 6.0));

        assert!( oriented_rectangle.has_separating_axis(&line_segment1));
        assert!(!oriented_rectangle.has_separating_axis(&line_segment2));
    }

    #[test]
    fn rectangle_hull() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rotation = 45.0;
        let rectangle = Rectangle::new(center, size);
        let oriented_rectangle = OrientedRectangle::new(rectangle, rotation);

        let rectangle_hull = oriented_rectangle.rectangle_hull();

        assert_eq!(rectangle_hull.left(), 2.5857863);
        assert_eq!(rectangle_hull.right(), 5.4142137);
        assert_eq!(rectangle_hull.top(), 5.414213);
        assert_eq!(rectangle_hull.bottom(), 2.5857863);
    }

    #[test]
    fn circle_hull() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rotation = 10.0;
        let rectangle = Rectangle::new(center, size);
        let oriented_rectangle = OrientedRectangle::new(rectangle, rotation);

        let circle_hull = oriented_rectangle.circle_hull();

        assert_eq!(circle_hull.center(), oriented_rectangle.center());
        assert_eq!(circle_hull.radius(), 1.4142135);
    }

    #[test]
    fn to_local_rectangle() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rotation = 15.0;
        let rectangle = Rectangle::new(center, size);
        let oriented_rectangle = OrientedRectangle::new(rectangle, rotation);

        let local_rectangle = oriented_rectangle.to_local_rectangle();

        assert_eq!(local_rectangle.center(), Vec2::new(1.0, 1.0));
        assert_eq!(local_rectangle.size(), Vec2::new(2.0, 2.0));
    }
}
