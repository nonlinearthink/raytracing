#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub fn empty() -> Interval {
        Interval {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }

    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn merge(&self, interval: &Interval) -> Interval {
        Interval {
            min: f32::min(self.min, interval.min),
            max: f32::max(self.max, interval.max),
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
