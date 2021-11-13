use glam::Vec2;

use crate::{circle::Circle, line_segment::LineSegment};

pub struct Rectangle {
    center: Vec2,
    size: Vec2
}
impl Rectangle {
    pub fn new(center: Vec2, size: Vec2)-> Rectangle {
        Rectangle { center, size }
    }
    pub fn center(&self) -> Vec2 {
        self.center
    }
    pub fn size(&self) -> Vec2 {
        self.size
    }
    pub fn left(&self) -> f32 {
        self.center.x - (self.width() * 0.5)
    }
    pub fn right(&self) -> f32 {
        self.center.x + (self.width() * 0.5)
    }
    pub fn top(&self) -> f32 {
        self.center.y + (self.height() * 0.5)
    }
    pub fn bottom(&self) -> f32 {
        self.center.y - (self.height() * 0.5)
    }
    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.left(), self.top())
    }
    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.top())
    }
    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.left(), self.bottom())
    }
    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.bottom())
    }
    pub fn width(&self) -> f32 {
        self.size.x
    }
    pub fn height(&self) -> f32 {
        self.size.y
    }
    pub fn get_corner(&self, number: u32) -> Vec2 {
        let corner_number = number % 4;

        let result = match corner_number {
            0 => self.bottom_right(),
            1 => self.top_right(),
            2 => self.top_left(),
            3 => self.bottom_left(),
            _ => panic!("Unknown corner number.")
        };

        result
    }
    pub fn get_corner_closest_to_point(&self, point: Vec2) -> Vec2 {
        let x = point.x.clamp(self.left(), self.right());
        let y = point.y.clamp(self.bottom(), self.top());
        let result = Vec2::new(x, y);

        result
    }
    pub fn get_edge(&self, number: u32) -> LineSegment {
        let edge_number = number % 4;

        let result = match edge_number {
            0 => LineSegment::new(self.get_corner(0), self.get_corner(1)),
            1 => LineSegment::new(self.get_corner(1), self.get_corner(2)),
            2 => LineSegment::new(self.get_corner(2), self.get_corner(3)),
            3 => LineSegment::new(self.get_corner(3), self.get_corner(0)),
            _ => panic!("Unknown edge number.")
        };

        result
    }
    pub fn has_separating_axis(&self, axis: &LineSegment) -> bool {
        let n = axis.point1() - axis.point2();
        let r_edge_0 = self.get_edge(0);
        let r_edge_0_range = r_edge_0.project(n);

        let r_edge_2 = self.get_edge(2);
        let r_edge_2_range = r_edge_2.project(n);

        let r_projection = r_edge_0_range.hull(r_edge_2_range);

        let axis_range = axis.project(n);

        let result = !axis_range.intersects(r_projection);

        result
    }
    pub fn enlarge_to_point(&self, p: Vec2) -> Rectangle {
        let x = self.left().min(p.x);
        let y = self.bottom().min(p.y);

        let size_x = self.right().max(p.x) - x;
        let size_y = self.top().max(p.y) - y;

        let result = Rectangle::new(Vec2::new(x + size_x * 0.5, y + size_y * 0.5), Vec2::new(size_x, size_y));

        result
    }
    pub fn circle_hull(&self) -> Circle {
        let half_size = self.size() * 0.5;
        let result = Circle::new(self.center(), half_size.length());

        result
    }
}

