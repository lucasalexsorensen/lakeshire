use lakeshire::core::screen::capture;
use capture::{GameGrabber, GrabberError, ONE_FRAME};
use lakeshire::core::util::print_all_info;
use scrap::{Capturer, Display};
use std::thread;
use std::time::Duration;

fn main () {
    let display = Display::primary().unwrap();
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

    let mut grabber = GameGrabber::new(&mut capturer);

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


         // BGRA
        // println!(
        //     "B={}, G={}, R={}, A={}",
        //     buffer[bottomleft_offset],
        //     buffer[bottomleft_offset+1],
        //     buffer[bottomleft_offset+2],
        //     buffer[bottomleft_offset+3],
        // );
    }
}
