mod constants;
mod dal;
use std::io::Write;

use dal::{DataAccessLayer, Page};

#[tokio::main]
async fn main() {
    // initialize db
    let dal = DataAccessLayer::new("test.db").await;
    let mut dal = match dal {
        Ok(dal) => dal,
        Err(e) => {
            eprintln!("DataAccessLayerError: {e}");
            return;
        }
    };

    // Create a new page
    let mut p = Page::new(dal.page_size);
    p.num = dal.free_list.get_next_page();
    let source = "TEST7".as_bytes();
    p.data[..source.len()].copy_from_slice(source);

    // Commit the transaction
    if let Err(e) = dal.write_page(&mut p).await {
        eprintln!("DataAccessLayerError: Couldn't persist the data to the disk. \n Details: {e}")
    };
    dal.write_free_list().unwrap();

    dal.close().await.unwrap();

    // let dal = DataAccessLayer::new("test.db").await;
    // let mut dal = match dal {
    //     Ok(dal) => dal,
    //     Err(e) => {
    //         eprintln!("DataAccessLayerError: {e}");
    //         return;
    //     }
    // };
    // let mut p = Page::new(dal.page_size);
    // p.num = dal.free_list.get_next_page();
    // p.data.write_all("DATA3".as_bytes()).unwrap();
    //
    // if let Err(e) = dal.write_page(&mut p).await {
    //     eprintln!("DataAccessLayerError: Couldn't persist the data to the disk. \n Details: {e}")
    // };
    //
    // let page_num = dal.free_list.get_next_page();
    // dal.free_list.release_page(page_num);
    //
    // dal.write_free_list().unwrap();
}
