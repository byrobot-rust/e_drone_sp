extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone::communication::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");

    if drone.is_connected() == false {
        return;
    }

    drone.request(DeviceType::Controller, DataType::Information);
    
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::C4, 200));
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::D4, 200));
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::E4, 200));
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::F4, 200));
    drone.send(&transfer::buzzer_scale_reserve(DeviceType::Controller, buzzer::Scale::G4, 200));

    loop {
        handler(&drone.check());

        if drone.get_time_passed_from_last_transfer() > 1200 {
            break;
        }
    }
}


fn handler(data: &Data) {
    match data {
        Data::Information(information) => {
            println!("{:?}", information);
        },
        _ => {},
    }
}

