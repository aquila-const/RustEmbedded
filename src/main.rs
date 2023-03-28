#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::port::Pin;
use arduino_hal::hal::port::PB5;
use arduino_hal::port::mode::Output;

fn shuttering_blink(led_: &mut Pin<Output, PB5>, times: usize){
    (0..times).map(|i| i * 10).for_each(|f| {
        led_.toggle();
        arduino_hal::delay_ms(f as u16);
    })
}

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins_ = arduino_hal::pins!(peripherals);
    
    let mut led = pins_.d13.into_output();
    loop {
        shuttering_blink(&mut led, 29);
    }
}
