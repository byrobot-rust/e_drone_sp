# e_drone_sp
Rust library for BYROBOT drones.

* Tested
  - Windows 10 (x64)
  - Ubuntu linux 20.04 (x64)
  - macOS Big Sur
  - Raspberry Pi OS (Raspberry PI Compute Module 4)

* Support
  - [E-Drone](http://dev.byrobot.co.kr/documents/kr/products/e_drone/)
  - [Coding Drone](http://dev.byrobot.co.kr/documents/kr/products/coding_drone/)
  - [Battle Drone](http://dev.byrobot.co.kr/documents/kr/products/battle_drone/)


<br>
<br>


## Example

### Cargo.toml
```toml
[dependencies]
e_drone_sp="21.*"
e_drone="21.*"
```


<br>
<br>


### main.rs
```rust
extern crate e_drone_sp;

use e_drone::system::{*};
use e_drone::protocol::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");             // windows
    //let mut drone: Drone = Drone::new("/dev/ttyACM0");      // linux

    if drone.is_connected() == false {
        return;
    }

    drone.request(DeviceType::Controller, DataType::Information);

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
```


<br>
<br>



## Examples in library

<br>

### Source code

https://github.com/byrobot-rust/e_drone_sp/tree/master/examples


<br>
<br>


### Show in github1s.com

* <a href="https://github1s.com/byrobot-rust/e_drone_sp/blob/master/examples/button.rs" target="_blank">Button</a>
* <a href="https://github1s.com/byrobot-rust/e_drone_sp/blob/master/examples/buzzer.rs" target="_blank">Buzzer</a>
* <a href="https://github1s.com/byrobot-rust/e_drone_sp/blob/master/examples/display.rs" target="_blank">Display</a>
* <a href="https://github1s.com/byrobot-rust/e_drone_sp/blob/master/examples/joystick.rs" target="_blank">Joystick</a>
* <a href="https://github1s.com/byrobot-rust/e_drone_sp/blob/master/examples/light.rs" target="_blank">Light</a>
* <a href="https://github1s.com/byrobot-rust/e_drone_sp/blob/master/examples/request.rs" target="_blank">Request</a>
* <a href="https://github1s.com/byrobot-rust/e_drone_sp/blob/master/examples/vibrator.rs" target="_blank">Vibrator</a>


<br>
<br>


### Clone Library

```
git clone https://github.com/byrobot-rust/e_drone_sp/
```


<br>
<br>


### Run
```
cargo run --example button
```
```
cargo run --example buzzer
```
```
cargo run --example display
```
```
cargo run --example joystick
```
```
cargo run --example light
```
```
cargo run --example request
```
```
cargo run --example vibrator
```

