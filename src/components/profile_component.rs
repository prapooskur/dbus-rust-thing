
use std::error::Error;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, RelmWidgetExt, AsyncComponentSender};
use relm4::component::AsyncComponent;
use relm4::component::AsyncComponentParts;
use relm4::gtk::traits::WidgetExt;
use relm4::gtk::prelude::{ToggleButtonExt};
use zbus::export::futures_util::StreamExt;
use crate::dbus_proxy::profile_proxy::ProfileProxy;

pub async fn setprofile(profile: String) -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::system().await.unwrap();

    let proxy = ProfileProxy::new(&connection).await.unwrap();
    let reply = proxy.set_active_profile(&profile).await;

    match reply {
        Ok(()) => println!("Succeeded! {:?}", reply.unwrap()),
        Err(err) => eprintln!("Error calling next_profile: {:?}", err),
    }

    Ok(())
}

pub async fn getprofile() -> PowerProfile {
    let connection = zbus::Connection::system().await.unwrap();

    let proxy = ProfileProxy::new(&connection).await.unwrap();
    let current_profile = proxy.active_profile().await.unwrap();
    return match current_profile.as_str() {
        "Quiet" => PowerProfile::Quiet,
        "Balanced" => PowerProfile::Balanced,
        "Performance" => PowerProfile::Performance,
        // fall back to balanced
        _ => PowerProfile::Balanced,
    }
}

#[derive(Debug)]
pub enum PowerProfile {
    Quiet,
    Balanced,
    Performance,
}

pub struct PowerModel {
    profile: PowerProfile,
}

#[derive(Debug)]
pub enum PowerMsg {
    SetQuiet,
    SetBalanced,
    SetPerformance,
    NotifyProfile(String),
}

#[relm4::component(async,pub)]
impl AsyncComponent for PowerModel {
    type Init = PowerProfile;

    type Input = PowerMsg;
    type Output = ();

    type CommandOutput = ();

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
                        set_label: &format!("Platform Profile: "),
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
                        #[watch]
                        set_active: matches!(model.profile, PowerProfile::Quiet),
                        connect_clicked[sender] => move |_| {
                            sender.input(PowerMsg::SetQuiet);
                        },
                    },

                    append: balanced_button = &gtk::ToggleButton {
                        set_label: "Balanced",
                        set_group: Some(&power_button),
                        #[watch]
                        set_active: matches!(model.profile, PowerProfile::Balanced),
                        connect_clicked[sender] => move |_| {
                            sender.input(PowerMsg::SetBalanced);
                        },
                    },

                    append: performance_button = &gtk::ToggleButton {
                        set_label: "Performance",
                        set_group: Some(&power_button),
                        #[watch]
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
    async fn init(
        profile: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = PowerModel { profile };

        // Insert the macro code generation here
        let widgets = view_output!();



        tokio::spawn(async move {
            // update profile when another app changes it
            let conn = zbus::Connection::system().await.unwrap();
            let proxy = ProfileProxy::new(&conn).await.unwrap();
            let mut profile_changed = proxy.receive_notify_profile().await.unwrap();
            //println!("Listening for notify_profile signals...");

            let mut profile_changed = profile_changed;
            while let Some(signal) = profile_changed.next().await {
                let change = signal.args().unwrap().profile.to_string();
                //println!("{:?}", change);
                sender.input(PowerMsg::NotifyProfile(change));
            }
        });


        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>, _root: &Self::Root) {
        match msg {
            PowerMsg::SetQuiet => {
                setprofile("Quiet".to_string()).await.unwrap();
            }
            PowerMsg::SetBalanced => {
                setprofile("Balanced".to_string()).await.unwrap();
            }
            PowerMsg::SetPerformance => {
                setprofile("Performance".to_string()).await.unwrap();
            }
            PowerMsg::NotifyProfile(profile) => {
                match profile.as_str() {
                    "Quiet" => self.profile = PowerProfile::Quiet,
                    "Balanced" => self.profile = PowerProfile::Balanced,
                    "Performance" => self.profile = PowerProfile::Performance,
                    // fall back to doing nothing
                    _ => { }
                }
            }
        }
    }
}