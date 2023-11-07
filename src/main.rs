mod line_follower;

mod robot;
mod menu;
mod pid;
mod settings;

use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use crate::robot::Robot;
use crate::settings::Settings;
use crate::robot::state::RobotState;

fn test() {
    let c = ev3dev_lang_rust::sensors::ColorSensor::get(ev3dev_lang_rust::sensors::SensorPort::In1).unwrap();

    c.set_mode_col_reflect().unwrap();

    let b = ev3dev_lang_rust::Button::new().unwrap();

    loop {
        b.process();
        if b.is_right() {
            break;
        }

        let v = c.get_color().unwrap();
        println!("{v:?}");

        std::thread::sleep(Duration::from_secs(1));
    }

}

fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    //test();

    #[cfg(target_arch = "arm")]
    // setup the fonts
    std::process::Command::new("setfont")
        .arg("/usr/share/consolefonts/Lat2-Terminus14.psf.gz")
        .status()?;

    let mut settings = Settings::get()?;

    let next_state = RobotState::get_initial()
        .context("Failed to parse command line arguments")?;

    let robot = Robot::new()
        .context("Failed to create robot")?;

    let main = || -> Result<()> {
        settings.next_state(&robot, next_state)?;

    //robot.test()?;
    //return Ok(());

        // we do 100 ticks per second
        let tick_time = Duration::from_millis(10);
        let mut n = 0;
        loop {
            let start = Instant::now();

            if settings.tick(&robot).context("Failed to tick robot")? {
                break;
            }

            let end = start.elapsed();

            if n == 0 {
                println!("tick took: {:?}", end);
            }
            n += 1;
            n %= 20;

            if let Some(dur) = tick_time.checked_sub(end) {
                std::thread::sleep(dur)
            }
        }

        Ok(())
    };

    let res = main();

    let _ = robot.left.stop();
    let _ = robot.right.stop();
    let _ = robot.top_arm.stop();

    res?;

    settings.write()?;

    Ok(())
}
