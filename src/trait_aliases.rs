use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

/// Non-negative values may be interpreted as distances.
pub trait Position = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy + Display + Debug;

pub trait Element = Debug;