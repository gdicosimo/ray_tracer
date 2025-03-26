/// A closed interval [min, max] on the real line.
///
///
/// # Examples
///
/// ```
/// use crate::math::Interval;
///
/// // Create an interval [1,5]
/// let i = Interval::new(1.0, 5.0);
///
/// // Check if a value is in the interval
/// assert!(i.contains(3.0));
/// assert!(!i.contains(6.0));
///
/// // Merge two intervals
/// let j = Interval::new(4.0, 7.0);
/// let merged = i.merge(&j);
/// assert_eq!(merged.min(), 1.0);
/// assert_eq!(merged.max(), 7.0);
/// ```
#[derive(Debug, Clone)]
pub struct Interval {
    pub(crate) min: f32,
    pub(crate) max: f32,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    pub const CAMERA_VIEW: Interval = Interval {
        min: 0.001,
        max: f32::INFINITY,
    };

    pub const fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
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

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let new_min = self.min.max(other.min);
        let new_max = self.max.min(other.max);
        if new_min > new_max {
            None
        } else {
            Some(Interval::new(new_min, new_max))
        }
    }

    #[inline(always)]
    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }

    #[inline(always)]
    pub fn expand(&self, delta: f32) -> Self {
        let padding = 2.0 / delta;
        Self::new(self.min - padding, self.max + padding)
    }

    #[inline(always)]
    pub fn expand_inplace(&mut self, delta: f32) {
        let padding = 2.0 / delta;
        self.min -= padding;
        self.max += padding;
    }

    #[inline(always)]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    #[inline(always)]
    pub fn merge_inplace(&mut self, other: &Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl std::ops::Add<f32> for &Interval {
    type Output = Interval;

    fn add(self, rhs: f32) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl std::ops::Add<&Interval> for f32 {
    type Output = Interval;

    fn add(self, rhs: &Interval) -> Self::Output {
        rhs + self
    }
}
