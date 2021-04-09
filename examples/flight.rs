extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");             // windows
    //let mut drone: Drone = Drone::new("/dev/ttyACM0");      // linux
    //let mut drone: Drone = Drone::new("/dev/serial0");      // raspberry pi uart

    if drone.is_connected() == false {
        return;
    }

    drone.request(DeviceType::Drone, DataType::Information);

    loop {
        handler(&drone.check());

        if drone.get_time_passed_from_last_transfer() > 1200 {
            break;
        }
    }

    println!("Takeoff");
    drone.takeoff();
    drone.sleep(4000);

    println!("Go Forward");
    drone.control_position(2.0, 0.0, 0.0, 1.0, 0, 0);
    drone.sleep(5000);

    println!("Landing");
    drone.landing();
    drone.sleep(4000);

    println!("Bye");
}


fn handler(data: &Data) {
    match data {
        Data::Information(information) => {
            println!("{:?}", information);
        },
        _ => {},
    }
}

