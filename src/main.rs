use crate::application::Application;

mod application;

fn main() {
    let mut cli = Application::new();
    cli.run();
}
