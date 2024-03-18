use iced::{
    Application, Settings,
};

mod app;
use app::App;

fn main() -> iced::Result {
    App::run(Settings::default())
}

