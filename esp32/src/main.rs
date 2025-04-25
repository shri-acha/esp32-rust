use std::str::FromStr;

use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use heapless::String;

use esp_idf_svc::wifi::{AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration, EspWifi, ScanMethod};
use esp_idf_svc::eventloop::EspSystemEventLoop;


fn main() {

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    let delay =  Delay::new(100);
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let w_modem = peripherals.modem;
    let w_nvs = EspDefaultNvsPartition::take().expect("[ERROR - w_nvs]"); 
    let w_sysloop = EspSystemEventLoop::take().expect("[ERROR - w_sysloop]");

    let mut w_driver = EspWifi::new(w_modem, w_sysloop, Some(w_nvs)).unwrap();    
    
    // let ssid = heapless::String::from_str("my_esp").expect("[ERROR - ssid]");
    // let password = heapless::String::from_str("password").expect("[ERROR - password]");

    // let ap_config = Configuration::AccessPoint(AccessPointConfiguration{
    //     ssid,
    //     password,
    //     ..Default::default()
    // });
   
    
    
        
    let cli_ssid = heapless::String::from_str("wifi_ssid").expect("[ERROR - cli_ssid]");
    let cli_sec = heapless::String::from_str("wifi_sec").expect("[ERROR] - cli_sec");
    let station_configuration = Configuration::Client(
        ClientConfiguration{
            ssid:cli_ssid,
            password:cli_sec,
            auth_method:AuthMethod::WPA2Personal,
            ..Default::default()
        }
    );
    // let _ = w_driver.set_configuration(&ap_config);

    w_driver.set_configuration(&station_configuration).unwrap();

    w_driver.start().unwrap();
    w_driver.connect().unwrap();

    while !w_driver.is_connected().unwrap(){
        log::info!("Not Connected! {:?}",station_configuration);
        delay.delay_ms(1000);
    }
        log::info!("Connected! {:?}",w_driver.sta_netif().get_ip_info());


    let mut led = PinDriver::output(pins.gpio2).unwrap();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    loop{
        delay.delay_ms(1000);
        led.set_high().unwrap();
        log::info!("Hello, world!");
        delay.delay_ms(500);
        led.set_low().unwrap();
    }
}
