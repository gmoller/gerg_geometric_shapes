pub struct Range {
    min: f32,
    max: f32
}
impl Range {
    pub fn new(min: f32, max: f32)-> Range {
        Range { min, max }
    }
    pub fn min(&self) -> f32 {
        self.min
    }
    pub fn max(&self) -> f32 {
        self.max
    }
    pub fn intersects(&self, b: Range) -> bool {
        b.min <= self.max && self.min <= b.max
    }
    pub fn contains(&self, f: f32) -> bool {
        f >= self.min && f <= self.max
    }
    pub fn sort(&self) -> Range {
        let result = if self.min > self.max {
            let result = Range::new(self.max, self.min);
            result
        } else {
            let result = Range::new(self.min, self.max);
            result
        };

        result
    }
    pub fn hull(&self, b: Range) -> Range {
        let hull_minimum = if self.min < b.min { self.min } else { b.min };
        let hull_maximum = if self.max > b.max { self.max } else { b.max };
        let result = Range::new(hull_minimum, hull_maximum);

        result
    }
}