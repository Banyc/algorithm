//! - source: <https://github.com/torvalds/linux/blob/master/include/linux/math.h>

#[inline]
pub fn reciprocal_scale_u8(val: u8, ep_ro: u8) -> u8 {
    ((val as u16 * ep_ro as u16) >> 8) as u8
}

#[inline]
pub fn reciprocal_scale_u16(val: u16, ep_ro: u16) -> u16 {
    ((val as u32 * ep_ro as u32) >> 16) as u16
}

#[inline]
pub fn reciprocal_scale_u32(val: u32, ep_ro: u32) -> u32 {
    ((val as u64 * ep_ro as u64) >> 32) as u32
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    #[bench]
    fn bench_reciprocal_scale_u8(b: &mut test::Bencher) {
        b.iter(|| {
            for group_size in 1..u8::MAX {
                for value in 0..u8::MAX {
                    let value = black_box(value);
                    let group_size = black_box(group_size);
                    let index = reciprocal_scale_u8(value, group_size);
                    black_box(index);
                }
            }
        })
    }

    #[bench]
    fn bench_modulo_u8(b: &mut test::Bencher) {
        b.iter(|| {
            for group_size in 1..u8::MAX {
                for value in 0..u8::MAX {
                    let value = black_box(value);
                    let group_size = black_box(group_size);
                    let index = value % group_size;
                    black_box(index);
                }
            }
        })
    }
}
