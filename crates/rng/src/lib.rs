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
//! use std::time::SystemTime;
//! use rng::Rand;
//! let rng = Rand::new(SystemTime::now()
//!                                     .duration_since(UNIX_EPOCH)
//!                                     .unwrap()
//!                                     .as_secs() as u32);
//! let num = Rand::rand();
//! let num_between_10_100 = Rand::rand_range(10, 100);
//! let num_float = Rand::rand_float();
//! 
//! ```


// Seeds
const KX: u32 = 123456789;
const KY: u32 = 362436068;
const KZ: u32 = 521288624;
const KW: u32 = 886751233;

#[inline]
fn rol32(x: u32, k: u32) -> u32 {
    x.rotate_left(k)
}

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
    /// let rng = Rand::new(1234);
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

    /// Generates a pseudo-random `i32` between `floor` and `roof`.
    pub fn rand_range(&mut self, floor: i32, roof: i32) -> i32 {
        let m = (roof - floor + 1) as u32;
        floor + (self.rand() % m) as i32
    }

    /// Generates a pseudo-random `f64`.
    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (u32::MAX as f64)
    }
}
