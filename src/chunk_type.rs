use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use anyhow::{ensure, Context, Error};

#[derive(Clone)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }
}

impl PartialEq<Self> for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes()
            .iter()
            .zip(other.bytes().iter())
            .all(|(s, o)| s == o)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Error> {
        Ok(ChunkType(value))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        ensure!(
            s.chars().all(|c| c.is_ascii_alphabetic()),
            "Invalid chunk type, chunk types must only consist of ASCII Alphabetic (a-Z) characters"
        );

        Ok(ChunkType(s.as_bytes().try_into().context(
            "Chunk types must have a size of 4 bytes (4 UTF-8 characters)",
        )?))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use core::str::from_utf8;

        write!(f, "{}", from_utf8(&self.bytes()).map_err(|_| fmt::Error)?)
    }
}

impl Debug for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.bytes())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr;

    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
