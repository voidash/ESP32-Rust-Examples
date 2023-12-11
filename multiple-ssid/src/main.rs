#![allow(unused)]

use esp_idf_svc::sys::{ESP_ERR_NVS_NO_FREE_PAGES, ESP_ERR_NVS_NEW_VERSION_FOUND};
use esp_idf_sys::{self, wifi_interface_t_WIFI_IF_AP, wifi_ps_type_t_WIFI_PS_NONE};
use std::ffi::{CString,c_void};

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

fn set_str(buf: &mut [u8], s: &str) {
   let cs = CString::new(s).unwrap(); 
   let ss = cs.as_bytes_with_nul();
   buf[..ss.len()].copy_from_slice(ss);
}


const RAW_BEACON : [u8; 57] = [
	0x80, 0x00,							// 0-1: Frame Control
	0x00, 0x00,							// 2-3: Duration
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff,				// 4-9: Destination address (broadcast)
	0xba, 0xde, 0xaf, 0xfe, 0x00, 0x06,				// 10-15: Source address
	0xba, 0xde, 0xaf, 0xfe, 0x00, 0x06,				// 16-21: BSSID
	0x00, 0x00,							// 22-23: Sequence / fragment number
	0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,			// 24-31: Timestamp (GETS OVERWRITTEN TO 0 BY HARDWARE)
	0x64, 0x00,							// 32-33: Beacon interval
	0x31, 0x04,							// 34-35: Capability info
	0x00, 0x00, 				// 36-38: SSID parameter set, 0x00:length:content
	0x01, 0x08, 0x82, 0x84,	0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24,	// 39-48: Supported rates
	0x03, 0x01, 0x01,						// 49-51: DS Parameter set, current channel 1 (= 0x01),
	0x05, 0x04, 0x01, 0x02, 0x00, 0x00,				// 52-57: Traffic Indication Map
];

const SSID : [&'static str;9] = [
    "Oh,the night is my world",
    "City light painted girl",
    "In the day nothing matters",
    "In the night, no control",
    "Through the wall something's breaking",
    "Wearing white as you're walkin",
    "Down the street of my soul",
    "You take my self, you take my self control",
    "You got me livin' only for the night",
];

const NO_OF_SSID : usize = SSID.len();
const BEACON_OFFSET: usize  = 38;
const SRCADDR_OFFSET: usize= 10;
const BSSID_OFFSET: usize= 16;
const SEQNUM_OFFSET: usize = 22;


fn spam_task() {
    let mut line = 0;
    
    let mut sequence_number = [0u32;NO_OF_SSID];
    loop {
        // insert lines into the beacon packet
        log::info!("SSID {}", SSID[line]);
        let mut new_beacon : [u8;200] = [0;200] ;
        new_beacon[0..BEACON_OFFSET].copy_from_slice(&RAW_BEACON[0..BEACON_OFFSET]);
        new_beacon[BEACON_OFFSET-1] = SSID[line].len() as u8; 
        let mut str_bytes: [u8; 64] = [0; 64];
        set_str(&mut str_bytes,SSID[line]);
        new_beacon[BEACON_OFFSET..(BEACON_OFFSET + str_bytes.len())].copy_from_slice(&str_bytes);
        new_beacon[(BEACON_OFFSET + str_bytes.len())..(BEACON_OFFSET + str_bytes.len() + RAW_BEACON.len() - BEACON_OFFSET)].copy_from_slice(&RAW_BEACON[BEACON_OFFSET..]);

        new_beacon[SRCADDR_OFFSET + 5] = line as u8;
        new_beacon[BSSID_OFFSET + 5] = line as u8;

        new_beacon[SEQNUM_OFFSET] = ((sequence_number[line] & 0x0f) << 4) as u8;
        new_beacon[SEQNUM_OFFSET + 1] = ((sequence_number[line] & 0xff0) >> 4) as u8;
        sequence_number[line] += 1;

        if sequence_number[line] > 0xfff {
            sequence_number[line] = 0;
        } 
        unsafe {esp_idf_sys::esp_wifi_80211_tx(esp_idf_sys::wifi_interface_t_WIFI_IF_AP, &new_beacon as *const [u8] as *const c_void, new_beacon.len() as i32, false)};
        line += 1;
        if line >= NO_OF_SSID {
            line = 0;
        }
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
                    ssid: [0; 32],
                    password: [0; 64],
                    ssid_len: 0,
                    channel: 1,
                    authmode: esp_idf_sys::wifi_auth_mode_t_WIFI_AUTH_WPA2_PSK,
                    // ssid_hidden: 0,
                    // max_connection: 4,
                    // beacon_interval: 60000,
                    ..Default::default()
                }
            };
            set_str(&mut ap_config.ap.ssid, "voidash");
            set_str(&mut ap_config.ap.password, "voidash123");

            error_check!(esp_idf_sys::esp_wifi_set_config(wifi_interface_t_WIFI_IF_AP, &mut ap_config), "can't assign configuration");
            error_check!(esp_idf_sys::esp_wifi_start(), "unable to start");
            error_check!(esp_idf_sys::esp_wifi_set_ps(wifi_ps_type_t_WIFI_PS_NONE), "Error when setting power save mode to None");

            spam_task();
    }


    log::info!("Hello this is initialized!");

}
