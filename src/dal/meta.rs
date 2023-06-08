extern crate bytemuck;
use std::io::Write;

use crate::dal::PageNumber;
use tokio::io::Result;

pub const META_PAGE_NUM: u64 = 0;

#[derive(Debug)]
pub struct Meta {
    pub free_list_page: PageNumber,
}

impl Meta {
    pub fn new() -> Self {
        Self { free_list_page: 0 }
    }

    pub fn serialize(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        let bytes = bytemuck::bytes_of(&mut self.free_list_page);
        buf.write_all(bytes)?;
        Ok(())
    }

    pub fn deserialize(&mut self, buf: &mut Vec<u8>) {
        self.free_list_page = bytemuck::from_bytes::<PageNumber>(buf).to_owned();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_name() {
//         let mut m = Meta { free_list_page: 23 };
//         let mut buf: Vec<u8> = Vec::new();
//         let mut buf = m.serialize(&mut buf).unwrap();
//         m.deserialize(&mut buf);
//         assert_eq!(m.free_list_page, 23);
//     }
// }
