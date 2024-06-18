use diesel::prelude::*;
use cached::proc_macro::once;
use crate::model;
use std::collections::BTreeMap;

// -- Start accomodation_property

pub fn find_accomodation_property_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::AccomodationProperty>, diesel::result::Error> {
    use crate::schema::accomodation_property::dsl::*;
    
    let result = accomodation_property
        .filter(id.eq(uid))
        .first::<model::AccomodationProperty>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn find_accomodation_property_by_uid_guard(
    uid: i64,
    roles: Vec<String>,
    conn: &mut MysqlConnection,
) -> Result<Option<model::AccomodationProperty>, diesel::result::Error> {
    use crate::schema::*;
    
    let result = accomodation_property::table
        .inner_join(role::table)
        .filter(accomodation_property::id.eq(uid))
        .filter(role::authority.eq_any(roles.clone()))
        .first::<(model::AccomodationProperty,model::Role)>(conn)
        .optional()?;

    if let Some(res) = result {
        println!("found with role {:?}", roles.clone());
        Ok(Some(res.0))
    }else {
        Ok(None)
    }
}

pub fn insert_new_accomodation_property(
    data: model::NewAccomodationProperty,
    conn: &mut MysqlConnection,
) -> Result<model::NewAccomodationProperty, diesel::result::Error> {
    use crate::schema::accomodation_property::dsl::*;

    diesel::insert_into(accomodation_property).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_accomodation_property(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationProperty>, diesel::result::Error> {

    Ok(fetch_accomodation_property_list(conn))
}

#[once]
pub fn fetch_accomodation_property_list(
    conn: &mut MysqlConnection,
)->Vec<model::AccomodationProperty>{
    use crate::schema::accomodation_property::dsl::*;
    return accomodation_property
        .select(model::AccomodationProperty::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_accomodation_property_guard(
    roles: Vec<String>,
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationProperty>, diesel::result::Error> {

    Ok(fetch_accomodation_property_list_guard(roles, conn))
}

pub fn fetch_accomodation_property_list_guard(
    roles: Vec<String>,
    conn: &mut MysqlConnection,
)->Vec<model::AccomodationProperty>{
    use crate::schema::*;

    if roles.contains(&"ROLE_ADMIN".to_string()) {
        return fetch_accomodation_property_list(conn);
    }
    
    let result = accomodation_property::table
        .inner_join(role::table)
        .filter(role::authority.eq_any(roles))
        .select(model::AccomodationProperty::as_select())
        .load(conn)
        .unwrap();
    
        return result;
}

pub fn find_accomodation_property_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationPropertyWithAll>, diesel::result::Error> {

    model::AccomodationProperty::find_all_eager(conn)
}

// -- End accomodation_property

// -- Start accomodation_property_facility

pub fn find_accomodation_property_facility_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::AccomodationPropertyFacility>, diesel::result::Error> {
    use crate::schema::accomodation_property_facility::dsl::*;
    
    let result = accomodation_property_facility
        .filter(id.eq(uid))
        .first::<model::AccomodationPropertyFacility>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_accomodation_property_facility(
    data: model::NewAccomodationPropertyFacility,
    conn: &mut MysqlConnection,
) -> Result<model::NewAccomodationPropertyFacility, diesel::result::Error> {
    use crate::schema::accomodation_property_facility::dsl::*;

    diesel::insert_into(accomodation_property_facility).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_accomodation_property_facility(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationPropertyFacility>, diesel::result::Error> {

    Ok(fetch_accomodation_property_facility_list(conn))
}

#[once]
pub fn fetch_accomodation_property_facility_list(
    conn: &mut MysqlConnection,
)->Vec<model::AccomodationPropertyFacility>{
    use crate::schema::accomodation_property_facility::dsl::*;
    return accomodation_property_facility
        .select(model::AccomodationPropertyFacility::as_select())
        .load(conn)
        .unwrap();
}

// -- End accomodation_property_facility

// -- Start accomodation_property_photo

pub fn find_accomodation_property_photo_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::AccomodationPropertyPhoto>, diesel::result::Error> {
    use crate::schema::accomodation_property_photo::dsl::*;
    
    let result = accomodation_property_photo
        .filter(id.eq(uid))
        .first::<model::AccomodationPropertyPhoto>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_accomodation_property_photo(
    data: model::NewAccomodationPropertyPhoto,
    conn: &mut MysqlConnection,
) -> Result<model::NewAccomodationPropertyPhoto, diesel::result::Error> {
    use crate::schema::accomodation_property_photo::dsl::*;

    diesel::insert_into(accomodation_property_photo).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_accomodation_property_photo(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationPropertyPhoto>, diesel::result::Error> {

    Ok(fetch_accomodation_property_photo_list(conn))
}

#[once]
pub fn fetch_accomodation_property_photo_list(
    conn: &mut MysqlConnection,
)->Vec<model::AccomodationPropertyPhoto>{
    use crate::schema::accomodation_property_photo::dsl::*;
    return accomodation_property_photo
        .select(model::AccomodationPropertyPhoto::as_select())
        .load(conn)
        .unwrap();
}

// -- End accomodation_property_photo

// -- Start accomodation_property_status

pub fn find_accomodation_property_status_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::AccomodationPropertyStatu>, diesel::result::Error> {
    use crate::schema::accomodation_property_status::dsl::*;
    
    let result = accomodation_property_status
        .filter(id.eq(uid))
        .first::<model::AccomodationPropertyStatu>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_accomodation_property_status(
    data: model::NewAccomodationPropertyStatu,
    conn: &mut MysqlConnection,
) -> Result<model::NewAccomodationPropertyStatu, diesel::result::Error> {
    use crate::schema::accomodation_property_status::dsl::*;

    diesel::insert_into(accomodation_property_status).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_accomodation_property_status(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationPropertyStatu>, diesel::result::Error> {

    Ok(fetch_accomodation_property_status_list(conn))
}

#[once]
pub fn fetch_accomodation_property_status_list(
    conn: &mut MysqlConnection,
)->Vec<model::AccomodationPropertyStatu>{
    use crate::schema::accomodation_property_status::dsl::*;
    return accomodation_property_status
        .select(model::AccomodationPropertyStatu::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_accomodation_property_status_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationPropertyStatuWithAll>, diesel::result::Error> {

    model::AccomodationPropertyStatu::find_all_eager(conn)
}

// -- End accomodation_property_status

// -- Start accomodation_property_to_facility

// -- End accomodation_property_to_facility

// -- Start accomodation_property_to_language

// -- End accomodation_property_to_language

// -- Start accomodation_property_to_type

// -- End accomodation_property_to_type

// -- Start accomodation_property_type

pub fn find_accomodation_property_type_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::AccomodationPropertyType>, diesel::result::Error> {
    use crate::schema::accomodation_property_type::dsl::*;
    
    let result = accomodation_property_type
        .filter(id.eq(uid))
        .first::<model::AccomodationPropertyType>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_accomodation_property_type(
    data: model::NewAccomodationPropertyType,
    conn: &mut MysqlConnection,
) -> Result<model::NewAccomodationPropertyType, diesel::result::Error> {
    use crate::schema::accomodation_property_type::dsl::*;

    diesel::insert_into(accomodation_property_type).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_accomodation_property_type(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AccomodationPropertyType>, diesel::result::Error> {

    Ok(fetch_accomodation_property_type_list(conn))
}

#[once]
pub fn fetch_accomodation_property_type_list(
    conn: &mut MysqlConnection,
)->Vec<model::AccomodationPropertyType>{
    use crate::schema::accomodation_property_type::dsl::*;
    return accomodation_property_type
        .select(model::AccomodationPropertyType::as_select())
        .load(conn)
        .unwrap();
}

// -- End accomodation_property_type

// -- Start address

pub fn find_address_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::Addres>, diesel::result::Error> {
    use crate::schema::address::dsl::*;
    
    let result = address
        .filter(id.eq(uid))
        .first::<model::Addres>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_address(
    data: model::NewAddres,
    conn: &mut MysqlConnection,
) -> Result<model::NewAddres, diesel::result::Error> {
    use crate::schema::address::dsl::*;

    diesel::insert_into(address).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_address(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Addres>, diesel::result::Error> {

    Ok(fetch_address_list(conn))
}

#[once]
pub fn fetch_address_list(
    conn: &mut MysqlConnection,
)->Vec<model::Addres>{
    use crate::schema::address::dsl::*;
    return address
        .select(model::Addres::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_address_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::AddresWithAll>, diesel::result::Error> {

    model::Addres::find_all_eager(conn)
}

// -- End address

// -- Start city

pub fn find_city_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::City>, diesel::result::Error> {
    use crate::schema::city::dsl::*;
    
    let result = city
        .filter(id.eq(uid))
        .first::<model::City>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_city(
    data: model::NewCity,
    conn: &mut MysqlConnection,
) -> Result<model::NewCity, diesel::result::Error> {
    use crate::schema::city::dsl::*;

    diesel::insert_into(city).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_city(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::City>, diesel::result::Error> {

    Ok(fetch_city_list(conn))
}

#[once]
pub fn fetch_city_list(
    conn: &mut MysqlConnection,
)->Vec<model::City>{
    use crate::schema::city::dsl::*;
    return city
        .select(model::City::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_city_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::CityWithAll>, diesel::result::Error> {

    model::City::find_all_eager(conn)
}

// -- End city

// -- Start country

pub fn find_country_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::Country>, diesel::result::Error> {
    use crate::schema::country::dsl::*;
    
    let result = country
        .filter(id.eq(uid))
        .first::<model::Country>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_country(
    data: model::NewCountry,
    conn: &mut MysqlConnection,
) -> Result<model::NewCountry, diesel::result::Error> {
    use crate::schema::country::dsl::*;

    diesel::insert_into(country).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_country(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Country>, diesel::result::Error> {

    Ok(fetch_country_list(conn))
}

#[once]
pub fn fetch_country_list(
    conn: &mut MysqlConnection,
)->Vec<model::Country>{
    use crate::schema::country::dsl::*;
    return country
        .select(model::Country::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_country_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::CountryWithAll>, diesel::result::Error> {

    model::Country::find_all_eager(conn)
}

// -- End country

// -- Start county

pub fn find_county_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::County>, diesel::result::Error> {
    use crate::schema::county::dsl::*;
    
    let result = county
        .filter(id.eq(uid))
        .first::<model::County>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_county(
    data: model::NewCounty,
    conn: &mut MysqlConnection,
) -> Result<model::NewCounty, diesel::result::Error> {
    use crate::schema::county::dsl::*;

    diesel::insert_into(county).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_county(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::County>, diesel::result::Error> {

    Ok(fetch_county_list(conn))
}

#[once]
pub fn fetch_county_list(
    conn: &mut MysqlConnection,
)->Vec<model::County>{
    use crate::schema::county::dsl::*;
    return county
        .select(model::County::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_county_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::CountyWithAll>, diesel::result::Error> {

    model::County::find_all_eager(conn)
}

// -- End county

// -- Start currency

pub fn find_currency_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::Currency>, diesel::result::Error> {
    use crate::schema::currency::dsl::*;
    
    let result = currency
        .filter(id.eq(uid))
        .first::<model::Currency>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_currency(
    data: model::NewCurrency,
    conn: &mut MysqlConnection,
) -> Result<model::NewCurrency, diesel::result::Error> {
    use crate::schema::currency::dsl::*;

    diesel::insert_into(currency).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_currency(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Currency>, diesel::result::Error> {

    Ok(fetch_currency_list(conn))
}

#[once]
pub fn fetch_currency_list(
    conn: &mut MysqlConnection,
)->Vec<model::Currency>{
    use crate::schema::currency::dsl::*;
    return currency
        .select(model::Currency::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_currency_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::CurrencyWithAll>, diesel::result::Error> {

    model::Currency::find_all_eager(conn)
}

// -- End currency

// -- Start language

pub fn find_language_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::Language>, diesel::result::Error> {
    use crate::schema::language::dsl::*;
    
    let result = language
        .filter(id.eq(uid))
        .first::<model::Language>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_language(
    data: model::NewLanguage,
    conn: &mut MysqlConnection,
) -> Result<model::NewLanguage, diesel::result::Error> {
    use crate::schema::language::dsl::*;

    diesel::insert_into(language).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_language(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Language>, diesel::result::Error> {

    Ok(fetch_language_list(conn))
}

#[once]
pub fn fetch_language_list(
    conn: &mut MysqlConnection,
)->Vec<model::Language>{
    use crate::schema::language::dsl::*;
    return language
        .select(model::Language::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_language_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::LanguageWithAll>, diesel::result::Error> {

    model::Language::find_all_eager(conn)
}

// -- End language

// -- Start meal_plan

pub fn find_meal_plan_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::MealPlan>, diesel::result::Error> {
    use crate::schema::meal_plan::dsl::*;
    
    let result = meal_plan
        .filter(id.eq(uid))
        .first::<model::MealPlan>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_meal_plan(
    data: model::NewMealPlan,
    conn: &mut MysqlConnection,
) -> Result<model::NewMealPlan, diesel::result::Error> {
    use crate::schema::meal_plan::dsl::*;

    diesel::insert_into(meal_plan).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_meal_plan(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::MealPlan>, diesel::result::Error> {

    Ok(fetch_meal_plan_list(conn))
}

#[once]
pub fn fetch_meal_plan_list(
    conn: &mut MysqlConnection,
)->Vec<model::MealPlan>{
    use crate::schema::meal_plan::dsl::*;
    return meal_plan
        .select(model::MealPlan::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_meal_plan_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::MealPlanWithAll>, diesel::result::Error> {

    model::MealPlan::find_all_eager(conn)
}

// -- End meal_plan

// -- Start photo_tag

pub fn find_photo_tag_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::PhotoTag>, diesel::result::Error> {
    use crate::schema::photo_tag::dsl::*;
    
    let result = photo_tag
        .filter(id.eq(uid))
        .first::<model::PhotoTag>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_photo_tag(
    data: model::NewPhotoTag,
    conn: &mut MysqlConnection,
) -> Result<model::NewPhotoTag, diesel::result::Error> {
    use crate::schema::photo_tag::dsl::*;

    diesel::insert_into(photo_tag).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_photo_tag(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::PhotoTag>, diesel::result::Error> {

    Ok(fetch_photo_tag_list(conn))
}

#[once]
pub fn fetch_photo_tag_list(
    conn: &mut MysqlConnection,
)->Vec<model::PhotoTag>{
    use crate::schema::photo_tag::dsl::*;
    return photo_tag
        .select(model::PhotoTag::as_select())
        .load(conn)
        .unwrap();
}

// -- End photo_tag

// -- Start reservation

pub fn find_reservation_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::Reservation>, diesel::result::Error> {
    use crate::schema::reservation::dsl::*;
    
    let result = reservation
        .filter(id.eq(uid))
        .first::<model::Reservation>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_reservation(
    data: model::NewReservation,
    conn: &mut MysqlConnection,
) -> Result<model::NewReservation, diesel::result::Error> {
    use crate::schema::reservation::dsl::*;

    diesel::insert_into(reservation).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_reservation(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Reservation>, diesel::result::Error> {

    Ok(fetch_reservation_list(conn))
}

#[once]
pub fn fetch_reservation_list(
    conn: &mut MysqlConnection,
)->Vec<model::Reservation>{
    use crate::schema::reservation::dsl::*;
    return reservation
        .select(model::Reservation::as_select())
        .load(conn)
        .unwrap();
}

// -- End reservation

// -- Start role

pub fn find_role_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::Role>, diesel::result::Error> {
    use crate::schema::role::dsl::*;
    
    let result = role
        .filter(id.eq(uid))
        .first::<model::Role>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_role(
    data: model::NewRole,
    conn: &mut MysqlConnection,
) -> Result<model::NewRole, diesel::result::Error> {
    use crate::schema::role::dsl::*;

    diesel::insert_into(role).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_role(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Role>, diesel::result::Error> {

    Ok(fetch_role_list(conn))
}

#[once]
pub fn fetch_role_list(
    conn: &mut MysqlConnection,
)->Vec<model::Role>{
    use crate::schema::role::dsl::*;
    return role
        .select(model::Role::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_role_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoleWithAll>, diesel::result::Error> {

    model::Role::find_all_eager(conn)
}

// -- End role

// -- Start room

pub fn find_room_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::Room>, diesel::result::Error> {
    use crate::schema::room::dsl::*;
    
    let result = room
        .filter(id.eq(uid))
        .first::<model::Room>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_room(
    data: model::NewRoom,
    conn: &mut MysqlConnection,
) -> Result<model::NewRoom, diesel::result::Error> {
    use crate::schema::room::dsl::*;

    diesel::insert_into(room).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_room(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Room>, diesel::result::Error> {

    Ok(fetch_room_list(conn))
}

#[once]
pub fn fetch_room_list(
    conn: &mut MysqlConnection,
)->Vec<model::Room>{
    use crate::schema::room::dsl::*;
    return room
        .select(model::Room::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_room_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoomWithAll>, diesel::result::Error> {

    model::Room::find_all_eager(conn)
}

// -- End room

// -- Start room_facilities

pub fn find_room_facility_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::RoomFacility>, diesel::result::Error> {
    use crate::schema::room_facilities::dsl::*;
    
    let result = room_facilities
        .filter(id.eq(uid))
        .first::<model::RoomFacility>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_room_facility(
    data: model::NewRoomFacility,
    conn: &mut MysqlConnection,
) -> Result<model::NewRoomFacility, diesel::result::Error> {
    use crate::schema::room_facilities::dsl::*;

    diesel::insert_into(room_facilities).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_room_facilities(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoomFacility>, diesel::result::Error> {

    Ok(fetch_room_facility_list(conn))
}

#[once]
pub fn fetch_room_facility_list(
    conn: &mut MysqlConnection,
)->Vec<model::RoomFacility>{
    use crate::schema::room_facilities::dsl::*;
    return room_facilities
        .select(model::RoomFacility::as_select())
        .load(conn)
        .unwrap();
}

// -- End room_facility

// -- Start room_meal_plan

// -- End room_meal_plan

// -- Start room_photo

pub fn find_room_photo_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::RoomPhoto>, diesel::result::Error> {
    use crate::schema::room_photo::dsl::*;
    
    let result = room_photo
        .filter(id.eq(uid))
        .first::<model::RoomPhoto>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_room_photo(
    data: model::NewRoomPhoto,
    conn: &mut MysqlConnection,
) -> Result<model::NewRoomPhoto, diesel::result::Error> {
    use crate::schema::room_photo::dsl::*;

    diesel::insert_into(room_photo).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_room_photo(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoomPhoto>, diesel::result::Error> {

    Ok(fetch_room_photo_list(conn))
}

#[once]
pub fn fetch_room_photo_list(
    conn: &mut MysqlConnection,
)->Vec<model::RoomPhoto>{
    use crate::schema::room_photo::dsl::*;
    return room_photo
        .select(model::RoomPhoto::as_select())
        .load(conn)
        .unwrap();
}

// -- End room_photo

// -- Start room_pricing_type

pub fn find_room_pricing_type_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::RoomPricingType>, diesel::result::Error> {
    use crate::schema::room_pricing_type::dsl::*;
    
    let result = room_pricing_type
        .filter(id.eq(uid))
        .first::<model::RoomPricingType>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_room_pricing_type(
    data: model::NewRoomPricingType,
    conn: &mut MysqlConnection,
) -> Result<model::NewRoomPricingType, diesel::result::Error> {
    use crate::schema::room_pricing_type::dsl::*;

    diesel::insert_into(room_pricing_type).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_room_pricing_type(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoomPricingType>, diesel::result::Error> {

    Ok(fetch_room_pricing_type_list(conn))
}

#[once]
pub fn fetch_room_pricing_type_list(
    conn: &mut MysqlConnection,
)->Vec<model::RoomPricingType>{
    use crate::schema::room_pricing_type::dsl::*;
    return room_pricing_type
        .select(model::RoomPricingType::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_room_pricing_type_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoomPricingTypeWithAll>, diesel::result::Error> {

    model::RoomPricingType::find_all_eager(conn)
}

// -- End room_pricing_type

// -- Start room_to_facilities

// -- End room_to_facility

// -- Start room_type_availability_rates

pub fn find_room_type_availability_rate_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::RoomTypeAvailabilityRate>, diesel::result::Error> {
    use crate::schema::room_type_availability_rates::dsl::*;
    
    let result = room_type_availability_rates
        .filter(id.eq(uid))
        .first::<model::RoomTypeAvailabilityRate>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_room_type_availability_rate(
    data: model::NewRoomTypeAvailabilityRate,
    conn: &mut MysqlConnection,
) -> Result<model::NewRoomTypeAvailabilityRate, diesel::result::Error> {
    use crate::schema::room_type_availability_rates::dsl::*;

    diesel::insert_into(room_type_availability_rates).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_room_type_availability_rates(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoomTypeAvailabilityRate>, diesel::result::Error> {

    Ok(fetch_room_type_availability_rate_list(conn))
}

#[once]
pub fn fetch_room_type_availability_rate_list(
    conn: &mut MysqlConnection,
)->Vec<model::RoomTypeAvailabilityRate>{
    use crate::schema::room_type_availability_rates::dsl::*;
    return room_type_availability_rates
        .select(model::RoomTypeAvailabilityRate::as_select())
        .load(conn)
        .unwrap();
}

pub fn find_room_type_availability_rate_with_all(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::RoomTypeAvailabilityRateWithAll>, diesel::result::Error> {

    model::RoomTypeAvailabilityRate::find_all_eager(conn)
}

// -- End room_type_availability_rate

// -- Start users

pub fn find_user_by_uid(
    uid: i64,
    conn: &mut MysqlConnection,
) -> Result<Option<model::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    
    let result = users
        .filter(id.eq(uid))
        .first::<model::User>(conn)
        .optional()?;
        
    Ok(result)
}

pub fn insert_new_user(
    data: model::NewUser,
    conn: &mut MysqlConnection,
) -> Result<model::NewUser, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(&data).execute(conn)?;

    Ok(data)
}



pub fn find_users(
    conn: &mut MysqlConnection,
) -> Result<Vec<model::User>, diesel::result::Error> {

    Ok(fetch_user_list(conn))
}

#[once]
pub fn fetch_user_list(
    conn: &mut MysqlConnection,
)->Vec<model::User>{
    use crate::schema::users::dsl::*;
    return users
        .select(model::User::as_select())
        .load(conn)
        .unwrap();
}

// -- End user

