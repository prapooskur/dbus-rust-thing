mod dbus_proxy;
mod components;

use std::error::Error;
use relm4::RelmApp;
use crate::components::profile_component::{get_profile, PowerModel};


#[tokio::main]
async fn main() {
    let app = RelmApp::new("com.pras.albatross");
    let init_profile = get_profile().await;
    println!("{:?}",init_profile);
    //let init_aura =

    app.run_async::<PowerModel>(init_profile);
}