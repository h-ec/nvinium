mod modules;

use modules::browsers;

#[tokio::main]
async fn main() 
{
    println!("Hello, world! From {}", "Nvinium");

    let brws: Vec<&'static str> = browsers::get();
    brws.iter().for_each(|browser| {
        println!("{}", browser)
    });
}