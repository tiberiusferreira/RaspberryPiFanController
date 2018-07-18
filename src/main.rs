extern crate sysfs_gpio;
use std::thread::sleep;
use std::time::Duration;
mod fan_control;
mod temperature_reader;
use fan_control::Fan;
#[macro_use]
extern crate log;
extern crate flexi_logger;
extern crate log_panics;
use flexi_logger::Logger;
use flexi_logger::opt_format;

fn main() {
    if !cfg!(target_os = "linux") {
        println!("This program is intended to run on a Raspberry Pi 3 running Raspbian");
        return;
    }
    Logger::with_str("info")
        .format(opt_format)
        .log_to_file()
        .directory("/tmp")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    log_panics::init();
    let fan = Fan::new();
    // Make sure it can turn on and off
    fan.turn_on_fan();
    sleep(Duration::from_millis(500));
    fan.turn_off_fan();
    info!("Started!");
    loop {
        let temperature = temperature_reader::get_temperature();
        if temperature > 65.0 {
            fan.turn_on_fan();
        }else if temperature < 50.0 {
            fan.turn_off_fan();
        }
        sleep(Duration::from_millis(500));
    }
}
