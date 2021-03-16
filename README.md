# e_drone_sp
Rust library for BYROBOT drones.

* Tested
  - Windows 10


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

use e_drone::base::system::{*};
use e_drone::protocol::{*};
use e_drone_sp::{*};


fn main() {
    let mut drone: Drone = Drone::new("COM75");

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


