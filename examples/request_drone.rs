extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");             // windows
    //let mut drone: Drone = Drone::new("/dev/ttyACM0");      // linux

    if drone.is_connected() == false {
        println!("Is not connected!");
        return;
    }

    //drone.request(DeviceType::Controller, DataType::Information);
    drone.request(DeviceType::Drone, DataType::Information);

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
