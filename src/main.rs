mod constants;
mod dal;
use dal::{DataAccessLayer, Page};

#[tokio::main]
async fn main() {
    let pgsize = page_size::get();
    let dal = DataAccessLayer::new("test.db", pgsize as u64).await;
    let mut dal = match dal {
        Ok(dal) => dal,
        Err(e) => {
            eprintln!("DataAccessLayerError: {e}");
            return;
        }
    };
    let mut p = Page::new(&dal);
    p.num = dal.free_list.get_next_page();
    "TEST DATA"
        .as_bytes()
        .into_iter()
        .for_each(|by| p.data.push(*by));

    if let Err(e) = dal.write_page(&mut p).await {
        eprintln!("DataAccessLayerError: Couldn't persist the data to the disk. \n Details: {e}")
    };
}
