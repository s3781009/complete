use std::error::Error;
mod dictionary;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    ui::setup_and_run()
}
