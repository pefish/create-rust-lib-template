
extern crate create_rust_lib_template;

use create_rust_lib_template::module::hello;
use anyhow::{Result, Ok};

#[tokio::main]
async fn main() -> Result<()> {

    println!("{}", hello().await);
    
    
    Ok(())
}

