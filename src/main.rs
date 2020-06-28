use std::io::{Read, Write};
use std::slice;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut devices = ftd2xx::scan_devices()?;
    println!("{} Devices Detected", devices.len());
    for (index, device) in devices.iter_mut().enumerate() {
        println!("{}: {}", index, device.get_description());
        println!("Serial Number: {}", device.get_serial_number());
        device.open()?;
        device.set_baud_rate(115200)?;
        println!("BitMode: {}", device.get_bitmode()?);
        let program_data = device.query_program_data()?;
        println!("Manufacturer: {}", program_data.get_manufacturer());
        println!("Manufacturer ID: {}", program_data.get_manufacturer_id());
        println!("Description: {}", program_data.get_description());
        println!("Serial Number: {}", program_data.get_serial_number());
        println!("Program Data: {}", program_data);
        for char_value in 'A'..'Z' {
            device.write_all(&[char_value as u8])?;

            let mut read_value: u8 = 0;
            device.read_exact(slice::from_mut(&mut read_value))?;

            let match_string = if char_value == read_value as char {
                "✔️"
            } else {
                "❌"
            };
            println!(
                "Sent: {} -> Received: {} [{}]",
                char_value, read_value as char, match_string
            );
        }
        device.close()?;
    }
    Ok(())
}