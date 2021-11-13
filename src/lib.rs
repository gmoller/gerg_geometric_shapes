use float_cmp::{F32Margin, approx_eq};
use glam::Vec2;

pub mod circle;
pub mod line;
pub mod line_segment;
pub mod oriented_rectangle;
pub mod range;
pub mod rectangle;

pub trait Vec2Extensions {
    fn rotate_vector(&self, angle_in_degrees: f32) -> Vec2;
    fn rotate_vector_90(&self) -> Vec2;
    fn is_parallel(&self, b: Vec2) -> bool;
    fn project(&self, onto: Vec2) -> Vec2;
}
impl Vec2Extensions for Vec2 {
    fn rotate_vector(&self, angle_in_degrees: f32) -> Vec2 {
        let radians = angle_in_degrees.to_radians();
        let sin = radians.sin();
        let cos = radians.cos();

        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        let result = Vec2::new(x, y);

        result
    }
    fn rotate_vector_90(&self) -> Vec2 {
        let result = Vec2::new(-self.y, self.x);

        result
    }
    fn is_parallel(&self, b: Vec2) -> bool {
        let na = self.rotate_vector_90();
        let dot_product = na.dot(b);
        let result = approx_eq!(f32, 0.0, dot_product, F32Margin::default());

        result
    }
    fn project(&self, onto: Vec2) -> Vec2 {
        let d = onto.dot(onto);
        if d > 0.0 {
            let dp = self.dot(onto);
            let foo = onto * dp / d;

            return foo;
        }

        onto
    }
}