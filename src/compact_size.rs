use crate::BitcoinError;
use std::io::Read;

// copy compact size from week 3
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompactSize {
    pub value: u64,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        // Construct a CompactSize from a u64 value
        CompactSize { value }
    }

    // comment out unused code
    // pub fn to_bytes(&self) -> Vec<u8> {
    //     // Encode according to Bitcoin's CompactSize format:
    //     // [0x00–0xFC] => 1 byte
    //     // [0xFDxxxx] => 0xFD + u16 (2 bytes)
    //     // [0xFExxxxxxxx] => 0xFE + u32 (4 bytes)
    //     // [0xFFxxxxxxxxxxxxxxxx] => 0xFF + u64 (8 bytes)
    //     match self.value {
    //         // less than 253
    //         v if v < 253 => {
    //             vec![self.value as u8]
    //         }
    //         // between 253 (incl) and 256^2 (incl)
    //         v if v <= u16::MAX as u64 => {
    //             // casting locks in required byte width
    //             let size: u16 = self.value as u16;
    //             let mut v = size.to_le_bytes().to_vec();
    //             v.insert(0, 0xFD);
    //             v
    //         }
    //         // between 256^2 + 1 and 256^4 (incl)
    //         v if v <= u32::MAX as u64 => {
    //             // casting locks in required byte width
    //             let size: u32 = self.value as u32;
    //             let mut v = size.to_le_bytes().to_vec();
    //             v.insert(0, 0xFE);
    //             v
    //         }
    //         // catchall: between 256^4 + 1 and 256^8 (incl)
    //         // v if v <= u64::MAX => {
    //         _ => {
    //             // no need to cast it's u64 already
    //             let mut v = self.value.to_le_bytes().to_vec();
    //             v.insert(0, 0xFF);
    //             v
    //         }
    //     }
    // }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // Decode CompactSize, returning value and number of bytes consumed.
        // First check if bytes is empty.
        if bytes.is_empty() {
            return Err(BitcoinError::InvalidTransaction);
        }

        // need mutable slice to be able to consume with read()
        let mut reader = bytes;

        let mut prefix_byte = [0u8; 1];
        // read from bytes
        reader
            .read(&mut prefix_byte)
            .map_err(|_| BitcoinError::InvalidTransaction)?;

        // Check that enough bytes are available based on prefix.
        let prefix = prefix_byte[0];
        match prefix {
            0..0xFD => {
                // prefix is the size, so return that
                let compact = CompactSize::new(prefix as u64);
                Ok((compact, 1))
            }
            0xFD => {
                // expect 2 more bytes
                let mut buffer = [0u8; 2];
                reader
                    // throws if not enough bytes left
                    .read(&mut buffer)
                    .map_err(|_| BitcoinError::InvalidTransaction)?;

                // 2*u8 fits into u16
                let size = u16::from_le_bytes(buffer);
                let compact = CompactSize::new(size as u64);
                Ok((compact, 1 + 2))
            }
            0xFE => {
                // expect 4 more bytes
                let mut buffer = [0u8; 4];
                reader
                    // throws if not enough bytes left
                    .read(&mut buffer)
                    .map_err(|_| BitcoinError::InvalidTransaction)?;

                // 4*u8 fits into u32
                let size = u32::from_le_bytes(buffer);
                let compact = CompactSize::new(size as u64);
                Ok((compact, 1 + 4))
            }
            0xFF => {
                // expect 8 more bytes
                let mut buffer = [0u8; 8];
                reader
                    // throws if not enough bytes left
                    .read(&mut buffer)
                    .map_err(|_| BitcoinError::InvalidTransaction)?;

                // 8*u8 fits into u64
                let size = u64::from_le_bytes(buffer);
                let compact = CompactSize::new(size);
                Ok((compact, 1 + 8))
            }
        }
    }
}
