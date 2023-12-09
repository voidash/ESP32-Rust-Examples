use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::{wifi::EspWifi, eventloop::EspSystemEventLoop};
use esp_idf_sys as _; 
use log::*;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::gpio::*;
#[allow(unused_imports)]
use anyhow::{bail, Result};
#[allow(unused_imports)]
use core::str;
#[allow(unused_imports)]
use embedded_svc::{
    http::{client::Client, Status},
    io::Read,
};

use embedded_svc::wifi::{
    AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration
};

pub fn wifi(
        ssid: &str,
        pass: &str,
        modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
        sysloop: EspSystemEventLoop,
           ) -> Result<Box<EspWifi<'static>>>{

    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        bail!("Missing wifi name");
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }

    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    info!("Starting Wifi....");

    wifi.start()?;

    info!("Scanning...");

    let ap_infos = wifi.scan()?;
    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        info!(
                "Found Configured access point {} on channel {}", ssid, ours.channel
             );
        Some(ours.channel)
    }else {
        info!("Configured Access point {} not found during scanning, will go to unknown channel", ssid);
    None
    };

    wifi.set_configuration(&Configuration::Mixed(
            ClientConfiguration {
                ssid : ssid.into(),
                channel,
                auth_method,
                ..Default::default()
            },
            AccessPointConfiguration {
                ssid: "aptest".into(),
                channel: channel.unwrap_or(1),
                ..Default::default()
            }
            ))?;
    info!("connecting wifi...");

    wifi.connect()?;

    info!("wait for DHCP lease..");

    wifi.wait_netif_up()?;
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}, ip_info");

    Ok(Box::new(esp_wifi))
}


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

