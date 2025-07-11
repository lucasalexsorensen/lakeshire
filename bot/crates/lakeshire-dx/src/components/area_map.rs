use dioxus::prelude::*;

include!(concat!(env!("OUT_DIR"), "/areas_assets.rs"));

#[derive(PartialEq, Props, Clone)]
pub struct AreaMapProps {
    pub area_id: usize,
    pub player_pos: (f32, f32),
    pub player_facing: f32,
}

#[component]
pub fn AreaMap(props: AreaMapProps) -> Element {
    let area_asset = AREA_MAPS.get(&props.area_id);
    rsx! {
        if let Some(area_asset) = area_asset {
            PlayerMarker { pos: props.player_pos, facing: props.player_facing }
            img { src: *area_asset, class: "w-full h-full rounded-md" }
        } else {
            "Area map not found for ID = {props.area_id:?}"
        }

    }
}

#[derive(PartialEq, Props, Clone)]
pub struct PlayerMarkerProps {
    pub pos: (f32, f32),
    pub facing: f32,
}

#[component]
pub fn PlayerMarker(props: PlayerMarkerProps) -> Element {
    let (left_perc, top_perc) = (props.pos.0 * 100.0, props.pos.1 * 100.0);
    let dot_style = format!("left: {}%; top: {}%;", left_perc, top_perc);

    // facing is in radians from [0,2Pi], where 0 = north
    let facing_deg = props.facing * 180.0 / std::f32::consts::PI - 90.0;

    // Position stick so its left end is at the dot's center
    // dot is w-2 h-2 (8px), stick is w-4 h-1 (16px x 4px)
    // offset by half the dot size to center the stick's start point
    let stick_style = format!(
        "left: calc({}% + 4px); top: calc({}% + 2px); transform-origin: left center; transform: rotate({}deg);",
        left_perc, top_perc, facing_deg
    );

    rsx! {
        // the dot
        div {
            class: "absolute w-2 h-2 bg-red-500 rounded-full",
            style: dot_style,
        }
        // the stick
        div {
            class: "absolute w-4 h-1 bg-red-500",
            style: stick_style,
        }
    }
}
