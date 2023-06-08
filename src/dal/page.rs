use super::DataAccessLayer;

pub type PageNumber = u64;

#[derive(Debug)]
pub struct Page {
    pub num: PageNumber,
    pub data: Vec<u8>,
}

impl Page {
    pub fn new(size: u16) -> Self {
        return Page {
            num: 0,
            data: vec![0u8; size.into()],
        };
    }
}
