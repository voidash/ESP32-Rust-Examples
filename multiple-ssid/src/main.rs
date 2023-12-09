use esp_idf_svc::sys::{ESP_ERR_NVS_NO_FREE_PAGES, ESP_ERR_NVS_NEW_VERSION_FOUND};
use esp_idf_sys;

#[macro_export]

macro_rules! error_check {
    ($x:expr) => {
        esp_idf_sys::EspError::convert($x)
    };

    ($x:expr, $y:expr) => {
        esp_idf_sys::EspError::convert($x).expect($y);
    }
}
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    // Embassy is async await for embedded  
    //
    unsafe {
        //There is no EEPROM on ESP32, just NVS which is non volatile key value store with different
        // data types.
        let mut ret = esp_idf_sys::nvs_flash_init();
        if ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND {
            if let Ok(_) = error_check!(esp_idf_sys::nvs_flash_erase()) {
                ret = esp_idf_sys::nvs_flash_init();
            }

            // it is intermediary between an IO driver and a network stack
            esp_idf_sys::netif_init();
            error_check!(esp_idf_sys::esp_event_loop_create_default(), "Couldn't create Event Loop");

            // esp_idf_sys::EspError::convert();
            esp_idf_sys::esp_netif_create_default_wifi_ap();

            error_check!(esp_idf_sys::esp_wifi_init(&esp_idf_sys::wifi_init_config_t{..Default::default()}), "Error initializing the wifi");


        }


    }


    log::info!("Hello, world!");

}
