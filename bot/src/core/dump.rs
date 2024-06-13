use crate::core::protos;
use protos::Lakeshire::Position;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub const DUMPFILE: &str = "C:\\Users\\lucas\\Desktop\\lsdump\\dump.txt";

pub fn dump_position (p: &Position) {
    let line = format!("{} {}\n", p.get_MapX(), p.get_MapY());
    dump_line_to_file(line);
}

fn dump_line_to_file (line: String) {
    println!("Dumping: '{}' to {}", line, DUMPFILE);
    let mut file = OpenOptions::new().write(true).append(true).open(DUMPFILE).unwrap();
    file.write_all(line.as_bytes()).unwrap();
}

pub fn read_dump_as_path() -> Vec<(f64, f64)> {
    let data = fs::read_to_string(DUMPFILE).unwrap();
    return data.split("\n").filter_map(|line| {
        let coords = line.split(" ").map(|c| c.parse::<f64>().unwrap_or(0.)).collect::<Vec<f64>>();
        if coords.len() != 2 {
            return None;
        } else {
            return Some((
                (coords[0] / 1e10).round(),
                (coords[1] / 1e10).round()
            ));
        }
    }).collect::<Vec<(f64, f64)>>();
}
