mod line_follower;
mod gradient_follower;

mod robot;
mod menu;
#[cfg(feature = "menu")]
mod lcd;
mod pid;
mod motor;
mod settings;

use anyhow::{bail, Context, Result};
use std::time::{Duration, Instant};
use crate::robot::Robot;
use crate::settings::Settings;

macro_rules! create_arg {
    (enum $name:ident { $( $field:ident = $field_text:literal $(,)? )*}) => {
        #[derive(Debug, Clone)]
        enum $name {
            $( $field , )*
        }

        impl $name {
            fn get() -> Result<Option<$name>> {
                std::env::args()
                    .skip(1)
                    .next()
                    .map_or(Ok(None), |string| match string.as_str() {
                        $(
                            $field_text => Ok(Some($name::$field)),
                        )*
                        x => {
                            let vec = [
                                $( $field_text , )*
                            ];
                            bail!("Cannot parse argument {x:?}, we only know the following: {vec:?}")
                        },
                    })
            }
        }
    }
}

create_arg!(
    enum Arg {
        Test = "test",
    }
);

fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let arg = Arg::get()
        .context("Failed to parse arguments")?;

    println!("got arg: {arg:?}");

    return Ok(());

    let mut settings = Settings::get()?;

    let mut robot = Robot::new(&settings)
        .context("Failed to create robot")?;

    //robot.test()?;
    //return Ok(());

    let tick_time = Duration::from_millis(50);
    loop {
        let start = Instant::now();

        if robot.tick().context("Failed to tick robot")? {
            break;
        }

        let end = start.elapsed();

        if let Some(dur) = tick_time.checked_sub(end) {
            std::thread::sleep(dur)
        }
    }

    settings.write()?;

    Ok(())
}
