mod modules;

use modules::browsers;
use native_dialog::{MessageDialog, MessageType};

#[tokio::main]
async fn main() 
{
    let brws: Vec<&'static str> = browsers::get();
    if brws.len() > 1 { 
        brws.iter().for_each(|browser| {
            println!("{}", browser)
        });
    } else {
        println!("{}", "NO_SUPPORTED_BROWSER");
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Nvinium Core | NO_SUPPORTED_BROWSER")
            .set_text("You don't have either of Google Chrome or Microsoft Edge (and/or) you need to update the browser.\nMinimum browser version 94")
            .show_alert()
            .unwrap();
        return;
    }
}