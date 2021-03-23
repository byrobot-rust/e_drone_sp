extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");

    if drone.is_connected() == false {
        return;
    }
    
    drone.request(DeviceType::Controller, DataType::Information);
    
    loop
    {
        handler(&drone.check());
        
        if drone.get_time_passed_from_last_transfer() > 10000 {
            break;
        }
    }
}


fn handler(data: &Data) -> bool {
    match data {
        Data::None => {
            return false;
        }
        Data::Information(information) => {
            println!("{:?}", information);
        },
        Data::Button(button) => {
            println!("{:?}", button);
        }
        _ => {},
    }
    
    true
}

