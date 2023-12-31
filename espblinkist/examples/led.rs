use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys as _; 
use log::*;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::gpio::*;


fn main() -> anyhow::Result<()>{ 
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio4)?;

    println!("Hello , Led");
    info!("Hello, world!");

    loop {
        led.set_high()?;
        FreeRtos::delay_ms(1000);

        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }
}


