use glam::Vec2;

use crate::range::Range;

pub struct LineSegment {
    point1: Vec2,
    point2: Vec2
}
impl LineSegment {
    pub fn new(point1: Vec2, point2: Vec2)-> LineSegment {
        LineSegment { point1, point2 }
    }
    pub fn point1(&self) -> Vec2 {
        self.point1
    }
    pub fn point2(&self) -> Vec2 {
        self.point2
    }

    pub fn project(&self, onto: Vec2) -> Range {
        let onto_unit = onto.normalize();
        let min = onto_unit.dot(self.point1());
        let max = onto_unit.dot(self.point2());
        let result = Range::new(min, max).sort();
    
        result
    }
}

//--------------------------------------------------
#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::line_segment::LineSegment;

    #[test]
    fn line_segment_can_be_created() {
        let point1 = Vec2::new(3.0, 4.0);
        let point2 = Vec2::new(11.0, 1.0);
        let line_segment = LineSegment::new(point1, point2);

        assert_eq!(line_segment.point1(), point1);
        assert_eq!(line_segment.point2(), point2);
    }

    #[test]
    fn line_segment_projection() {
        let point1 = Vec2::new(3.0, 3.0);
        let point2 = Vec2::new(6.0, 3.0);
        let line_segment = LineSegment::new(point1, point2);

        let result = line_segment.project(Vec2::new(1.0, 0.0));

        assert_eq!(result.min(), 3.0);
        assert_eq!(result.max(), 6.0);
    }
}
