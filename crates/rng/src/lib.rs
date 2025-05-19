//! # RNG
//!
//! `RNG` is a library that provides a Random Number Generator
//!
//! ## Features
//! - Pseudo Random Number Generator using the [Xoroshiro128+ algorithm](https://en.wikipedia.org/wiki/Xorshift#xoroshiro).
//! - Easy to use
//!
//! ## Example
//! ```
//! fn main() {
//!     use rng::Rand;
//!
//!     let mut rng = Rand::new(1234);
//!
//!     let num = rng.rand();
//!     assert_eq!(num, 1408042037);
//!
//!     let num_between_10_100 = rng.rand_range(10, 100);
//!     assert_eq!(num_between_10_100, 91);
//!
//!     let num_float = rng.rand_float();
//!     assert_eq!(num_float, 0.20413496862261904);
//! }
//!
//! ```

// Seeds. **If you change these you need to update EVERY test**.
const KX: u32 = 123456789;
const KY: u32 = 362436068;
const KZ: u32 = 521288624;
const KW: u32 = 886751233;

#[inline]
fn rol32(x: u32, k: u32) -> u32 {
    x.rotate_left(k)
}

#[derive(Debug)]
pub struct Rand {
    w: u32,
    x: u32,
    y: u32,
    z: u32,
}

impl Rand {
    /// Constructs a new `Rand`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rng::Rand;
    ///
    /// let mut rng = Rand::new(1234);
    /// ```
    pub fn new(seed: u32) -> Self {
        Rand {
            w: KW ^ seed,
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ ^ seed,
        }
    }

    /// Generates a pseudo-random `u32`.
    pub fn rand(&mut self) -> u32 {
        let res = self.w.wrapping_add(self.z);
        let t = self.x.wrapping_shl(17);

        self.y ^= self.w;
        self.z ^= self.x;
        self.x ^= self.y;
        self.w ^= self.z;

        self.y ^= t;
        self.z = rol32(self.z, 45);

        res
    }

    /// Generates a pseudo-random `i32` between `a` and `b`.
    pub fn rand_range(&mut self, a: i32, b: i32) -> i32 {
        let (floor, ceil) = if a < b { (a, b) } else { (b, a) };

        let m = (ceil - floor + 1) as u32;
        floor + (self.rand() % m) as i32
    }

    /// Generates a pseudo-random `f64` between 0 and 1.
    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (u32::MAX as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_rand() {
        let rng = Rand::new(1234);
        assert_eq!(rng.w, 886752467);
        assert_eq!(rng.x, 123455943);
        assert_eq!(rng.y, 362434870);
        assert_eq!(rng.z, 521289570);
    }

    #[test]
    fn rand_simple() {
        let mut rng = Rand::new(1234);
        assert_eq!(rng.rand(), 1408042037);
    }

    #[test]
    fn rand_range_simple() {
        let mut rng = Rand::new(1234);
        assert_eq!(rng.rand_range(10, 100), 48);
    }

    #[test]
    fn rand_range_neg() {
        let mut rng = Rand::new(1234);
        assert_eq!(rng.rand_range(-200, -50), -94);
    }

    #[test]
    fn rand_range_backward() {
        let mut rng = Rand::new(1234);
        assert_eq!(rng.rand_range(500, 10), 365);
    }

    #[test]
    fn rand_range_neg_backward() {
        let mut rng = Rand::new(1234);
        assert_eq!(rng.rand_range(50, -10), 11);
    }

    #[test]
    fn rand_float() {
        let mut rng = Rand::new(1234);
        assert_eq!(rng.rand_float(), 0.3278353338427458);
    }
}
