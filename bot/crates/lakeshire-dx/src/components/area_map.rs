use dioxus::prelude::*;
use lakeshire_core::serialization::FixedPositionInfo;

include!(concat!(env!("OUT_DIR"), "/areas_assets.rs"));

#[derive(PartialEq, Props, Clone)]
pub struct AreaMapProps {
    pub area_id: usize,
    pub pos_info: FixedPositionInfo,
}

#[component]
pub fn AreaMap(props: AreaMapProps) -> Element {
    let area_asset = AREA_MAPS.get(&props.area_id);
    if area_asset.is_none() {
        return rsx! { "Area map not found for ID = {props.area_id:?}" };
    }
    let area_asset = area_asset.unwrap();
    rsx! {
        div {
            class: "relative w-full h-full",
            onmousedown: move |e| {},
            onmousemove: move |e| {},
            onmouseup: move |e| {},
            PlayerMarker { pos_info: props.pos_info }
            img { src: *area_asset, class: "w-full h-full rounded-md" }
        }
    }
}

#[component]
pub fn PlayerMarker(pos_info: FixedPositionInfo) -> Element {
    let (left_perc, top_perc) = (
        pos_info.map_position.x * 100.0,
        pos_info.map_position.y * 100.0,
    );
    let dot_style = format!("left: {}%; top: {}%;", left_perc, top_perc);

    // Position stick so its left end is at the dot's center
    // dot is w-2 h-2 (8px), stick is w-4 h-1 (16px x 4px)
    // offset by half the dot size to center the stick's start point

    // the facing is 0rad at north, and increasing counterclockwise
    // which is different than CSS's rotate(), hence the negative sign
    let fixed_facing = pos_info.facing + std::f64::consts::PI / 2.0;
    let stick_style = format!(
        "left: calc({}% + 4px); top: calc({}% + 2px); transform-origin: left center; transform: rotate(-{}rad);",
        left_perc, top_perc, fixed_facing
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
