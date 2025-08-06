use anyhow::Error;

/// Slices a &[u8] to &[u8; 4] starting from the first_idx
pub fn slice_4_bytes(value: &[u8], first_idx: usize) -> Result<[u8; 4], Error> {
    Ok(<[u8; 4]>::try_from(&value[first_idx..first_idx + 4])?)
}
