use serde_derive::{Deserialize, Serialize};
use crate::manifest::displayproperties::DisplayPropertiesData;

#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyDestinationDefinitionData {

    #[serde(rename = "hash")]
    pub id:u32,

    #[serde(rename = "displayProperties")]
    pub display_properties:DisplayPropertiesData,

    #[serde(rename = "placeHash")]
    pub place_hash:u32,
}