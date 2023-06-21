use anyhow::Result;
use app::config::{info::APP_ID, setup};
use relm4::RelmApp;

use app::Done;

mod app;
// mod application;
// mod factories;
// mod widgets;

fn main() -> Result<()> {
	let app = RelmApp::new(APP_ID);
	setup::init()?;
	app.run_async::<Done>(());
	Ok(())
}
