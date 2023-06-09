extern crate bytemuck;

use crate::{constants::PAGE_NUM_SIZE, dal::PageNumber};
use tokio::io::Result;

pub const META_PAGE_NUM: u64 = 0;

#[derive(Debug)]
pub struct Meta {
    pub free_list_page: PageNumber,
}

impl Meta {
    pub fn new() -> Self {
        Self { free_list_page: 1 }
    }

    pub fn serialize(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        let mut pos = 0;
        let bytes = bytemuck::bytes_of(&mut self.free_list_page);
        buf[pos..pos + bytes.len()].copy_from_slice(bytes);
        pos += PAGE_NUM_SIZE as usize;
        Ok(())
    }

    pub fn deserialize(&mut self, buf: &mut Vec<u8>) {
        let mut pos = 0;
        self.free_list_page =
            bytemuck::from_bytes::<PageNumber>(&buf[pos..pos + PAGE_NUM_SIZE as usize]).clone();
        // pos += PAGE_NUM_SIZE as usize;
    }
}

#[cfg(test)]
mod tests {
    use crate::dal::Page;

    use super::*;

    #[test]
    fn test_name() {
        let mut m = Meta::new();
        let mut p = Page::new(4096);
        p.num = META_PAGE_NUM;
        m.serialize(&mut p.data).unwrap();
        // println!("PAGE AFTER SERIALIZING==> {:?}", p);
        //

        let mut m = Meta::new();
        m.deserialize(&mut p.data);
        println!("AFTER deserialize ==> {:?}", m);
        assert_eq!(true, false);
    }
}
