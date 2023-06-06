extern crate bytemuck;
use std::io::Write;

use crate::dal::PageNumber;
use tokio::io::Result;

const META_PAGE_NUMBER: u64 = 0;

pub struct Meta {
    pub free_list_page: PageNumber,
}

impl Meta {
    pub fn new() -> Self {
        Self { free_list_page: 0 }
    }

    pub fn serialize(&mut self, buf: &mut Vec<u8>) -> Result<Vec<u8>> {
        let bytes = bytemuck::bytes_of(&mut self.free_list_page);
        buf.write_all(bytes)?;
        Ok(buf.clone())
    }

    pub fn deserialize(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        self.free_list_page = bytemuck::from_bytes::<PageNumber>(buf).to_owned();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut m = Meta { free_list_page: 23 };
        let mut buf: Vec<u8> = Vec::new();
        let mut buf = m.serialize(&mut buf).unwrap();
        m.deserialize(&mut buf).unwrap();
        assert_eq!(m.free_list_page, 23);
    }
}