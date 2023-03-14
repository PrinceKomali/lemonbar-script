use std::time::Duration;
use std::thread;
use std::fs;

use crate::display::{parse_battery, parse_volume, disk};
mod display;
extern "C" {
    fn get_thread_status() -> i32;
}
#[no_mangle]
pub fn start_rs() {
    let mut bat = String::new();
    let mut disk_p = String::from("Disk (0.00%)  [--------------------]");
    let mut bat_int = 0;
    let mut bat_charging = true;
   
    
    loop {
    


        println!(
            "%{{c}} {} {} {}", 
            parse_volume(),
            parse_battery(bat.clone(), bat_charging),
            disk_p
        );

        if bat_int == 150 || bat_int == 0 {
            bat_int = 1;
            disk_p = disk();
            match fs::read_to_string("/sys/class/power_supply/BAT1/capacity") {
                Ok(v) => { bat = v},
                Err(_) => {}
            }
            match fs::read_to_string("/sys/class/power_supply/BAT1/status") {
                Ok(v) => { 
                    bat_charging = v == "Charging\n"
                },
                Err(_) => {}
            }
            
        }
        bat_int += 1;
        if unsafe { get_thread_status() } == 0 { break }
        thread::sleep(Duration::from_millis(33));
        
    }
}
