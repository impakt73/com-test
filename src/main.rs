use gumdrop::Options;
use std::io::{Read, Write};
use std::slice;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Options)]
struct AppOptions {
    #[options(command)]
    command: Option<Command>,
}

#[derive(Options)]
enum Command {
    #[options(help = "run a test suite on a device")]
    Test(TestOps),
}

#[derive(Options)]
struct TestOps {
    #[options(help = "index of the device to test")]
    index: usize,
}

fn main() -> Result<()> {
    let opts = AppOptions::parse_args_default_or_exit();

    let mut devices = ftd2xx::scan_devices()?;
    println!("{} Devices Detected", devices.len());

    if let Some(command) = opts.command {
        match command {
            Command::Test(opts) => {
                let device = &mut devices[opts.index];
                device.open()?;
                device.set_baud_rate(12000000)?;

                for value in 0..1024 as u32 {
                    let write_buffer = unsafe {
                        slice::from_raw_parts(
                            &value as *const u32 as *const u8,
                            std::mem::size_of::<u32>(),
                        )
                    };

                    let mut read_value: u32 = 0;
                    let read_buffer = unsafe {
                        slice::from_raw_parts_mut(
                            &mut read_value as *mut u32 as *mut u8,
                            std::mem::size_of::<u32>(),
                        )
                    };

                    for i in 0..4 {
                        device.write_all(&[write_buffer[i]])?;
                        device.read_exact(slice::from_mut(&mut read_buffer[i]))?;
                    }

                    let match_string = if value == read_value { "✔️" } else { "❌" };
                    println!(
                        "Sent: {} -> Received: {} [{}]",
                        value, read_value, match_string
                    );
                }

                device.close()?;
            }
        }
    } else {
        for (index, device) in devices.iter_mut().enumerate() {
            println!("{}: {}", index, device.get_description());
            println!("Serial Number: {}", device.get_serial_number());
            device.open()?;
            device.set_baud_rate(12000000)?;
            println!("BitMode: {}", device.get_bitmode()?);
            let program_data = device.query_program_data()?;
            println!("Manufacturer: {}", program_data.get_manufacturer());
            println!("Manufacturer ID: {}", program_data.get_manufacturer_id());
            println!("Description: {}", program_data.get_description());
            println!("Serial Number: {}", program_data.get_serial_number());
            println!("Program Data: {}", program_data);
            device.close()?;
        }
    }

    Ok(())
}
