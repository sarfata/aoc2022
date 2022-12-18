/*

> Worry levels are no longer divided by three after each item is inspected; you'll need to find another way to keep your worry levels manageable.

Divider are always prime numbers.

If x mod d == 0 => x * y mod d == 0 (for any int value of y)
And therefore x * x mod d == 0 too.

=> This is a sufficient condition. But it's not necessary.

Yes it kinda is. Because if
n = p1^e1 + p2^e2 + p3^e3 ...

Divisible by a prime => does px have ex > 0.

When multiplying by self, e2' = e2 * 2.

When multiplying by a prime, eg: x3
e3' = e3 + 1

When adding, we have to recalculate the prime factors?



How about, we just store the result of mod x

 - n mod a prime factor will be 0
 - multiplying by a prime factor will set that mod to 0
 - adding by a number smaller than a prime factor of interest will add to that mod result until it's 0.
 - multiplying by self does not change the mods.


*/

const INTERESTING_PRIMES: [u32; 8] = [13, 19, 5, 2, 17, 11, 7, 3];

#[derive(Clone, Debug)]
pub struct Worry {
    mod_of: [u32; INTERESTING_PRIMES.len()],
    // v: u128,
}

impl Worry {
    pub fn from(x: &u32) -> Self {
        let mut mods = [0; INTERESTING_PRIMES.len()];

        for i in 0..INTERESTING_PRIMES.len() {
            mods[i] = x % INTERESTING_PRIMES[i];
        }
        Worry {
            mod_of: mods,
            // v: *x as u128,
        }
    }

    /// When you multiply a number by y, it becomes dividable by y.
    pub fn mul(&mut self, y: u32) {
        for i in 0..INTERESTING_PRIMES.len() {
            if INTERESTING_PRIMES[i] == y {
                self.mod_of[i] = 0;
            } else {
                self.mod_of[i] = self.mod_of[i] * y % INTERESTING_PRIMES[i];
            }
        }
        // self.v = self.v * y as u128;
        // self.self_check("mul");
    }

    /// When you add to a number, you add to the rest of the euclidian division, up to the divider
    pub fn add(&mut self, y: u32) {
        for i in 0..INTERESTING_PRIMES.len() {
            self.mod_of[i] = (self.mod_of[i] + y) % INTERESTING_PRIMES[i];
        }
        // self.v = self.v + y as u128;
        // self.self_check("add");
    }

    pub fn square(&mut self) {
        // self.v *= self.v;
        for i in 0..INTERESTING_PRIMES.len() {
            self.mod_of[i] = (self.mod_of[i] * self.mod_of[i]) % INTERESTING_PRIMES[i];
        }
        // squaring has no effect on the remainder
        // self.self_check("square");
    }

    /// A number is dividable by y if the rest of the euclidian division is 0
    pub fn dividable_by(&self, y: u32) -> bool {
        // let result = self.v % y as u128 == 0;

        for i in 0..INTERESTING_PRIMES.len() {
            if INTERESTING_PRIMES[i] == y {
                let r = self.mod_of[i] == 0;
                // if r != result {
                //     println!("{} % {} = {} but {:?}", self.v, y, result, self);
                // }
                // assert_eq!(r, result);
                return r;
            }
        }

        // This should never happen because we should have found y.
        // Might be able to avoid this by using enums to limit the range of prime numbers.
        assert!(false);
        false
    }

    // fn self_check(&self, label: &str) {
    //     for i in 0..INTERESTING_PRIMES.len() {
    //         if self.v % INTERESTING_PRIMES[i] as u128 != self.mod_of[i] as u128 {
    //             println!(
    //                 "After {}: {} % {} = {}   but mod_of[{}]={}",
    //                 label,
    //                 self.v,
    //                 INTERESTING_PRIMES[i],
    //                 self.v % INTERESTING_PRIMES[i] as u128,
    //                 INTERESTING_PRIMES[i],
    //                 self.mod_of[i]
    //             );
    //             assert!(false);
    //         }
    //     }
    // }
}

use std::fmt;
impl fmt::Display for Worry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // write!(f, "WL<{}>[", self.v)?;
        write!(f, "WL[")?;
        for i in 0..INTERESTING_PRIMES.len() {
            write!(f, "%{}={} ", INTERESTING_PRIMES[i], self.mod_of[i])?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {}
