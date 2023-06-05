use super::DataAccessLayer;

pub type PageNumber = u64;

#[derive(Debug)]
pub struct Page {
    pub num: PageNumber,
    pub data: Vec<u8>,
}

impl Page {
    pub fn new(d: &DataAccessLayer) -> Self {
        return Page {
            num: 0,
            data: Vec::with_capacity(d.page_size as usize),
        };
    }
}
