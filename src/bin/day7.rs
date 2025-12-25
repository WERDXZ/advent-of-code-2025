#![feature(portable_simd)]
use std::io::{BufRead, stdin};
use std::simd::u64x4;

#[derive(Clone, Copy)]
struct M256(u64x4);

// Bitwise ops - SIMD handles these natively!
impl std::ops::BitAnd for M256 {
    type Output = M256;
    fn bitand(self, rhs: Self) -> Self::Output {
        M256(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for M256 {
    type Output = M256;
    fn bitor(self, rhs: Self) -> Self::Output {
        M256(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for M256 {
    type Output = M256;
    fn bitxor(self, rhs: Self) -> Self::Output {
        M256(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for M256 {
    type Output = M256;
    fn not(self) -> Self::Output {
        M256(!self.0)
    }
}

// Shifts - need manual carry across lanes
impl std::ops::Shl<u32> for M256 {
    type Output = M256;

    fn shl(self, rhs: u32) -> Self::Output {
        let [a, b, c, d] = self.0.to_array();
        // Shift each lane, carry top bits to next lane
        M256(u64x4::from_array([
            a << rhs,
            (b << rhs) | (a >> (64 - rhs)),
            (c << rhs) | (b >> (64 - rhs)),
            (d << rhs) | (c >> (64 - rhs)),
        ]))
    }
}

impl std::ops::Shr<u32> for M256 {
    type Output = M256;

    fn shr(self, rhs: u32) -> Self::Output {
        let [a, b, c, d] = self.0.to_array();
        // Shift each lane, carry bottom bits to previous lane
        M256(u64x4::from_array([
            (a >> rhs) | (b << (64 - rhs)),
            (b >> rhs) | (c << (64 - rhs)),
            (c >> rhs) | (d << (64 - rhs)),
            d >> rhs,
        ]))
    }
}

impl M256 {
    const ZERO: Self = M256(u64x4::from_array([0, 0, 0, 0]));

    fn with_bit(pos: usize) -> Self {
        let mut arr = [0u64; 4];
        arr[pos / 64] = 1u64 << (pos % 64);
        M256(u64x4::from_array(arr))
    }

    fn count_ones(&self) -> u32 {
        let arr = self.0.to_array();
        arr.iter().map(|x| x.count_ones()).sum()
    }
}

fn main() {
    let simds = stdin().lock().lines().map_while(Result::ok).step_by(2).map(|s| {
        s.chars().enumerate().fold(M256::ZERO, |mut acc, (col, ch)| {
            match ch {
                'S' | '^' => acc = acc | M256::with_bit(col),
                _ => {}
            }
            acc
        })
    }).collect::<Vec<M256>>();
    
    let res = simds[1..].iter().fold((simds[0], 0u32), |(acc, sum), v| {
        let hits = acc & *v;
        let split_left = hits << 1;
        let split_right = hits >> 1;
        let pass_through = acc & !*v;
        // (split_left | split_right | pass_through, sum + ((!acc) & (split_left | split_right | pass_through)).count_ones())
        (split_left | split_right | pass_through, sum + hits.count_ones())
    });

    println!("{}", res.1);
}
