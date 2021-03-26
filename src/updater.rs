extern crate serialport;
extern crate e_drone;


use serialport::{*};
use std::{thread};
use std::time::{Duration, Instant};

use e_drone::communication::{*};
use e_drone::communication::receiver::{*};
use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone::protocol::display::{*};
use e_drone::protocol::command::{*};

use crate::Drone;


// -- StateUpdate ----------------------------------------------------------------------------------------------
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum StateUpdate {
    Ready,
    Start,
    Connect,
    CheckTarget,
    Transfer,
    Finish,
    Failure
}



// -- Updater ----------------------------------------------------------------------------------------------
pub struct Updater
{
    pub device_type: DeviceType,
    pub information: Information,
    
    pub index_block_next: u16,
    pub flag_updated: bool,
    pub flag_update_complete: bool,
    
    pub time_start: Instant,        // 시작 시간
    pub time_total: Instant,        // 전체 시간
    pub time_progress: Instant,     // 진행한 시간
    pub time_left: Instant,         // 남은 시간
}


impl Updater {
    pub fn new() -> Updater{
        Updater{
            device_type: DeviceType::Drone,
            information: Information::new(),
            index_block_next: 0,
            flag_updated: false,
            flag_update_complete: false,
            time_start: Instant::now(),
            time_total: Instant::now(),
            time_progress: Instant::now(),
            time_left: Instant::now(),
        }
    }


    pub fn update_information(&mut self, information: Information)
    {
        self.information = information;
    }

    pub fn update_update_location(&mut self, update_location: UpdateLocation)
    {
        self.index_block_next = update_location.index_block_next;
    }

    pub fn get_serialport_list() -> Vec<String>
    {
        let mut vec_serialport: Vec<String> = Vec::new();

        match serialport::available_ports() {
            Ok(vec_sp_info) => {
                for sp_info in vec_sp_info {
                    vec_serialport.push(sp_info.port_name);
                }
            },
            _ => {}
        }

        vec_serialport
    }

}
