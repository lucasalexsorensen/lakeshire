use serde::{self, de::DeserializeOwned};

pub struct DbcHandler {
    pub uimapassignment: Vec<UiMapAssignmentRow>,
    pub uimap: Vec<UiMapRow>,
}

impl DbcHandler {
    pub fn new () -> Self {
        Self {
            uimapassignment: DbcHandler::parse_csv_file::<UiMapAssignmentRow>(
                include_str!("../../../dbc/uimapassignment.csv")
            ),
            uimap: DbcHandler::parse_csv_file::<UiMapRow>(
                include_str!("../../../dbc/uimap.csv")
            )
        }
    }

    fn parse_csv_file<T: DeserializeOwned> (file_contents: &str) -> Vec<T> {
        let mut reader = csv::Reader::from_reader(file_contents.as_bytes());
        let deser = reader.deserialize::<T>();
        deser.into_iter().map(|x| x.unwrap()).collect()
    }
}


#[derive(serde::Deserialize, Debug)]
pub struct UiMapAssignmentRow {
    #[serde(rename = "Region[0]")]
    region_0: f32,
    #[serde(rename = "Region[1]")]
    region_1: f32,
    #[serde(rename = "Region[2]")]
    region_2: f32,
    #[serde(rename = "Region[3]")]
    region_3: f32,
    #[serde(rename = "Region[4]")]
    region_4: f32,
    #[serde(rename = "Region[5]")]
    region_5: f32,
    #[serde(rename = "UiMapID")]
    ui_map_id: u32,
    #[serde(rename = "AreaID")]
    area_id: u32,
}

#[derive(serde::Deserialize, Debug)]
pub struct UiMapRow {
    #[serde(rename = "Name_lang")]
    name: String,
    #[serde(rename = "ID")]
    id: u32,
}
