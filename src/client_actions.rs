use diesel::prelude::*;
use cached::proc_macro::once;
use crate::model;
use std::collections::BTreeMap;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct PromotionResponse {
    cities: Vec<model::City>,
    promotion_property: Vec<AccomodationPropertyWithPhoto>
}


pub fn find_promoted_properties_and_locations(
    conn: &mut MysqlConnection,
) -> Result<PromotionResponse, diesel::result::Error> {

    Ok(fetch_promoted_properties_and_locations(conn))
}

#[once]
pub fn fetch_promoted_properties_and_locations(
    conn: &mut MysqlConnection
)-> PromotionResponse {

    let cities = city::table
        .select(model::City::as_select())
        .limit(5)
        .load(conn)
        .unwrap();


    let mut promotion_property: Vec<AccomodationPropertyWithPhoto> = Vec::with_capacity(20);
    let mut last_id = 0;
    let properties = accomodation_property::table
    .left_join(accomodation_property_photo::table)
    .select((model::AccomodationProperty::as_select(),  Option::<model::AccomodationPropertyPhoto>::as_select()))
    .load::<(model::AccomodationProperty, Option :: < model::AccomodationPropertyPhoto >) >(conn)
    .unwrap();

    for (accomodation_property, accomodation_property_photo) in properties {
        let current_id = accomodation_property.id ; 
        if last_id != current_id {
            if let Some (accomodation_property_photo_exists) = accomodation_property_photo {
                let photo = format!("https://allbookers.com/klienti/{}_{}/{}", accomodation_property.name.replace(" ", "_"), accomodation_property.id + 2654435, accomodation_property_photo_exists.file_name);

                let data = AccomodationPropertyWithPhoto {
                    self_data: accomodation_property,
                    photo: Some(photo)
                };
                promotion_property.push(data);
            }else {
                let data = AccomodationPropertyWithPhoto {
                    self_data: accomodation_property,
                    photo: None
                };
                promotion_property.push(data);
            }
        }
        last_id = current_id;
        if promotion_property.len() == 20 {
            return PromotionResponse {
                cities,
                promotion_property
            };
        }
    }


    return PromotionResponse {
        cities,
        promotion_property
    };
}

#[derive(Clone, Serialize)]
pub struct AccomodationPropertyWithPhoto {
    #[serde(flatten)]
    self_data: model::AccomodationProperty,
    photo: Option<String>,
}


pub fn search_property(
    query: String,
    conn: &mut MysqlConnection,
) -> Result<Vec<AccomodationPropertyWithPhoto>, diesel::result::Error> {
    let mut ret_value: Vec<AccomodationPropertyWithPhoto> = Vec::with_capacity(20);
    let mut last_id = 0;
    let pattern = format!("%{}%", query);
    let properties = accomodation_property::table
    .left_join(accomodation_property_photo::table)
    .filter(accomodation_property::name.like(pattern))
    .select((model::AccomodationProperty::as_select(),  Option::<model::AccomodationPropertyPhoto>::as_select()))
    .load::<(model::AccomodationProperty, Option :: < model::AccomodationPropertyPhoto >) >(conn)
    .unwrap();

    for (accomodation_property, accomodation_property_photo) in properties {
        let current_id = accomodation_property.id ; 
        if last_id != current_id {
            if let Some (accomodation_property_photo_exists) = accomodation_property_photo {
                let photo = format!("https://allbookers.com/klienti/{}_{}/{}", accomodation_property.name.replace(" ", "_"), accomodation_property.id + 2654435, accomodation_property_photo_exists.file_name);

                let data = AccomodationPropertyWithPhoto {
                    self_data: accomodation_property,
                    photo: Some(photo)
                };
                ret_value.push(data);
            }else {
                let data = AccomodationPropertyWithPhoto {
                    self_data: accomodation_property,
                    photo: None
                };
                ret_value.push(data);
            }
        }
        last_id = current_id;
        if ret_value.len() == 20 {
            return Ok(ret_value);
        }
    }
    
    Ok(ret_value)
}