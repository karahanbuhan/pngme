use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;
use crate::Error;

#[derive(Clone)]
pub struct Chunk {
    pub length: u32,
    pub r#type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl Chunk {
    pub(crate) fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Chunk::calculate_crc(&chunk_type, &data);

        Chunk {
            length: data.len() as u32,
            r#type: chunk_type,
            data,
            crc,
        }
    }

    pub(crate) fn data_as_string(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.data)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.r#type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    const CRC32_ISO: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

    fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        Chunk::CRC32_ISO.checksum(
            &(chunk_type
                .bytes()
                .iter()
                .chain(data.iter())
                .copied()
                .collect::<Vec<u8>>()),
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let length = u32::from_be_bytes(<[u8; 4]>::try_from(&value[0..4])?);
        let r#type = ChunkType::try_from(<[u8; 4]>::try_from(&value[4..8])?)?;
        let data = value[8..value.len().saturating_sub(4)].to_vec();

        let crc = u32::from_be_bytes(<[u8; 4]>::try_from(
            &value[value.len().saturating_sub(4)..value.len()],
        )?);
        let crc = crc
            .eq(&Chunk::calculate_crc(&r#type, &data))
            .then_some(crc)
            .ok_or_else(|| Error::from("invalid crc"))?;

        Ok(Chunk {
            length: length
                .eq(&(data.len() as u32))
                .then_some(length)
                .ok_or_else(|| Error::from("invalid length"))?,
            r#type,
            data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "length: {}, type: {}, data: {:?}, crc: {}",
            &self.length, &self.r#type, &self.data, &self.crc
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::chunk_type::ChunkType;

    use super::*;

    fn chunk_data(data_length: u32, chunk_type: &[u8], message_bytes: &[u8], crc: u32) -> Vec<u8> {
        data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data = chunk_data(data_length, chunk_type, message_bytes, crc);

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length, 42);
        assert_eq!(chunk.crc, 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length, 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.r#type.to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc, 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = chunk_data(data_length, chunk_type, message_bytes, crc);

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length, 42);
        assert_eq!(chunk.r#type.to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc, 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = chunk_data(data_length, chunk_type, message_bytes, crc);

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = chunk_data(data_length, chunk_type, message_bytes, crc);

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
