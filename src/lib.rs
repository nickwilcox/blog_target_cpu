pub type RGBA32 = u32;
pub fn indexed_to_rgba32(input: &[u8], palette: &[RGBA32], output: &mut [RGBA32]) {
    let pallete = &palette[0..256];
    for (y, index) in output.iter_mut().zip(input.iter()) {
        *y = pallete[*index as usize];
    }
}
