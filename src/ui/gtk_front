extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use crate::core;

pub fn run() {
    let application = gtk::Application::new(Some("fr.raboland.yavanna"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    let mut empty_args: Vec<String> = Vec::new();
    application.run(&mut empty_args);
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Yavanna");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(200, 150);

    let main_grid = gtk::Grid::new();
    main_grid.attach(&build_quick_time(), 0, 0, 9, 1);
    main_grid.attach(&build_time_selector(), 0, 1, 5, 3);
    main_grid.attach(&build_remaining_time(), 5, 1, 4, 3);

    window.add(&main_grid);
    window.show_all();
}

fn build_quick_time() -> gtk::Grid {
    let grid = gtk::Grid::new();

    let thirty_min_button = gtk::Button::with_label("30 min");
    let one_hour_button = gtk::Button::with_label("1 hour");
    let two_hour_button = gtk::Button::with_label("2 hours");


    thirty_min_button.connect_clicked(move |_| {
        println!("Button sleep 30");
        core::sleep::after(30);
    });
    one_hour_button.connect_clicked(move |_| {
        println!("Button sleep 60");
        core::sleep::after(60);
    });
    two_hour_button.connect_clicked(move |_| {
        println!("Button sleep 120");
        core::sleep::after(2 * 60);
    });

    grid.attach(&thirty_min_button, 0, 0, 2, 1);
    grid.attach(&one_hour_button, 2, 0, 2, 1);
    grid.attach(&two_hour_button, 4, 0, 2, 1);

    grid
}


fn build_time_selector() -> gtk::Grid {
    let grid = gtk::Grid::new();

    let sleep_type_button = gtk::CheckButton::with_label("Timer");
    sleep_type_button.set_active(true);
    let slide_hours_button = gtk::SpinButton::with_range(0.0, 23.0, 1.0);
    let slide_minutes_button = gtk::SpinButton::with_range(0.0, 59.0, 1.0);
    let sleep_button = gtk::Button::with_label("Sleep!");

    grid.attach(&sleep_type_button, 0, 0, 2, 1);
    grid.attach(&slide_hours_button, 0, 2, 5, 1);
    grid.attach(&slide_minutes_button, 0, 4, 5, 1);
    grid.attach(&sleep_button, 0, 25, 2, 1);

    sleep_button.connect_clicked(move |_| {
        let timer_ticked = sleep_type_button.get_active();
        let hours = slide_hours_button.get_value_as_int() as u32;
        let minutes = slide_minutes_button.get_value_as_int() as u32;

        println!("Got : {}h{}min - timer is {}", hours, minutes, timer_ticked);

        if timer_ticked {
            core::sleep::after((hours * 60) + minutes);
        } else {
            core::sleep::at(hours, minutes);
        }
    });

    grid
}

fn build_remaining_time() -> gtk::Grid {
    let grid = gtk::Grid::new();

    // let timer_display = gtk::Label::new(Some("00h00"));
    let cancel_button = gtk::Button::with_label("Cancel");

    cancel_button.connect_clicked(move |_| {
        println!("Cancel!");
        core::sleep::cancel();
    });

    // grid.attach(&timer_display, 0, 0, 2, 1  );
    grid.attach(&cancel_button, 0, 1, 2, 1  );

    grid
}
