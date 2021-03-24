extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone::protocol::display::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");

    if drone.is_connected() == false {
        return;
    }

    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyFlicker.into(), 200, 240, 240, 20);
    show(&mut drone, String::from("BODY FLICKER"), String::from("YELLOW"), 5);

    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyFlicker.into(), 200, 20, 240, 240);
    show(&mut drone, String::from("BODY FLICKER"), String::from("CYAN"), 5);

    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyFlicker.into(), 200, 240, 20, 240);
    show(&mut drone, String::from("BODY FLICKER"), String::from("MAGENTA"), 5);


    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyDimming.into(), 2, 240, 240, 20);
    show(&mut drone, String::from("BODY DIMMING"), String::from("YELLOW"), 5);

    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyDimming.into(), 2, 20, 240, 240);
    show(&mut drone, String::from("BODY DIMMING"), String::from("CYAN"), 5);

    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyDimming.into(), 2, 240, 20, 240);
    show(&mut drone, String::from("BODY DIMMING"), String::from("MAGENTA"), 5);


    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyRainbow.into(), 3, 0, 0, 0);
    show(&mut drone, String::from("BODY RAINBOW"), String::from(""), 12);

    drone.light_mode_color(DeviceType::Controller, light::ModeLight::BodyRainbow2.into(), 3, 0, 0, 0);
    show(&mut drone, String::from("BODY RAINBOW 2"), String::from(""), 12);

    show(&mut drone, String::from(""), String::from("BYE"), 0);
}


fn show(drone: &mut Drone, title: String, sub_title: String, time_wait_sec: i32)
{
    drone.sleep(10);
    drone.draw_clear_all(Pixel::White);

    drone.sleep(10);
    drone.draw_string_align(0, 128, 32 - 4 - 8 - 4, Align::Center, Font::LM5x8, Pixel::Black, title);

    drone.sleep(10);
    drone.draw_string_align(0, 128, 32 - 4, Align::Center, Font::LM5x8, Pixel::Black, sub_title);

    if time_wait_sec == 0 {
        return;
    }

    drone.sleep(10);
    let mut time_remain = time_wait_sec;
    loop 
    {
        drone.draw_string_align(0, 128, 32 + 4 + 4, Align::Center, Font::LM5x8, Pixel::Black, format!("  {}  ", time_remain));
        drone.sleep(1000);

        time_remain = time_remain - 1;

        if time_remain <= 0 {
            break;
        }
    }
}