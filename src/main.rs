use std::env;

use cclient::modules::console;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
	
	if args.len() == 2 {
		console::output(&format!["\ncclient ({})\n", VERSION], true);
		
		match cclient::run(&args[1]).await {
			Ok(_) => {},
			Err(_) => {
				console::output("Server not found", true);
			},
		}
	} else {
		console::output("invalid number of arguments; usage:", true);
		console::output("cclient.exe (ip:port)", true);
	}
}