extern crate multicast_dns;
extern crate libnixstore_sys;

use futures::future::Future;

mod avahi;
mod state;
mod web;
mod narinfo;


fn main() {
    use clap::{App, Arg};
    env_logger::init();

    let matches = App::new("Nix-Local-Cache-Serve-Narinfo")
                     .version("1.0")
                     .author("Andreas Rammhold <andreas@rammhold.de>")
                     .about("Serves narinfo files present in the local nix store. Only serves those files that were downloaded from the official hydra. No private builds should be leaked.")
                    .arg(Arg::with_name("port")
                    .long("port")
                    .help("The port to listen on for incoming HTTP connections.")
                    .default_value("8380"))
                  .get_matches();

    let port = matches
        .value_of("port")
        .expect("Missing the port to listen on.")
        .parse::<i16>()
        .expect("Invalid port (not numeric)");
    println!("Running on port: {}", port);

    web::serve(port).unwrap();
}
