extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use crate::core;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Yavanna");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(100, 100);

    let grid = gtk::Grid::new();
    let sleep_type_button = gtk::CheckButton::new_with_label("Timer mode");

    let slide_hours_button = gtk::SpinButton::new_with_range(0.0, 23.0, 1.0);
    let slide_minutes_button = gtk::SpinButton::new_with_range(0.0, 59.0, 1.0);

    let sleep_button = gtk::Button::new_with_label("Sleep!");
    let cancel_button = gtk::Button::new_with_label("Cancel");

    grid.attach(&sleep_type_button, 0, 0, 2, 1);
    grid.attach(&slide_hours_button, 0, 2, 5, 1);
    grid.attach(&slide_minutes_button, 0, 4, 5, 1);

    grid.attach(&sleep_button, 0, 25, 2, 1);
    grid.attach(&cancel_button, 3, 25, 2, 1);

    window.add(&grid);

    sleep_button.connect_clicked(move |_| {
        let timer = sleep_type_button.get_active();
        let hours = slide_hours_button.get_value_as_int() as u32;
        let minutes = slide_minutes_button.get_value_as_int() as u32;

        println!("Got : {}h{}min - timer is {}", hours, minutes, timer);

        if timer {
            core::sleep::after((hours * 60) + minutes)
        } else {
            core::sleep::at(hours, minutes);
        }
    });

    cancel_button.connect_clicked(move |_| {
        println!("Cancel!");
        core::sleep::cancel();
    });

    window.show_all();
}

pub fn run() {
    let application = gtk::Application::new(Some("fr.raboland.yavanna"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    let mut empty_args: Vec<String> = Vec::new();
    application.run(&mut empty_args);
}