#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp32c3_hal::{IO, clock::ClockControl, embassy, peripherals::Peripherals, prelude::*};
use esp32c3_hal::gpio::{AnyPin, Output, PushPull};
use esp_backtrace as _;
use esp_println::print;
// use embassy_futures::yield_now;

#[embassy_executor::task]
async fn one_second_task(mut led: AnyPin<Output<PushPull>>) {
  let mut count = 0;
    loop {
        esp_println::println!("Spawn Task Count: {}", count);
        count += 1;

        led.set_high().unwrap();
        Timer::after(Duration::from_millis(1_000)).await;
        led.set_low().unwrap();
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    
    // Initialize esp32_hal things as embassy things
    embassy::init(
        &clocks,
        esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    // Take IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led4 = io.pins.gpio4.into_push_pull_output().degrade();

    spawner.spawn(one_second_task(led4)).unwrap();

    // This line is for Wokwi only so that the console output is formatted correctly
    print!("\x1b[20h");

    let mut count = 0;
    loop {
        esp_println::println!("Main Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(5_000)).await;
    }
}