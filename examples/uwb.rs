extern crate e_drone_sp;

use e_drone::protocol::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM4");             // windows
    //let mut drone: Drone = Drone::new("/dev/ttyUSB0");      // linux

    if drone.is_connected() == false {
        println!("Device is NOT Connected!");
        return;
    }

    //drone.set_show_debug_message(true);

    loop {
        handler(&drone.check());

        if drone.get_time_passed_from_last_transfer() > 3600 {
            break;
        }
    }
}


fn handler(data: &Data) {
    match data {
        Data::Information(information) => {
            println!("{:?}", information);
        },
        Data::UwbPosition(position) => {
            println!("X:{}, Y:{}, Z:{}, Error:{}", position.x, position.y, position.z, position.error);
        },
        _ => {},
    }
}

