extern crate serialport;


fn main() {
    let mut vec_serialport: Vec<String> = Vec::new();

    println!("Available Ports: {:?}", serialport::available_ports());

    match serialport::available_ports() {
        Ok(vec_sp_info) => {
            println!("Length: {:?}", vec_sp_info.len());
            for sp_info in vec_sp_info {
                println!("{:?} / {:?}", sp_info.port_name, sp_info.port_type);
                vec_serialport.push(sp_info.port_name);
            }
        },
        _ => {}
    }
}

