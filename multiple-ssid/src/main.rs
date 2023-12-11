use esp_idf_svc::sys::{ESP_ERR_NVS_NO_FREE_PAGES, ESP_ERR_NVS_NEW_VERSION_FOUND};
use esp_idf_sys::{self, wifi_interface_t_WIFI_IF_AP, wifi_ps_type_t_WIFI_PS_NONE};

#[macro_export]

macro_rules! error_check {
    ($x:expr) => {
        esp_idf_sys::EspError::convert($x)
    };

    ($x:expr, $y:expr) => {
        esp_idf_sys::EspError::convert($x).expect($y);
        log::info!("No error in {}", $y);
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
        log::info!("nothing happens inside here");
        //There is no EEPROM on ESP32, just NVS which is non volatile key value store with different
        // data types.
        let ret = esp_idf_sys::nvs_flash_init();
        if ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND {
            if let Ok(_) = error_check!(esp_idf_sys::nvs_flash_erase()) {
                error_check!(esp_idf_sys::nvs_flash_init(),"can't init nvs");
            }
        }

            // it is intermediary between an IO driver and a network stack
            esp_idf_sys::netif_init();
            error_check!(esp_idf_sys::esp_event_loop_create_default(),"Error initializing Event Loop");

            // esp_idf_sys::EspError::convert();
            esp_idf_sys::esp_netif_create_default_wifi_ap();

            error_check!(esp_idf_sys::esp_wifi_init(&esp_idf_sys::wifi_init_config_t{
                 #[cfg(esp_idf_version_major = "4")]
                event_handler: Some(esp_idf_sys::esp_event_send_internal),
                osi_funcs: &mut esp_idf_sys::g_wifi_osi_funcs,
                wpa_crypto_funcs: esp_idf_sys::g_wifi_default_wpa_crypto_funcs,
                static_rx_buf_num: esp_idf_sys::CONFIG_ESP32_WIFI_STATIC_RX_BUFFER_NUM as _,
                dynamic_rx_buf_num: esp_idf_sys::CONFIG_ESP32_WIFI_DYNAMIC_RX_BUFFER_NUM as _,
                tx_buf_type: esp_idf_sys::CONFIG_ESP32_WIFI_TX_BUFFER_TYPE as _,
                static_tx_buf_num: esp_idf_sys::WIFI_STATIC_TX_BUFFER_NUM as _,
                dynamic_tx_buf_num: esp_idf_sys::WIFI_DYNAMIC_TX_BUFFER_NUM as _,
                csi_enable: esp_idf_sys::WIFI_CSI_ENABLED as _,
                ampdu_rx_enable: esp_idf_sys::WIFI_AMPDU_RX_ENABLED as _,
                ampdu_tx_enable: esp_idf_sys::WIFI_AMPDU_TX_ENABLED as _,
                amsdu_tx_enable: esp_idf_sys::WIFI_AMSDU_TX_ENABLED as _,
                nvs_enable: esp_idf_sys::WIFI_NVS_ENABLED as _,
                nano_enable: esp_idf_sys::WIFI_NANO_FORMAT_ENABLED as _,
                rx_ba_win: esp_idf_sys::WIFI_DEFAULT_RX_BA_WIN as _,
                wifi_task_core_id: esp_idf_sys::WIFI_TASK_CORE_ID as _,
                beacon_max_len: esp_idf_sys::WIFI_SOFTAP_BEACON_MAX_LEN as _,
                mgmt_sbuf_num: esp_idf_sys::WIFI_MGMT_SBUF_NUM as _,
                feature_caps: esp_idf_sys::g_wifi_feature_caps,
                sta_disconnected_pm: esp_idf_sys::WIFI_STA_DISCONNECTED_PM_ENABLED != 0,

                espnow_max_encrypt_num: esp_idf_sys::CONFIG_ESP_WIFI_ESPNOW_MAX_ENCRYPT_NUM as i32,
                magic: esp_idf_sys::WIFI_INIT_CONFIG_MAGIC as _,
                ..Default::default()
            }), "Error initializing the wifi");
            error_check!(esp_idf_sys::esp_wifi_set_storage(esp_idf_sys::wifi_storage_t_WIFI_STORAGE_RAM),"Error setting RAM as WiFi Storage");

            error_check!(esp_idf_sys::esp_wifi_set_mode(esp_idf_sys::wifi_mode_t_WIFI_MODE_AP), "Can't set wifi mode to AP");


            let mut ap_config = esp_idf_sys::wifi_config_t {
                ap: esp_idf_sys::wifi_ap_config_t {
                    ssid: "voidash".as_bytes().try_into().unwrap(),
                    password: "hsadiov789".as_bytes().try_into().unwrap(),
                    ssid_len: 0,
                    channel: 1,
                    authmode: esp_idf_sys::wifi_auth_mode_t_WIFI_AUTH_WPA2_PSK,
                    // ssid_hidden: 0,
                    // max_connection: 4,
                    // beacon_interval: 60000,
                    ..Default::default()
                }
            };

            error_check!(esp_idf_sys::esp_wifi_set_config(wifi_interface_t_WIFI_IF_AP, &mut ap_config), "can't assign configuration");
            error_check!(esp_idf_sys::esp_wifi_start(), "unable to start");
            error_check!(esp_idf_sys::esp_wifi_set_ps(wifi_ps_type_t_WIFI_PS_NONE), "Error when setting power save mode to None");



    }


    log::info!("Hello this is initialized!");

}
