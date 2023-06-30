mod freelist;
mod meta;
mod page;

use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, Result};

pub use page::{Page, PageNumber};

use self::freelist::FreeList;
use self::meta::{Meta, META_PAGE_NUM};

#[derive(Debug)]
pub struct DataAccessLayer {
    pub file: File,
    pub page_size: u16,
    pub free_list: FreeList,
    pub meta: Meta,
}

impl DataAccessLayer {
    pub async fn new(path: &str) -> Result<Self> {
        match fs::metadata(path).await {
            Ok(_) => {
                // println!("File exists");
                let file = OpenOptions::new().read(true).write(true).open(path).await?;
                let mut dal = DataAccessLayer {
                    file,
                    page_size: page_size::get() as u16,
                    free_list: FreeList::new(),
                    meta: Meta::new(),
                };
                dal.read_meta().await?;
                dal.read_free_list().await?;
                Ok(dal)
            }
            Err(_) => {
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path)
                    .await?;

                let mut dal = DataAccessLayer {
                    file,
                    free_list: FreeList::new(),
                    page_size: page_size::get() as u16,
                    meta: Meta::new(),
                };
                dal.meta.free_list_page = dal.free_list.get_next_page();
                dal.write_free_list().await?;
                dal.write_meta().await?;
                Ok(dal)
            }
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        self.file.flush().await?;
        Ok(())
    }

    pub async fn read_page(&mut self, page_num: PageNumber) -> Result<Page> {
        let mut p = Page::new(self.page_size);

        let offset = page_num * self.page_size as u64;
        self.file.seek(std::io::SeekFrom::Start(offset)).await?;
        self.file.read_exact(&mut p.data).await?;

        Ok(p)
    }

    pub async fn write_page(&mut self, page: &mut Page) -> Result<()> {
        let offset = page.num * self.page_size as u64;

        self.file.seek(std::io::SeekFrom::Start(offset)).await?;
        self.file.write_all(&page.data).await?;

        Ok(())
    }

    async fn write_meta(&mut self) -> Result<Page> {
        let mut p = Page::new(self.page_size);
        p.num = META_PAGE_NUM;
        self.meta.serialize(&mut p.data)?;
        self.write_page(&mut p).await?;
        Ok(p)
    }

    async fn read_meta(&mut self) -> Result<()> {
        let mut page = self.read_page(META_PAGE_NUM).await?;
        self.meta.deserialize(&mut page.data);
        Ok(())
    }

    pub async fn write_free_list(&mut self) -> Result<Page> {
        let mut p = Page::new(self.page_size);
        p.num = self.meta.free_list_page;
        self.free_list.serialize(&mut p.data);
        self.write_page(&mut p).await?;
        self.meta.free_list_page = p.num;
        Ok(p)
    }

    async fn read_free_list(&mut self) -> Result<()> {
        let mut page = self.read_page(self.meta.free_list_page).await?;
        self.free_list.deserialize(&mut page.data)?;
        Ok(())
    }
}
