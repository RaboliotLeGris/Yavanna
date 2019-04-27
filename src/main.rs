mod ui {
    pub mod front;
}

mod core {
    pub mod sleep;
}

fn main() {
//     println!("Hello, world!");
//     core::sleep::at(String::from("14"), String::from("00"));

    ui::front::run();
}
