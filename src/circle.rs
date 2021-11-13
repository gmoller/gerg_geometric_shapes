use glam::Vec2;

use crate::rectangle::Rectangle;

pub struct Circle {
    center: Vec2,
    radius: f32
}
impl Circle {
    pub fn new(center: Vec2, radius: f32)-> Circle {
        Circle { center, radius }
    }
    pub fn center(&self) -> Vec2 {
        self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn rectangle_hull(&self) -> Rectangle {
        let result = Rectangle::new(self.center(), Vec2::new(self.radius() * 2.0, self.radius() * 2.0));

        result
    }
}

//--------------------------------------------------
#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::circle::Circle;

    #[test]
    fn circle_can_be_created() {
        let center = Vec2::new(6.0, 4.0);
        let radius = 4.0;
        let circle = Circle::new(center, radius);

        assert_eq!(circle.center(), center);
        assert_eq!(circle.radius(), radius);
    }

    #[test]
    fn rectangle_hull() {
        let center = Vec2::new(4.0, 4.0);
        let radius = 4.0;
        let circle = Circle::new(center, radius);

        let rectangle_hull = circle.rectangle_hull();

        assert_eq!(rectangle_hull.left(), 0.0);
        assert_eq!(rectangle_hull.right(), 8.0);
        assert_eq!(rectangle_hull.top(), 8.0);
        assert_eq!(rectangle_hull.bottom(), 0.0);
    }
}
