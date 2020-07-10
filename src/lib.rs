pub type RGBA32 = u32;
pub fn indexed_to_rgba32(input: &[u8], palette: &[RGBA32], output: &mut [RGBA32]) {
    let palette = &palette[0..256];
    for (y, index) in output.iter_mut().zip(input.iter()) {
        *y = palette[*index as usize];
    }
}

#[cfg(target_feature = "avx2")]
pub unsafe fn indexed_to_rgba32_avx2(input: &[u8], palette: &[RGBA32], output: &mut [RGBA32]) {
    use std::arch::x86_64::*;
    let palette = &palette[0..256];

    assert!(input.len() % 8 == 0);
    assert!(output.len() % 8 == 0);
    assert!(output.len() >= input.len());

    for (output_chunk, index_chunk) in output.chunks_exact_mut(8).zip(input.chunks_exact(8)) {
        let index_u8 = _mm_loadu_si128(index_chunk.as_ptr() as *const __m128i);
        let index_u32 = _mm256_cvtepu8_epi32(index_u8);
        let y = _mm256_i32gather_epi32(palette.as_ptr() as *const i32, index_u32, 4);
        _mm256_storeu_si256(output_chunk.as_mut_ptr() as *mut __m256i, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Standard;
    use rand::prelude::*;
    #[test]
    fn avx2_vs_reference() {
        let samples = 16 * 16;
        let palette = {
            let mut p = Vec::new();
            let mut i = 0u8;
            p.resize_with(256, || {
                let x = u32::from_le_bytes([i, i, i, 255]);
                i = i.wrapping_add(1);
                x
            });
            p
        };
        let rng = StdRng::seed_from_u64(0xdead_beef);
        let src: Vec<_> = rng.sample_iter(Standard).take(samples).collect();
        let mut expected = vec![0; samples];
        let mut actual = vec![0; samples];

        indexed_to_rgba32(&src, &palette, &mut expected);
        unsafe {
            indexed_to_rgba32_avx2(&src, &palette, &mut actual);
        }

        assert_eq!(expected, actual);
    }
}
