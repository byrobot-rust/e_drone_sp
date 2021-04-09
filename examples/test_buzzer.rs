extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");             // windows
    //let mut drone: Drone = Drone::new("/dev/ttyACM0");      // linux

    if drone.is_connected() == false {
        return;
    }
    
    //*
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C4, 3000);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::CS4, 3000);
    drone.sleep(1200);
    drone.buzzer_stop(DeviceType::Controller);
    drone.sleep(3200);
    // */

    /*
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::CS4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::D4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::DS4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::E4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::F4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::FS4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::G4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::GS4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::A4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::AS4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::B4, 3000);
    drone.sleep(3200);
    drone.buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C5, 3000);
    drone.sleep(3200);
    // */
    
    /*
    drone.buzzer_hz_reserve(DeviceType::Controller, 1000, 3000);
    drone.sleep(3200);
    
    drone.buzzer_hz_reserve(DeviceType::Controller, 2000, 3000);
    drone.sleep(3200);
    // */
    
    /*
    drone.buzzer_hz_reserve(DeviceType::Drone, 1000, 3000);
    drone.sleep(3200);
    
    drone.buzzer_hz_reserve(DeviceType::Drone, 2000, 3000);
    drone.sleep(3200);
    // */
}

