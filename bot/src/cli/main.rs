use lakeshire::core::screen::capture;
use capture::{GameGrabber, GrabberError, ONE_FRAME};
use lakeshire::core::util::print_all_info;
use std::thread;
use std::time::Duration;
use clap::Parser;
use lakeshire::core::dbc::DbcHandler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which display to capture
    #[arg(short, long, default_value_t = 0)]
    display: usize,
 }




fn main () {
    let dbc_handler = DbcHandler::new();

    let args = Args::parse();
    let mut grabber = GameGrabber::new(args.display);
    loop {
        let msg = match grabber.get_frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                match error {
                    GrabberError::WouldBlock => {
                        thread::sleep(ONE_FRAME);
                        continue;
                    },
                    GrabberError::InvalidControlPixel |
                    GrabberError::ProtobufDecodeError => {
                        println!("Error: {:?}", error);
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    },
                    GrabberError::Other => {
                        panic!("get_frame() produced other error");
                    }
                }
            }
        };

        print_all_info(&msg);

    }
}
