
mod proxy_profile;
use proxy_profile::AsusDaemonProxyBlocking;

use std::error::Error;

async fn tesr() -> Result<(), Box<dyn Error>> {
    let connection = zbus::blocking::Connection::system().unwrap();

    let proxy = AsusDaemonProxyBlocking::new(&connection).unwrap();
    let reply_switch = proxy.next_profile();
    let reply_new = proxy.active_profile();

    match reply_switch {
        Ok(()) => println!("{reply_new:?}"),
        Err(err) => eprintln!("Error calling next_profile: {:?}", err),
    }

    Ok(())
}

fn nextprofile_blocking() -> Result<(), Box<dyn Error>> {
    let connection = zbus::blocking::Connection::system().unwrap();

    let proxy = AsusDaemonProxyBlocking::new(&connection).unwrap();
    let reply_switch = proxy.next_profile();
    let reply_new = proxy.active_profile();

    match reply_switch {
        Ok(()) => println!("{reply_new:?}"),
        Err(err) => eprintln!("Error calling next_profile: {:?}", err),
    }

    Ok(())
}

fn getprofile_blocking() -> PowerProfile {
    let connection = zbus::blocking::Connection::system().unwrap();

    let proxy = AsusDaemonProxyBlocking::new(&connection).unwrap();
    let current_profile = proxy.active_profile().unwrap();
    return match current_profile.as_str() {
        "Quiet" => PowerProfile::Quiet,
        "Balanced" => PowerProfile::Balanced,
        "Performance" => PowerProfile::Performance,
        _ => PowerProfile::Balanced,
    }
}


use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
enum PowerProfile {
    Quiet,
    Balanced,
    Performance,
}

struct PowerModel {
    profile: PowerProfile,
}

#[derive(Debug)]
enum PowerMsg {
    NextProfile,
}

#[relm4::component]
impl SimpleComponent for PowerModel {
    type Init = PowerProfile;

    type Input = PowerMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Albatross"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Next Profile",
                    connect_clicked[sender] => move |_| {
                        sender.input(PowerMsg::NextProfile);
                    }
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Current profile: {:?}", model.profile),
                    set_margin_all: 5,
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        profile: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PowerModel { profile };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PowerMsg::NextProfile => {
                nextprofile_blocking().unwrap();
                self.profile = getprofile_blocking();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    let init_profile = getprofile_blocking();
    app.run::<PowerModel>(init_profile);
}