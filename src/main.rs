// Code Developed by Daniel Mattila for Testing GTK-4 and Libadwaita only
// Last Modified February 27, 2022
// Provided as is and may contain errors

use gtk4 as gtk;
use libadwaita as adw;
use adw::prelude::*;

use adw::{ActionRow, ApplicationWindow, HeaderBar};
use gtk::{Application, Box, ListBox, Orientation, Label, Switch};

fn main() {
    //Build Application
    let app = Application::builder()
        .application_id("com.Libadwaita-Example")
        .build();
    //When application is started
    app.connect_startup(|_| {
        //initialize libadwaita, then apply style manager
        adw::init();
        adw::StyleManager::set_color_scheme(&adw::StyleManager::default(), adw::ColorScheme::PreferDark);
        //adw::StyleManager::set_color_scheme(&adw::StyleManager::default(), adw::ColorScheme::PreferLight);
    });

    //Connect the UI
    app.connect_activate(build_ui);

    //Run Application
    app.run();

}

//Function to build User interface
fn build_ui(app: &Application) {
    let row = ActionRow::builder()
        .activatable(true)
        .selectable(false)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let row2 = ActionRow::builder()
        .activatable(false)
        .selectable(false)
        .title("Enable Dark Mode")
        .subtitle("This row shows a switch")
        .build();

    //Read Initial State of myswitch
    let status = adw::StyleManager::is_dark(&adw::StyleManager::default());

    let myswitch = Switch::builder()
        .valign(gtk::Align::Center)
        .state(status)
        .build();

    //On Switch flick change between light and dark
    myswitch.connect_active_notify(move |_| {
        let mystate = adw::StyleManager::is_dark(&adw::StyleManager::default());

        if mystate == false {
            adw::StyleManager::set_color_scheme(&adw::StyleManager::default(), adw::ColorScheme::PreferDark);
        }
        else {
            adw::StyleManager::set_color_scheme(&adw::StyleManager::default(), adw::ColorScheme::PreferLight);
        }
    });

    row2.add_suffix(&myswitch);


    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .css_classes(vec![String::from("boxed-list")])
        .build();

    list.append(&row);
    list.append(&row2);

    let mylabel = Label::builder()
        .label("Capitalize String")
        .margin_top(32)
        .margin_end(32)
        .margin_start(32)
        .css_classes(vec![String::from("accent")])
        .build();

    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(
        &HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("First App", ""))
            .build(),
    );
    content.append(&mylabel);
    content.append(&list);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(400)
        .default_height(400)
        // add content to window
        .content(&content)
        .build();
    window.present();
}