# e_drone_sp
Rust library for BYROBOT drones.

* Tested
  - Windows 10 (x64)
  - Ubuntu linux 20.04 (x64)

- Support
  * [E-Drone](http://dev.byrobot.co.kr/documents/kr/products/e_drone/)
  * [Coding Drone](http://dev.byrobot.co.kr/documents/kr/products/coding_drone/)
  * [Battle Drone](http://dev.byrobot.co.kr/documents/kr/products/battle_drone/)


<br>
<br>


## Example

### Cargo.toml
```
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


