use crate::core::protos;
use protos::Lakeshire;

pub fn hello_world () {
    println!("Hello, world!");
}

pub fn print_all_info (msg: &Lakeshire::GameState) {
    print!("\x1B[2J\x1B[1;1H");

    println!("State: {}", translate_bot_state(msg.get_BotState()));

    let p = msg.get_Player().get_UnitInfo();
    println!("Name:\t{}\t(level {} {})", p.get_Name(), p.get_Level(), translate_class_name(p.get_Class()));
    println!("HP:\t{} / {}", p.get_HealthCurrent(), p.get_HealthMax());
    println!("Power:\t{} / {}", p.get_PowerCurrent(), p.get_PowerMax());

    println!("\nMap position:");
    let pos = msg.get_Player().get_PosInfo();
    println!("Map ID:\t{}", pos.get_MapId());
    println!("X:\t{}\nY:\t{}\nFacing:\t{:.2}", pos.get_MapX() as f64 / 1e14, pos.get_MapY() as f64 / 1e14, pos.get_Facing() as f64 / 1e10);

    println!("\nWorld position:");
    let pos = msg.get_Player().get_PosInfo();
    println!("X:\t\t{}\nY:\t\t{}\nInstanceID:\t{:.2}", pos.get_WorldX() as f64 / 1e5, pos.get_WorldY() as f64 / 1e5, pos.get_InstanceId());

    println!("\nFlags:");
    println!("IsOutdoors:\t{}", (msg.get_Player().get_Flags() & 1) != 0);
    println!("IsMounted:\t{}", (msg.get_Player().get_Flags() & 2) != 0);
    println!("IsFlying:\t{}", (msg.get_Player().get_Flags() & 4) != 0);


    println!("\n-----------------------\n");
    let t = msg.get_Target().get_UnitInfo();
    println!("Target:\t{}\t(level {})", t.get_Name(), t.get_Level());
    println!("HP:\t{} / {}", t.get_HealthCurrent(), t.get_HealthMax());

}

pub fn translate_class_name (c: Lakeshire::Class) -> String {
    let s = match c {
        Lakeshire::Class::None => "None",
        Lakeshire::Class::Warrior => "Warrior",
        Lakeshire::Class::Paladin => "Paladin",
        Lakeshire::Class::Hunter => "Hunter",
        Lakeshire::Class::Rogue => "Rogue",
        Lakeshire::Class::Priest => "Priest",
        Lakeshire::Class::DeathKnight => "DeathKnight",
        Lakeshire::Class::Shaman => "Shaman",
        Lakeshire::Class::Mage => "Mage",
        Lakeshire::Class::Warlock => "Warlock",
        Lakeshire::Class::Monk => "Monk",
        Lakeshire::Class::Druid => "Druid"
    };
    return String::from(s);
}

pub fn translate_bot_state (c: Lakeshire::BotState) -> String {
    let s = match c {
        Lakeshire::BotState::Stopped => "Stopped",
        Lakeshire::BotState::Running => "Running",
        Lakeshire::BotState::DumpPos => "Dumping position"
    };
    return String::from(s);
}
