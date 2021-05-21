#[macro_use]
extern crate lazy_static;

fn main(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run()).unwrap();
}

async fn run() -> Result<(), Box<dyn std::error::Error>>{
    manga::run().await?;
    Ok(())
}

mod manga;
mod tui;