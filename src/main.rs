use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use serialport::SerialPort;

fn main() {
    let port_name = "COM3"; // Port name may change
    let baud_rate = 115200;

    // Open the serial port
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Seri porta bağlanılamadı");

    println!("{} portuna bağlandı.", port_name);

    // Use a thread to read user inputs and write them to the serial port
    let mut port_clone = port.try_clone().expect("Seri port klonlanamadı");
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        let mut buf = [0u8; 1];
        loop {
           // Read user input
            let bytes_read = stdin_lock.read(&mut buf).unwrap_or(0);
            if bytes_read > 0 {
               // Write the read data to the serial port
                port_clone.write_all(&buf[..bytes_read]).expect("Veri seri porta yazılamadı");
                port_clone.flush().expect("Seri port flush edilemedi");
            }
        }
    });

    // Read data from the serial port and write it to the terminal
    let mut serial_buf = [0u8; 1024];
    loop {
        match port.read(&mut serial_buf) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    io::stdout().write_all(&serial_buf[..bytes_read]).expect("Veri yazılamadı");
                    io::stdout().flush().expect("Stdout flush edilemedi");
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // Ignore timeout errors
            }
            Err(e) => {
                eprintln!("Hata: {:?}", e);
                break;
            }
        }
    }
}

// Note: after running the code click on the "En" key