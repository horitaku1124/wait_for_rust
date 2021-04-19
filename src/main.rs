use std::env;
use std::path::Path;
use std::{thread, time};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut use_tcp = false;
    let mut use_file = false;
    let mut tcp_endpoint = "".to_string();
    let mut file_name = "".to_string();

    let mut skip = false;
    for i in 1..env::args().len() {
        if skip {
            skip = false;
            continue;
        }
        let arg = match env::args().nth(i) {
            Option::Some(val) => val,
            Option::None => "".to_owned(),
        };

        if arg == "-f" {
            file_name = match env::args().nth(i + 1) {
                Option::Some(val) => val,
                Option::None => panic!("f is required"),
            };
            use_file = true;
            skip = true;
        }
        if arg == "-p" {
            tcp_endpoint = match env::args().nth(i + 1) {
                Option::Some(val) => val,
                Option::None => panic!("p is required"),
            };
            use_tcp = true;
            skip = true;
        }
        
    }

    if !use_tcp && !use_file {
        panic!("-p or -f is required");
    }


    let mut found = false;
    if use_file {
        for _i in  0..5{
            if Path::new(&file_name).exists() {
                found = true;
                break;
            } else {
                let wait = time::Duration::from_millis(1000);
                println!("waiting");
                thread::sleep(wait);
            }
        }
    }
    if use_tcp {
        for _i in  0..5{
            if let Ok(_stream) = TcpStream::connect(&tcp_endpoint) {
                found = true;
                break;
            } else {
                let wait = time::Duration::from_millis(1000);
                println!("waiting");
                thread::sleep(wait);
            }
        }
    }
    if found {
        Ok(())
    } else {
        panic!("file nor listener didn't appear");
    }
}
