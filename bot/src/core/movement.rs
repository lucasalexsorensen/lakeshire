extern crate enigo;
use std::time::Duration;
use std::thread;
use std::collections::HashSet;
use enigo::{Enigo, Key, KeyboardControllable};
use crate::core::keys;
use keys::LsKey;
use crate::core::protos::Lakeshire::*;

// CONSTS
pub const MOVE_DURATION: Duration = Duration::from_millis(10);
pub const RAD: f64 = 2. * std::f64::consts::PI;


pub fn mv (enigo: &mut Enigo, msg: &GameState, prev_keys: &mut HashSet<LsKey>, path: &(f64, f64)) -> bool {
    // first, we compute which keys to press based on current state
    let mut new_keys = <HashSet<LsKey>>::new();
    let mut result = false;

    let p = msg.get_Player().get_PosInfo();
    let x_diff = path.0 - (p.get_MapX() as f64 / 1e10).round();
    let y_diff =  path.1 - (p.get_MapY() as f64 / 1e10).round();

    let dist = ((x_diff * x_diff) + (y_diff * y_diff)).sqrt();
    println!("DIST: {}", dist);
    if dist > 5. {
        new_keys.insert(LsKey::W);

        let current_angle = p.get_Facing() as f64 / 1e10;
        let desired_angle = x_diff.atan2(y_diff) + std::f64::consts::PI;
        let diff = (current_angle - desired_angle).abs();
        println!("Angle diff: {}", diff);
        if diff > 0.1 {
            new_keys.insert(get_key_from_angle(current_angle, desired_angle));
        }
    } else {
        result = true;
    }

    println!("{:?}", new_keys);

    // then, determine key exclusivity and release unused keys
    // OLD KEYS THAT ARE NOT IN NEW KEYS => RELEASE THEM
    prev_keys.difference(&new_keys).into_iter().for_each(|k| enigo.key_up(translate_lskey_to_key(*k)));
    // NEW KEYS THAT ARE NOT IN OLD KEYS => PRESS THEM
    new_keys.difference(&prev_keys).into_iter().for_each(|k| enigo.key_down(translate_lskey_to_key(*k)));

    // then, set prev keys to new keys for next iteration
    prev_keys.clear();
    new_keys.iter().for_each(|k| {
        prev_keys.insert(*k);
        return;
    });

    return result;
}

fn get_key_from_angle (current_angle: f64, desired_angle: f64) -> LsKey {
    if (RAD - current_angle + desired_angle) % RAD < std::f64::consts::PI {
        return LsKey::A;
    } else {
        return LsKey::D;
    }
}

fn translate_lskey_to_key (lk: LsKey) -> Key {
    return match lk {
        LsKey::W => Key::Layout('w'),
        LsKey::A => Key::Layout('a'),
        LsKey::S => Key::Layout('s'),
        LsKey::D => Key::Layout('d'),
    };
}
