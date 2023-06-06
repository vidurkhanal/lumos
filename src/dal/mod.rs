mod freelist;
mod meta;
mod page;

use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, Result};

pub use page::{Page, PageNumber};

use self::freelist::FreeList;

#[derive(Debug)]
pub struct DataAccessLayer {
    pub file: File,
    pub page_size: u64,
    pub free_list: FreeList,
}

impl DataAccessLayer {
    pub async fn new(path: &str, page_size: u64) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;
        Ok(DataAccessLayer {
            file,
            page_size,
            free_list: FreeList::new(),
        })
    }

    pub async fn close(&mut self) -> Result<()> {
        self.file.flush().await?;
        Ok(())
    }

    pub async fn read_page(&mut self, page_num: PageNumber) -> Result<Page> {
        let mut p = Page::new(self);

        let offset = page_num * self.page_size;
        self.file.seek(std::io::SeekFrom::Start(offset)).await?;
        self.file.read_exact(&mut p.data).await?;

        Ok(p)
    }

    pub async fn write_page(&mut self, page: &mut Page) -> Result<()> {
        let offset = page.num * self.page_size;

        self.file.seek(std::io::SeekFrom::Start(offset)).await?;
        self.file.write(&mut page.data).await?;

        Ok(())
    }
}
