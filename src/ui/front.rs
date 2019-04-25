extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

// upgrade weak reference or return
//#[macro_export]
//macro_rules! upgrade_weak {
//    ($x:ident, $r:expr) => {{
//        match $x.upgrade() {
//            Some(o) => o,
//            None => return $r,
//        }
//    }};
//    ($x:ident) => {
//        upgrade_weak!($x, ())
//    };
//}

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Yavanna");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    

    let sleep_button = gtk::Button::new_with_label("Sleep!");
    window.add(&sleep_button);

    window.show_all();
}

pub fn run() {
    let application = gtk::Application::new("fr.raboland.yavanna",
                                            Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    let mut empty_args: Vec<String> = Vec::new();
    application.run(&mut empty_args);
}