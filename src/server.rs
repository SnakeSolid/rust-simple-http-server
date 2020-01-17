use crate::options::Options;
use iron::Iron;
use staticfile::Static;

pub fn start(options: &Options) -> () {
    let handler = Static::new(options.path());
    let address = options.address();
    let port = options.port();

    println!("Listening on {}:{}...", address, port);

    match Iron::new(handler).http((address, port)) {
        Ok(_) => {}
        Err(err) => error!("Failed to start HTTP server: {}", err),
    }
}
