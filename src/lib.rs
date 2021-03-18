extern crate serialport;
extern crate e_drone;


use serialport::{*};
use std::time::{Duration, Instant};

use e_drone::communication::{*};
use e_drone::communication::receiver::{*};
use e_drone::system::{*};
use e_drone::protocol::{*};


pub struct Drone
{
    pub time_start: Instant,
    pub time_transfer: Instant,
    pub time_receive: Instant,
    pub receiver: Receiver,
    pub buffer: [u8; 1024],
    pub port: Result<Box<dyn SerialPort>>,
}


impl Drone {
    pub fn new(port_name: &str) -> Drone{
        Drone{
            time_start: Instant::now(),
            time_transfer: Instant::now(),
            time_receive: Instant::now(),
            receiver: Receiver::new(),
            buffer: [0u8; 1024],
            port: serialport::new(port_name, 57_600)
                .timeout(Duration::from_millis(100))
                .open()
        }
    }

    pub fn is_connected(&mut self) -> bool
    {
        match &mut self.port {
            Ok(_port) => { true },
            _ => { false },
        }
    }

    pub fn check(&mut self) -> Data
    {
        match &mut self.port {
            Ok(port) => {
                let length_read = &port.read(&mut self.buffer);
                match length_read {
                    Ok(len) => {
                        if *len > 0 {
                            self.receiver.push_slice(&self.buffer[..*len]);
                        }

                        if let messaging::State::Loaded = self.receiver.check()
                        {
                            self.receiver.clear();
                            self.time_receive = Instant::now();

                            return handler::check(self.receiver.get_header(), self.receiver.get_data())
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        Data::None
    }

    pub fn get_time_passed_from_start(&self) -> u128
    {
        self.time_start.elapsed().as_millis()
    }

    pub fn get_time_passed_from_last_transfer(&self) -> u128
    {
        self.time_transfer.elapsed().as_millis()
    }

    pub fn get_time_passed_from_last_receive(&self) -> u128
    {
        self.time_receive.elapsed().as_millis()
    }


    pub fn sleep(&self, time_sleep: u128)
    {
        let time_start = Instant::now();

        loop {
            if time_start.elapsed().as_millis() > time_sleep {
                break;
            }

            if time_start.elapsed().as_millis() > 1000 * 60 * 60 {
                break;
            }
        }
    }


    // -- Transfer ----------------------------------------------------------------------------------
    pub fn send(&mut self, slice_data: &[u8])
    {
        match &mut self.port {
            Ok(port) => { match port.write(slice_data) {
                Ok(_len) => { self.time_transfer = Instant::now(); },
                _ => {},
            } },
            _ => {},
        }
    }


    pub fn request(&mut self, device_type: DeviceType, data_type: DataType)
    {
        self.send(&transfer::request(device_type, data_type));
    }

}
