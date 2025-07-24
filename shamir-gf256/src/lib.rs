// shamir_gf256.rs
//
// Minimal implementation of Shamir’s Secret Sharing over GF(256)
//
// Arithmetic follows the explanations in Guillaume Endignoux’s blog post:
// https://gendignoux.com/blog/2021/11/01/horcrux-1-math.html
//
// Uses the AES polynomial x^8 + x^4 + x^3 + x + 1 (0x11B) as the field modulus.

use rand::random;
use std::fmt;

/// An element of GF(256), represented as an unsigned byte.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GF256(u8);

impl GF256 {
    /// The field’s irreducible (modulus) polynomial: x^8 + x^4 + x^3 + x + 1
    const MOD: u16 = 0x11B; // 0b1_0001_1011

    /// Zero element
    pub const ZERO: GF256 = GF256(0);
    /// One element
    pub const ONE: GF256 = GF256(1);

    /// Field addition (XOR in GF(256))
    #[inline(always)]
    fn add_internal(self, other: Self) -> Self {
        GF256(self.0 ^ other.0)
    }

    /// Multiply two field elements using Russian‑peasant/xtime algorithm.
    pub fn mul_internal(self, other: Self) -> Self {
        let mut a = self.0;
        let mut b = other.0;
        let mut res: u8 = 0;

        for _ in 0..8 {
            if (b & 1) != 0 {
                res ^= a;
            }
            let carry = (a & 0x80) != 0;
            a <<= 1;
            if carry {
                a ^= (GF256::MOD & 0xFF) as u8;
            }
            b >>= 1;
        }
        GF256(res)
    }

    /// Exponentiate by a 8‑bit exponent (for inversion).
    pub fn pow(self, mut e: u8) -> Self {
        let mut base = self;
        let mut acc = GF256::ONE;
        while e != 0 {
            if e & 1 != 0 {
                acc = acc.mul(base);
            }
            base = base.mul(base);
            e >>= 1;
        }
        acc
    }

    /// Multiplicative inverse using Fermat’s little theorem: a^(254) = a^(-1)
    pub fn inv(self) -> Self {
        assert!(self.0 != 0, "attempted inversion of zero");
        self.pow(254)
    }
}

/// Convenience traits for +, -, *, / operators.
use core::ops::{Add, Mul, Sub, Div};

impl Add for GF256 {
    type Output = GF256;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { self.add_internal(rhs) }
}
impl Sub for GF256 {
    type Output = GF256;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { self.add_internal(rhs) } // same as addition
}
impl Mul for GF256 {
    type Output = GF256;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output { self.mul_internal(rhs) }
}
impl Div for GF256 {
    type Output = GF256;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output { self.mul_internal(rhs.inv()) }
}

/// A share: (x, y_bytes) where x ∈ GF(256) and y_bytes is 1‑to‑1 with secret length.
#[derive(Clone, Debug)]
pub struct Share {
    pub x: GF256,
    pub y: Vec<u8>,
}

impl fmt::Display for Share {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Share(x={:02X}, y={:?})", self.x.0, self.y)
    }
}

/// Produce `share_count` shares of `secret`, requiring `threshold` to reconstruct.
///
/// The secret is split byte‑wise: for every byte we build an independent random
/// polynomial of degree `threshold‑1` with the secret byte as the free coefficient.
pub fn split(secret: &[u8],
            threshold: usize,
            share_count: usize) -> Vec<Share> {
    assert!((1..=255).contains(&threshold));
    assert!(share_count >= threshold && share_count <= 255);

    let xs: Vec<GF256> = (1..=share_count as u8).map(GF256).collect();
    let mut shares: Vec<Share> = xs.iter()
        .map(|&x| Share { x, y: Vec::with_capacity(secret.len()) })
        .collect();

    for &secret_byte in secret {
        let mut coeffs: Vec<GF256> = Vec::with_capacity(threshold);
        coeffs.push(GF256(secret_byte));
        for _ in 1..threshold {
            coeffs.push(GF256(random::<u8>()));
        }

        for share in &mut shares {
            let mut y = GF256::ZERO;
            for &coeff in coeffs.iter().rev() {
                y = y * share.x + coeff;
            }
            share.y.push(y.0);
        }
    }
    shares
}

/// Reconstruct the secret from at least `threshold` shares using Lagrange interpolation.
pub fn reconstruct(shares: &[Share], threshold: usize) -> Vec<u8> {
    assert!(shares.len() >= threshold);
    let secret_len = shares[0].y.len();
    assert!(shares.iter().all(|s| s.y.len() == secret_len));

    let mut secret = Vec::with_capacity(secret_len);
    for byte_idx in 0..secret_len {
        let mut s = GF256::ZERO;
        for (i, share_i) in shares.iter().take(threshold).enumerate() {
            let mut num = GF256::ONE;
            let mut den = GF256::ONE;
            for (j, share_j) in shares.iter().take(threshold).enumerate() {
                if i == j { continue; }
                num = num * share_j.x;
                den = den * (share_j.x - share_i.x);
            }
            let li = num / den;
            s = s + (GF256(share_i.y[byte_idx]) * li);
        }
        secret.push(s.0);
    }
    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gf_add_mul() {
        let a = GF256(0x57);
        let b = GF256(0x83);
        assert_eq!((a + b).0, 0xD4);
        assert_eq!((a * b).0, 0xC1);
    }

    #[test]
    fn round_trip() {
        let secret = b"AB";
        let threshold = 4;
        let shares = 4;
        let parts = split(secret, threshold, shares);
        let recovered = reconstruct(&parts, threshold);
        assert_eq!(recovered, secret);
        
        println!("Secret_str: {:?}", String::from_utf8_lossy(secret));
        println!("Secret: {secret:?}");
        println!("Parts: {parts:?}");
        println!("Parts_0: {}", parts[0]);

        println!("Treshhold: {threshold:?}");
        println!("Result: {}", String::from_utf8_lossy(&recovered));
    }
}