//--------------------------------------------------
#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::{line_segment::LineSegment, rectangle::Rectangle};

    #[test]
    fn rectangle_can_be_created() {
        let center = Vec2::new(5.0, 5.0);
        let size = Vec2::new(6.0, 4.0);
        let rectangle = Rectangle::new(center, size);

        assert_eq!(rectangle.center(), center);
        assert_eq!(rectangle.size(), size);
        assert_eq!(rectangle.left(), 2.0);
        assert_eq!(rectangle.right(), 8.0);
        assert_eq!(rectangle.top(), 7.0);
        assert_eq!(rectangle.bottom(), 3.0);
        assert_eq!(rectangle.top_left(), Vec2::new(2.0, 7.0));
        assert_eq!(rectangle.top_right(), Vec2::new(8.0, 7.0));
        assert_eq!(rectangle.bottom_right(), Vec2::new(8.0, 3.0));
        assert_eq!(rectangle.bottom_left(), Vec2::new(2.0, 3.0));
        assert_eq!(rectangle.width(), 6.0);
        assert_eq!(rectangle.height(), 4.0);
    }

    #[test]
    fn get_corner() {
        let center = Vec2::new(5.0, 5.0);
        let size = Vec2::new(6.0, 4.0);
        let rectangle = Rectangle::new(center, size);

        assert_eq!(rectangle.get_corner(0), Vec2::new(8.0, 3.0)); // bottom right
        assert_eq!(rectangle.get_corner(1), Vec2::new(8.0, 7.0)); // top right
        assert_eq!(rectangle.get_corner(2), Vec2::new(2.0, 7.0)); // top left
        assert_eq!(rectangle.get_corner(3), Vec2::new(2.0, 3.0)); // bottom left
    }

    #[test]
    fn get_corner_closest_to_point() {
        let center = Vec2::new(5.0, 5.0);
        let size = Vec2::new(6.0, 4.0);
        let rectangle = Rectangle::new(center, size);

        let p1 = Vec2::new(1.9, 7.1); // top left
        let p2 = Vec2::new(8.1, 7.1); // top right
        let p3 = Vec2::new(8.1, 2.9); // bottom right
        let p4 = Vec2::new(1.9, 2.9); // bottom left
        let p5 = Vec2::new(5.0, 5.0); // center

        assert_eq!(rectangle.get_corner_closest_to_point(p1), Vec2::new(2.0, 7.0));
        assert_eq!(rectangle.get_corner_closest_to_point(p2), Vec2::new(8.0, 7.0));
        assert_eq!(rectangle.get_corner_closest_to_point(p3), Vec2::new(8.0, 3.0));
        assert_eq!(rectangle.get_corner_closest_to_point(p4), Vec2::new(2.0, 3.0));
        assert_eq!(rectangle.get_corner_closest_to_point(p5), Vec2::new(5.0, 5.0));
    }

    #[test]
    fn get_edge() {
        let center = Vec2::new(5.0, 5.0);
        let size = Vec2::new(6.0, 4.0);
        let rectangle = Rectangle::new(center, size);

        assert_eq!(rectangle.get_edge(0).point1(), Vec2::new(8.0, 3.0));
        assert_eq!(rectangle.get_edge(0).point2(), Vec2::new(8.0, 7.0));

        assert_eq!(rectangle.get_edge(1).point1(), Vec2::new(8.0, 7.0));
        assert_eq!(rectangle.get_edge(1).point2(), Vec2::new(2.0, 7.0));

        assert_eq!(rectangle.get_edge(2).point1(), Vec2::new(2.0, 7.0));
        assert_eq!(rectangle.get_edge(2).point2(), Vec2::new(2.0, 3.0));

        assert_eq!(rectangle.get_edge(3).point1(), Vec2::new(2.0, 3.0));
        assert_eq!(rectangle.get_edge(3).point2(), Vec2::new(8.0, 3.0));
    }

    #[test]
    fn has_separating_axis() {
        let center = Vec2::new(5.0, 5.0);
        let size = Vec2::new(6.0, 4.0);
        let rectangle = Rectangle::new(center, size);

        let line_segment1 = LineSegment::new(Vec2::new(10.0, 9.0), Vec2::new(15.0, 9.0));
        let line_segment2 = LineSegment::new(Vec2::new(3.0, 9.0), Vec2::new(4.0, 9.0));

        assert!( rectangle.has_separating_axis(&line_segment1));
        assert!(!rectangle.has_separating_axis(&line_segment2));
    }

    #[test]
    fn enlarge_to_point() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rectangle = Rectangle::new(center, size);

        let p1 = Vec2::new(2.0, 6.0);

        assert_eq!(rectangle.enlarge_to_point(p1).left(), 2.0);
        assert_eq!(rectangle.enlarge_to_point(p1).right(), 5.0);
        assert_eq!(rectangle.enlarge_to_point(p1).top(), 6.0);
        assert_eq!(rectangle.enlarge_to_point(p1).bottom(), 3.0);
    }

    #[test]
    fn circle_hull() {
        let center = Vec2::new(4.0, 4.0);
        let size = Vec2::new(2.0, 2.0);
        let rectangle = Rectangle::new(center, size);

        let circle_hull = rectangle.circle_hull();

        assert_eq!(circle_hull.center(), rectangle.center());
        assert_eq!(circle_hull.radius(), 1.4142135);
    }
}
