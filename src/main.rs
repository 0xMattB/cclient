use std::env;

use cclient::modules::console;
use cclient::modules::console::Color;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
	
	if args.len() == 2 {
		console::output(&format!["\ncclient ({})\n", VERSION], true, Color::Default);
		
		match cclient::run(&args[1]).await {
			Ok(_) => {},
			Err(_) => {
				console::output("Server not found", true, Color::Default);
			},
		}
	} else {
		console::output("invalid number of arguments; usage:", true, Color::Default);
		console::output("cclient.exe (ip:port)", true, Color::Default);
	}
}