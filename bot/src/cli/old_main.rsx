extern crate scrap;
extern crate protobuf;
extern crate enigo;
use scrap::{Capturer, Display};
use enigo::{Enigo};
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;
use std::thread;
use std::fs::File;

use lakeshire::lib::util::{hello_world};
// mod protos;
// use protos::Lakeshire;
// use protobuf::{parse_from_bytes};
// use std::collections::HashSet;
// mod lib;
// use lib::util;
// use lib::movement;
// use lib::dump;


fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().unwrap();
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let w = capturer.width();
    let h = capturer.height();

    // important stuff
    let mut enigo = Enigo::new();
    let mut keys = <HashSet<LsKey>>::new();

    loop {
        let buffer;
        match capturer.frame() {
            Ok(b) => {
                buffer = b;
            },
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        let bottomleft_offset = buffer.len() - w*4;

        if !(buffer[bottomleft_offset+2] == 199 && buffer[bottomleft_offset+1] == 99) {
            println!("NOT IN GAME!");
            thread::sleep(one_second);
            continue;
        }

        let cell_size = buffer[bottomleft_offset] as usize;

        let reqpixels_offset = bottomleft_offset + 4*cell_size;
        let reqpixels2 = buffer[reqpixels_offset] as u16;
        let reqpixels1 = buffer[reqpixels_offset + 1] as u16;

        let reqpixels: u16 = (reqpixels2 << 8) + reqpixels1;

        let square_dims = (reqpixels as f32).sqrt().ceil() as u32;

        let remainder_offset = reqpixels_offset + 2;
        let channel_remainder_last_pixel = match buffer[remainder_offset] {
            0 => 3,
            x => x
        };

        let mut pixel_array = Vec::new();

        for i in 2..reqpixels {
            let row = (i as f32 / square_dims as f32).floor() as u32;
            let col = (i as u32) % square_dims;

            let mut offset = bottomleft_offset;

            offset -=  w*4*cell_size*row as usize;
            offset += (4*(cell_size as u32)*col) as usize;

            if i != (reqpixels-1) {
                pixel_array.push(buffer[offset + 2] as u8);
                pixel_array.push(buffer[offset + 1] as u8);
                pixel_array.push(buffer[offset] as u8);
            } else {
                for i in 0..(channel_remainder_last_pixel) {
                    pixel_array.push(buffer[offset + (2 - i) as usize] as u8);
                }
            }

        }

        let r: Lakeshire::StructuredMessage;
        match parse_from_bytes::<Lakeshire::StructuredMessage>(&pixel_array) {
            Ok(msg) => r = msg,
            Err(_) => {
                println!("decoding failed!!");
                continue;
            }
        }

        // cool info for user
        util::print_all_info(&r);

        // DETERMINE WHAT TO DO IN CURRENT STATE

        // if r.get_BotState() == Lakeshire::BotState::Running {
        //     println!("Moving towards path node #{}", current_path_index);
        //     let result = movement::mv(&mut enigo, &r, &mut keys, &path[current_path_index]);
        //     if result {
        //         if current_path_index < path.len() - 1 {
        //             println!("Moving to next node!");
        //             current_path_index += 1;
        //         } else {
        //             println!("ARRIVED AT FINAL DESTINATION OF PATH!!");
        //         }

        //     }


        // } else if r.get_BotState() == Lakeshire::BotState::DumpPos {
        //     dump::dump_position(r.get_Player().get_PosInfo());
        //     thread::sleep(Duration::from_millis(1500));
        // }

        // SLEEP UNTIL NEXT STATE
        // thread::sleep(Duration::from_millis(500));
    }
}
