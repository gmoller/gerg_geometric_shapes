use glam::Vec2;

use crate::{Vec2Extensions, line_segment::LineSegment};

pub struct Line {
    base: Vec2,
    direction: Vec2
}
impl Line {
    pub fn new(base: Vec2, direction: Vec2)-> Line {
        Line { base, direction }
    }
    pub fn base(&self) -> Vec2 {
        self.base
    }
    pub fn direction(&self) -> Vec2 {
        self.direction
    }
    pub fn on_one_side(&self, line_segment: &LineSegment) -> bool {
        let d1 = line_segment.point1() - self.base();
        let d2 = line_segment.point2() - self.base();
        let n = self.direction().rotate_vector_90();
    
        let result = n.dot(d1) * n.dot(d2) > 0.0;
    
        result
    }
    pub fn is_equivalent(&self, line: &Line) -> bool {
        let parallel_vectors = self.direction().is_parallel(line.direction());
        let result = if !parallel_vectors {
            false
        } else {
            let d = self.base() - line.base();
            d.is_parallel(self.direction())
        };

        result
    }
}

//--------------------------------------------------
#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::{line::Line, line_segment::LineSegment};

    #[test]
    fn line_can_be_created() {
        let base = Vec2::new(3.0, 3.0);
        let direction = Vec2::new(7.0, -2.0);
        let line = Line::new(base, direction);

        assert_eq!(line.base(), base);
        assert_eq!(line.direction(), direction);
    }

    #[test]
    fn on_one_side() {
        let base = Vec2::new(3.0, 3.0);
        let direction = Vec2::new(7.0, -2.0);
        let line = Line::new(base, direction);
        let line_segment1 = LineSegment::new(Vec2::new(4.0, 4.0), Vec2::new(8.0, 4.0));
        let line_segment2 = LineSegment::new(Vec2::new(4.0, 4.0), Vec2::new(4.0, 0.0));

        assert!( line.on_one_side(&line_segment1));
        assert!(!line.on_one_side(&line_segment2));
    }

    #[test]
    fn is_equivalent() {
        let base = Vec2::new(3.0, 3.0);
        let direction = Vec2::new(1.0, 0.0);
        let line1 = Line::new(base, direction);
        let line2 = Line::new(base + Vec2::new(1.0, 0.0), direction);
        let line3 = Line::new(base + Vec2::new(1.0, 1.0), direction);

        assert!( line1.is_equivalent(&line2));
        assert!(!line1.is_equivalent(&line3));
    }
}
