pub mod core;
pub mod ui;

fn main() {
    #[cfg(feature = "iced_f")]
        ui::iced_front::run();
    #[cfg(feature = "gtk_f")]
        ui::gtk_front::run();
}
