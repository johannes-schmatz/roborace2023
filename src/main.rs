mod robot;
mod menu;
mod pid;
mod settings;
mod io;
mod state;

use anyhow::{Context, Result};

use crate::robot::Robot;


fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    #[cfg(target_arch = "arm")]
    // setup the fonts
    std::process::Command::new("setfont")
        .arg("/usr/share/consolefonts/Lat2-Terminus14.psf.gz")
        .status()?;



    let bot = Robot::new().context("Failed to create robot")?;

    let mut settings = io::read().context("Failed to read the config file")?;

    let res = settings.main(&bot);

    // stop all of the motors
    let _ = bot.left.stop();
    let _ = bot.right.stop();
    let _ = bot.top_arm.stop();

    res?;

    if false {
        io::write(settings).context("Failed to write the config file")?;
    }

    Ok(())
}
