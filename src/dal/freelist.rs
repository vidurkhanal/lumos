use crate::constants::PAGE_NUM_SIZE;
use tokio::io::Result;

use super::page::PageNumber;
// MetaPage is the maximum pgnum that is used by the db for its own purposes. For now, only page 0 is used as the
// header page. It means all other page numbers can be used.

#[derive(Debug)]
pub struct FreeList {
    // Holds the maximum page allocated. maxPage*PageSize = fileSize
    max_page: PageNumber,
    // Pages that were previouslly allocated but are now free
    released_pages: Vec<PageNumber>,
}

impl FreeList {
    pub fn new() -> Self {
        Self {
            max_page: 0,
            released_pages: Vec::new(),
        }
    }

    pub fn get_next_page(&mut self) -> PageNumber {
        // If possible, fetch pages first from the released pages.
        // Else, increase the maximum page
        match self.released_pages.pop() {
            Some(page_id) => page_id,
            None => {
                self.max_page += 1;
                self.max_page
            }
        }
    }

    pub fn release_page(&mut self, page_id: PageNumber) {
        self.released_pages.push(page_id)
    }

    pub fn serialize(&mut self, buf: &mut Vec<u8>) {
        let mut pos: usize = 0;
        let max_page_bytes = bytemuck::bytes_of(&self.max_page);
        buf[pos..pos + PAGE_NUM_SIZE as usize].copy_from_slice(max_page_bytes);
        pos += PAGE_NUM_SIZE as usize;

        let released_pages_count = self.released_pages.len();
        let released_page_count_bytes = bytemuck::bytes_of(&released_pages_count);
        buf[pos..pos + PAGE_NUM_SIZE as usize].copy_from_slice(released_page_count_bytes);
        pos += PAGE_NUM_SIZE as usize;

        for page in &self.released_pages {
            buf[pos..pos + PAGE_NUM_SIZE as usize].copy_from_slice(bytemuck::bytes_of(page));
            pos += PAGE_NUM_SIZE as usize;
        }
    }

    pub fn deserialize(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        let mut pos = 0;
        self.max_page =
            bytemuck::from_bytes::<PageNumber>(&buf[pos..pos + PAGE_NUM_SIZE as usize]).to_owned();
        pos += PAGE_NUM_SIZE as usize;

        let released_pages_count =
            bytemuck::from_bytes::<PageNumber>(&buf[pos..pos + PAGE_NUM_SIZE as usize]).to_owned();
        pos += PAGE_NUM_SIZE as usize;

        for i in 0..released_pages_count {
            self.released_pages.push(
                bytemuck::from_bytes::<PageNumber>(&buf[pos..pos + PAGE_NUM_SIZE as usize])
                    .to_owned(),
            );
            pos += PAGE_NUM_SIZE as usize;
        }
        Ok(())
    }
}
