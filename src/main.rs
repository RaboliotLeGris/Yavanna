mod ui {
    pub mod front;
}

mod core {
    pub mod sleep;
}

fn main() {
    ui::front::run();
}
