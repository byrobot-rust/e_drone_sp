extern crate e_drone_sp;

use chrono::{Timelike, Local};

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone::protocol::display::{*};
use e_drone::communication::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");

    if drone.is_connected() == false {
        return;
    }

    drone.request(DeviceType::Controller, DataType::Information);

    drone.send(&transfer::draw_clear_all(Pixel::Black));
    drone.sleep(1000);

    drone.send(&transfer::draw_clear(10, 10, 108, 44, Pixel::White));
    drone.sleep(1000);

    drone.send(&transfer::draw_invert(5, 5, 20, 20));
    drone.sleep(1000);

    drone.send(&transfer::draw_point(30, 30, Pixel::Black));
    drone.sleep(1000);

    drone.send(&transfer::draw_line(108, 30, 20, 44, Pixel::Black, Line::Solid));
    drone.sleep(1000);

    drone.send(&transfer::draw_rect(20, 10, 90, 40, Pixel::Black, true, Line::Solid));
    drone.sleep(1000);

    drone.send(&transfer::draw_circle(64, 32, 30, Pixel::White, true));
    drone.sleep(1000);

    drone.send(&transfer::draw_string(40, 18, Font::LM5x8, Pixel::Black, String::from("HELLO WORLD")));
    drone.sleep(1000);

    loop {
        let now = Local::now();
        let str_time: String = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
    
        drone.send(&transfer::draw_string_align(0, 128, 40, Align::Center, Font::LM10x16, Pixel::White, str_time));
        drone.sleep(27);
        
        handler(&drone.check());

        if drone.get_time_passed_from_start() > 20000 {
            break;
        }
    }

    drone.send(&transfer::draw_string(40, 20, Font::LM10x16, Pixel::White, String::from(" BYE ")));
    drone.sleep(1000);
}


fn handler(data: &Data) {
    match data {
        Data::Information(information) => {
            println!("{:?}", information);
        },
        _ => {},
    }
}

