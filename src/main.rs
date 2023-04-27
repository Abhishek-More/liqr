use clap::Parser;
use local_ip_address::local_ip;
use qr2term::print_qr;

#[derive(Parser)]
struct Cli {
    #[clap(default_value_t = 5500)]
    port: i32,

    #[clap(short, long)]
    url: Option<String>,

    #[clap(short, long)]
    route: Option<String>,
}

fn main() {
    let args: Cli = Cli::parse();
    let mut url;
    let arg_url = args.url.unwrap_or("".to_string());

    if arg_url == "" {
        let ip = local_ip().unwrap();
        url = format!("http://{}:{}", ip, args.port);

        let arg_route = args.route.unwrap_or("".to_string());
        if arg_route != "" {
            url += &arg_route;
        }

    } else {
        url = arg_url;
    }

    //Clear the terminal and display the QR code
    print!("\x1B[2J\x1B[1;1H");
    println!("QR Code for {}", url);

    print_qr(url).unwrap();
}
