pub mod modules;

use modules::ip::IP;
use modules::console;
use std::process;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;
use std::io;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

enum ClientState {
	LogIn,
	LoggedIn,
	Disconnect,
}

pub async fn run(ip: &str) -> Result<(), Box<dyn Error>> {
	let ip = ip_get(ip);

    let mut stream = TcpStream::connect(ip.get()).await?;
    console::output("connected to server\n", true);
	
	let stdin_channel = spawn_stdin_channel();
	let mut state = ClientState::LogIn;
	let mut buf = vec![0; 1024];
	let mut allow_read = true;
	loop {

		match state {
			ClientState::LogIn => {
				if allow_read {
					if let Ok(_n) = stream.try_read(&mut buf) { // TODO: if 'n' > 0?
						allow_read = false;
						let s = std::str::from_utf8(&buf).expect("Error converting buf to string");
						let mut s = clean_string(&String::from(s));
						s = trim_null(&s);
						
						console::output(&format!["{s}"], false);
						
						if &s[0..=6] == "Invalid" {
							console::output("", true);
							state = ClientState::Disconnect;
						} else if s == "Logged in" {
							console::output("", true);
							state = ClientState::LoggedIn;
						}
						
						for i in 0..buf.len() { buf[i] = 0; }
					}
				}
				
				match stdin_channel.try_recv() {
					Ok(key) => {
						allow_read = true;
						let _ = stream.write(key.as_bytes()).await;
					},
					Err(_) => {},
				}
			},
			ClientState::LoggedIn => {
				if let Ok(_n) = stream.try_read(&mut buf) { // TODO: if 'n' > 0?
					let s = std::str::from_utf8(&buf).expect("Error converting buf to string");
					let s = clean_string(&String::from(s));

					console::output(&format!["{s}"], true);
					for i in 0..buf.len() { buf[i] = 0; }
				}
				
				match stdin_channel.try_recv() {
					Ok(key) => {
						let _ = stream.write(key.as_bytes()).await;
					},
					Err(_) => {},
				}
			},
			ClientState::Disconnect	=> {
				console::output("Disconnected", true);
				break;
			},
		}
	}
	
	Ok(())
}

fn ip_get(ip_str: &str) -> IP {
	let ip = IP::new(ip_str);
	
	match ip {
		Ok(_) => {},
		Err(e) => {
			console::output(&format!["Error: {e}"], true);
			process::exit(0);
		}
	}
	
	ip.unwrap()
}

pub fn input() -> Result<String, &'static str> {
	let mut read = String::new();
	
	if let Ok(_) = io::stdin().read_line(&mut read) {
		Ok(read)
	} else {
		Err("console::input(): error reading from input")
	}
}

fn clean_string(s: &String) -> String {
	let s = s.replace("\r", "\0");
	let s = s.replace("\n", "\0");
	s
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn trim_null(s: &String) -> String {
	String::from(s.trim_matches(char::from(0)))
}