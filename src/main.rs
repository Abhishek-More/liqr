use clap::Parser;
use local_ip_address::local_ip;
use qr2term::print_qr;

#[derive(Parser)]
struct Cli {
    #[clap(default_value_t=5500, value_parser=clap::value_parser!(i32).range(0...65535))]
    port: i32,

    #[clap(short, long)]
    url: Option<String>,
}

fn main() {
    let args: Cli = Cli::parse(); 
    let url;
    let arg_url = args.url.unwrap_or("".to_string());


    if arg_url == "" {
        let ip = local_ip().unwrap();
        url = format!("http://{}:{}", ip, args.port);
    } else {
        url = arg_url;
    }

    //Clear the terminal and display the QR code
    print! ("\x1B[2J\x1B[1;1H");
    println!("QR Code for {}", url); 

    print_qr(url).unwrap();
}
