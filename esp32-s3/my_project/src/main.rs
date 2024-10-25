#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::{self, CpuClock}, delay::Delay, prelude::*};

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    //esp_println::println!("teste");// logger::init_logger_from_env();

    let max = CpuClock::max();
    let default = CpuClock::default();
    let used = clock::Clocks::get().cpu_clock.to_MHz();

    esp_println::println!("{:?}", max);
    esp_println::println!("{:?}", default);
    esp_println::println!("{:?}", used);

    loop {
        delay.delay(500.millis());
    }
}

