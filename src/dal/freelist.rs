use super::page::PageNumber;
// MetaPage is the maximum pgnum that is used by the db for its own purposes. For now, only page 0 is used as the
// header page. It means all other page numbers can be used.
const META_PAGE: PageNumber = 0;

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
}
