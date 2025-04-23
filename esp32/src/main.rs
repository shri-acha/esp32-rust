use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    let delay =  Delay::new(100);
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut led = PinDriver::output(pins.gpio2).unwrap();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    loop{
        delay.delay_ms(1000);
        led.set_high().unwrap();
        log::info!("Hello, world!");
        delay.delay_ms(1000);
        led.set_low().unwrap();
    }
}
