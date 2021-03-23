extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone::communication::{*};
use e_drone::communication::transfer::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");

    if drone.is_connected() == false {
        return;
    }
    
    // use e_drone::communication::{*};
    drone.send(&transfer::buzzer_hz(DeviceType::Controller, 1200, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_hz_reserve(DeviceType::Controller, 1000, 100));
    drone.sleep(500);

    drone.send(&transfer::buzzer_scale(DeviceType::Controller, buzzer::Scale::C4, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::D4, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::E4, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::F4, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::G4, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::A4, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::B4, 100));
    drone.sleep(10);
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C5, 100));
    drone.sleep(2000);

    
    // use e_drone::communication::transfer::{*};
    drone.send(&buzzer_hz(DeviceType::Controller, 1000, 100));
    drone.sleep(10);
    drone.send(&buzzer_hz_reserve(DeviceType::Controller, 1200, 100));
    drone.sleep(500);

    drone.send(&buzzer_scale(DeviceType::Controller, buzzer::Scale::C5, 100));
    drone.sleep(10);
    drone.send(&buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::B4, 100));
    drone.sleep(10);
    drone.send(&buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::A4, 100));
    drone.sleep(10);
    drone.send(&buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::G4, 100));
    drone.sleep(10);
    drone.send(&buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::F4, 100));
    drone.sleep(10);
    drone.send(&buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::E4, 100));
    drone.sleep(10);
    drone.send(&buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::D4, 100));
    drone.sleep(10);
    drone.send(&buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C4, 100));
    drone.sleep(2000);
}
