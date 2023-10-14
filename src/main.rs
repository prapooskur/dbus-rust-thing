mod dbus_proxy;
use dbus_proxy::profile::ProfileProxyBlocking;

use std::error::Error;

fn setprofile_blocking(profile: String) -> Result<(), Box<dyn Error>> {
    let connection = zbus::blocking::Connection::system().unwrap();

    let proxy = ProfileProxyBlocking::new(&connection).unwrap();
    let reply = proxy.set_active_profile(&profile);

    match reply {
        Ok(()) => println!("Succeeded! {:?}", reply.unwrap()),
        Err(err) => eprintln!("Error calling next_profile: {:?}", err),
    }

    Ok(())
}

fn getprofile_blocking() -> PowerProfile {
    let connection = zbus::blocking::Connection::system().unwrap();

    let proxy = ProfileProxyBlocking::new(&connection).unwrap();
    let current_profile = proxy.active_profile().unwrap();
    return match current_profile.as_str() {
        "Quiet" => PowerProfile::Quiet,
        "Balanced" => PowerProfile::Balanced,
        "Performance" => PowerProfile::Performance,
        // fall back to balanced
        _ => PowerProfile::Balanced,
    }
}


use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::gtk::traits::WidgetExt;
use relm4::gtk::prelude::{ToggleButtonExt};

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
    SetQuiet,
    SetBalanced,
    SetPerformance,
    NotifyProfile(String),
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
            set_default_height: 75,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,
                    set_halign: gtk::Align::Start,

                    gtk::Label {
                        #[watch]
                        set_label: &format!("Platform Profile:"),
                        set_margin_all: 5,
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,
                    set_halign: gtk::Align::Center,

                    append: power_button = &gtk::ToggleButton {
                        set_label: "Quiet",
                        //set_group: Some(&power_button),
                        set_active: matches!(model.profile, PowerProfile::Quiet),
                        connect_clicked[sender] => move |_| {
                            sender.input(PowerMsg::SetQuiet);
                        },
                    },



                    gtk::ToggleButton {
                        set_label: "Balanced",
                        set_group: Some(&power_button),
                        set_active: matches!(model.profile, PowerProfile::Balanced),
                        connect_clicked[sender] => move |_| {
                            sender.input(PowerMsg::SetBalanced);
                        },
                    },

                    gtk::ToggleButton {
                        set_label: "Performance",
                        set_group: Some(&power_button),
                        set_active: matches!(model.profile, PowerProfile::Performance),
                        connect_clicked[sender] => move |_| {
                            sender.input(PowerMsg::SetPerformance);
                        },
                    },



                },



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
            PowerMsg::SetQuiet => {
                setprofile_blocking("Quiet".to_string()).unwrap();
            }
            PowerMsg::SetBalanced => {
                setprofile_blocking("Balanced".to_string()).unwrap();
            }
            PowerMsg::SetPerformance => {
                setprofile_blocking("Performance".to_string()).unwrap();
            }
            PowerMsg::NotifyProfile(profile) => {
                match profile.as_str() {
                    "Quiet" => self.profile = PowerProfile::Quiet,
                    "Balanced" => self.profile = PowerProfile::Balanced,
                    "Performance" => self.profile = PowerProfile::Performance,
                    // fall back to doing nothing
                    _ => {}
                }
            }
        }
        //self.profile = getprofile_blocking();
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    let init_profile = getprofile_blocking();
    println!("{:?}",init_profile);
    app.run::<PowerModel>(init_profile);
}