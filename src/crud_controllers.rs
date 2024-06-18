use ntex::web;
use crate::model;
use crate::db;
use crate::actions;
use crate::auth_middleware::JwtAuthData;
use crate::client_actions;

// -- Start client page promotions

#[web::post("/client/promotions")]
async fn get_promotions(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let promotion_response = web::block(move || client_actions::find_promoted_properties_and_locations(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&promotion_response))
}
// -- End client page promotions


// -- Start client page promotions

#[web::post("/client/search/{search_query}")]
async fn get_search_res(
    pool: web::types::State<db::DbPool>,
    search_query: web::types::Path<String>,
) -> Result<web::HttpResponse, web::Error> {
    let search_query_i = search_query.into_inner();

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let promotion_response = web::block(move || client_actions::search_property(search_query_i, &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&promotion_response))
}
// -- End client page promotions
// -- Start accomodation_property

#[web::get("/accomodation_property/{accomodation_property_id}")]
async fn get_accomodation_property_by_id(
    auth_data: JwtAuthData,
    pool: web::types::State<db::DbPool>,
    accomodation_property_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let accomodation_property_uid = accomodation_property_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property = web::block(move || actions::find_accomodation_property_by_uid_guard(accomodation_property_uid, auth_data.roles, &mut conn)).await?;

    if let Some(accomodation_property) = accomodation_property {
        Ok(web::HttpResponse::Ok().json(&accomodation_property))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No accomodation_property found with uid: {}", accomodation_property_uid));
        Ok(res)
    }
}

#[web::get("/accomodation_property")]
async fn get_accomodation_property(
    auth_data: JwtAuthData,
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_list = web::block(move || actions::find_accomodation_property_guard(auth_data.roles, &mut conn)).await?;

    //Ok(web::HttpResponse::Ok().json(&accomodation_property_list))
    Ok(web::HttpResponse::Ok().body(bitcode::serialize(&accomodation_property_list).unwrap()))
}

#[web::get("/accomodation_property_eager/")]
async fn get_accomodation_property_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_list = web::block(move || actions::find_accomodation_property_with_all(&mut conn)).await?;

    //Ok(web::HttpResponse::Ok().json(&accomodation_property_list))
    Ok(web::HttpResponse::Ok().body(bitcode::serialize(&accomodation_property_list).unwrap()))
}

#[web::post("/accomodation_property")]
async fn add_accomodation_property(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewAccomodationProperty>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property = web::block(move || actions::insert_new_accomodation_property(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property))
}

// -- End accomodation_property

// -- Start accomodation_property_facility

#[web::get("/accomodation_property_facility/{accomodation_property_facility_id}")]
async fn get_accomodation_property_facility_by_id(
    pool: web::types::State<db::DbPool>,
    accomodation_property_facility_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let accomodation_property_facility_uid = accomodation_property_facility_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_facility = web::block(move || actions::find_accomodation_property_facility_by_uid(accomodation_property_facility_uid, &mut conn)).await?;

    if let Some(accomodation_property_facility) = accomodation_property_facility {
        Ok(web::HttpResponse::Ok().json(&accomodation_property_facility))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No accomodation_property_facility found with uid: {}", accomodation_property_facility_uid));
        Ok(res)
    }
}

#[web::get("/accomodation_property_facility")]
async fn get_accomodation_property_facility(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_facility_list = web::block(move || crate::model::AccomodationPropertyFacility::find_all_with_join_property(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_facility_list))
}

// #[web::get("/accomodation_property_facility/selected/{accomodation_property_id}")]
// async fn get_accomodation_property_facility(
//     pool: web::types::State<db::DbPool>,
//     accomodation_property_id: web::types::Path<i64>,
// ) -> Result<web::HttpResponse, web::Error> {
//     let accomodation_property_id = accomodation_property_id.into_inner();
//     let mut conn = pool.get().expect("couldn't get db connection from pool");

//     // use web::block to offload blocking Diesel code without blocking server thread
//     let accomodation_property_facility_list = web::block(move || actions::find_accomodation_property_facility(&mut conn)).await?;

//     Ok(web::HttpResponse::Ok().json(&accomodation_property_facility_list))
// }


#[web::post("/accomodation_property_facility")]
async fn add_accomodation_property_facility(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewAccomodationPropertyFacility>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_facility = web::block(move || actions::insert_new_accomodation_property_facility(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_facility))
}

// -- End accomodation_property_facility

// -- Start accomodation_property_photo

#[web::get("/accomodation_property_photo/{accomodation_property_photo_id}")]
async fn get_accomodation_property_photo_by_id(
    pool: web::types::State<db::DbPool>,
    accomodation_property_photo_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let accomodation_property_photo_uid = accomodation_property_photo_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_photo = web::block(move || actions::find_accomodation_property_photo_by_uid(accomodation_property_photo_uid, &mut conn)).await?;

    if let Some(accomodation_property_photo) = accomodation_property_photo {
        Ok(web::HttpResponse::Ok().json(&accomodation_property_photo))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No accomodation_property_photo found with uid: {}", accomodation_property_photo_uid));
        Ok(res)
    }
}

#[web::get("/accomodation_property_photo")]
async fn get_accomodation_property_photo(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_photo_list = web::block(move || actions::find_accomodation_property_photo(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_photo_list))
}


#[web::post("/accomodation_property_photo")]
async fn add_accomodation_property_photo(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewAccomodationPropertyPhoto>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_photo = web::block(move || actions::insert_new_accomodation_property_photo(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_photo))
}

// -- End accomodation_property_photo

// -- Start accomodation_property_status

#[web::get("/accomodation_property_status/{accomodation_property_status_id}")]
async fn get_accomodation_property_status_by_id(
    pool: web::types::State<db::DbPool>,
    accomodation_property_status_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let accomodation_property_status_uid = accomodation_property_status_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_status = web::block(move || actions::find_accomodation_property_status_by_uid(accomodation_property_status_uid, &mut conn)).await?;

    if let Some(accomodation_property_status) = accomodation_property_status {
        Ok(web::HttpResponse::Ok().json(&accomodation_property_status))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No accomodation_property_status found with uid: {}", accomodation_property_status_uid));
        Ok(res)
    }
}

#[web::get("/accomodation_property_status")]
async fn get_accomodation_property_status(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_status_list = web::block(move || actions::find_accomodation_property_status(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_status_list))
}

#[web::get("/accomodation_property_status_eager/")]
async fn get_accomodation_property_status_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_status_list = web::block(move || actions::find_accomodation_property_status_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_status_list))
}

#[web::post("/accomodation_property_status")]
async fn add_accomodation_property_status(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewAccomodationPropertyStatu>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_status = web::block(move || actions::insert_new_accomodation_property_status(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_status))
}

// -- End accomodation_property_status

// -- Start accomodation_property_to_facility


// -- End accomodation_property_to_facility

// -- Start accomodation_property_to_language


// -- End accomodation_property_to_language

// -- Start accomodation_property_to_type


// -- End accomodation_property_to_type

// -- Start accomodation_property_type

#[web::get("/accomodation_property_type/{accomodation_property_type_id}")]
async fn get_accomodation_property_type_by_id(
    pool: web::types::State<db::DbPool>,
    accomodation_property_type_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let accomodation_property_type_uid = accomodation_property_type_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_type = web::block(move || actions::find_accomodation_property_type_by_uid(accomodation_property_type_uid, &mut conn)).await?;

    if let Some(accomodation_property_type) = accomodation_property_type {
        Ok(web::HttpResponse::Ok().json(&accomodation_property_type))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No accomodation_property_type found with uid: {}", accomodation_property_type_uid));
        Ok(res)
    }
}

#[web::get("/accomodation_property_type")]
async fn get_accomodation_property_type(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_type_list = web::block(move || actions::find_accomodation_property_type(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_type_list))
}


#[web::post("/accomodation_property_type")]
async fn add_accomodation_property_type(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewAccomodationPropertyType>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let accomodation_property_type = web::block(move || actions::insert_new_accomodation_property_type(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&accomodation_property_type))
}

// -- End accomodation_property_type

// -- Start address

#[web::get("/address/{address_id}")]
async fn get_address_by_id(
    pool: web::types::State<db::DbPool>,
    address_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let address_uid = address_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let address = web::block(move || actions::find_address_by_uid(address_uid, &mut conn)).await?;

    if let Some(address) = address {
        Ok(web::HttpResponse::Ok().json(&address))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No address found with uid: {}", address_uid));
        Ok(res)
    }
}

#[web::get("/address")]
async fn get_address(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let address_list = web::block(move || actions::find_address(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&address_list))
}

#[web::get("/address_eager/")]
async fn get_address_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let address_list = web::block(move || actions::find_address_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&address_list))
}

#[web::post("/address")]
async fn add_address(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewAddres>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let address = web::block(move || actions::insert_new_address(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&address))
}

// -- End address

// -- Start city

#[web::get("/city/{city_id}")]
async fn get_city_by_id(
    pool: web::types::State<db::DbPool>,
    city_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let city_uid = city_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let city = web::block(move || actions::find_city_by_uid(city_uid, &mut conn)).await?;

    if let Some(city) = city {
        Ok(web::HttpResponse::Ok().json(&city))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No city found with uid: {}", city_uid));
        Ok(res)
    }
}

#[web::get("/city")]
async fn get_city(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let city_list = web::block(move || actions::find_city(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&city_list))
}

#[web::get("/city_eager/")]
async fn get_city_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let city_list = web::block(move || actions::find_city_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&city_list))
}

#[web::post("/city")]
async fn add_city(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewCity>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let city = web::block(move || actions::insert_new_city(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&city))
}

// -- End city

// -- Start country

#[web::get("/country/{country_id}")]
async fn get_country_by_id(
    pool: web::types::State<db::DbPool>,
    country_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let country_uid = country_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let country = web::block(move || actions::find_country_by_uid(country_uid, &mut conn)).await?;

    if let Some(country) = country {
        Ok(web::HttpResponse::Ok().json(&country))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No country found with uid: {}", country_uid));
        Ok(res)
    }
}

#[web::get("/country")]
async fn get_country(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let country_list = web::block(move || actions::find_country(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&country_list))
}

#[web::get("/country_eager/")]
async fn get_country_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let country_list = web::block(move || actions::find_country_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&country_list))
}

#[web::post("/country")]
async fn add_country(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewCountry>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let country = web::block(move || actions::insert_new_country(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&country))
}

// -- End country

// -- Start county

#[web::get("/county/{county_id}")]
async fn get_county_by_id(
    pool: web::types::State<db::DbPool>,
    county_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let county_uid = county_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let county = web::block(move || actions::find_county_by_uid(county_uid, &mut conn)).await?;

    if let Some(county) = county {
        Ok(web::HttpResponse::Ok().json(&county))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No county found with uid: {}", county_uid));
        Ok(res)
    }
}

#[web::get("/county")]
async fn get_county(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let county_list = web::block(move || actions::find_county(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&county_list))
}

#[web::get("/county_eager/")]
async fn get_county_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let county_list = web::block(move || actions::find_county_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&county_list))
}

#[web::post("/county")]
async fn add_county(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewCounty>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let county = web::block(move || actions::insert_new_county(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&county))
}

// -- End county

// -- Start currency

#[web::get("/currency/{currency_id}")]
async fn get_currency_by_id(
    pool: web::types::State<db::DbPool>,
    currency_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let currency_uid = currency_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let currency = web::block(move || actions::find_currency_by_uid(currency_uid, &mut conn)).await?;

    if let Some(currency) = currency {
        Ok(web::HttpResponse::Ok().json(&currency))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No currency found with uid: {}", currency_uid));
        Ok(res)
    }
}

#[web::get("/currency")]
async fn get_currency(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let currency_list = web::block(move || actions::find_currency(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&currency_list))
}

#[web::get("/currency_eager/")]
async fn get_currency_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let currency_list = web::block(move || actions::find_currency_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&currency_list))
}

#[web::post("/currency")]
async fn add_currency(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewCurrency>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let currency = web::block(move || actions::insert_new_currency(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&currency))
}

// -- End currency

// -- Start language

#[web::get("/language/{language_id}")]
async fn get_language_by_id(
    pool: web::types::State<db::DbPool>,
    language_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let language_uid = language_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let language = web::block(move || actions::find_language_by_uid(language_uid, &mut conn)).await?;

    if let Some(language) = language {
        Ok(web::HttpResponse::Ok().json(&language))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No language found with uid: {}", language_uid));
        Ok(res)
    }
}

#[web::get("/language")]
async fn get_language(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let language_list = web::block(move || actions::find_language(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&language_list))
}

#[web::get("/language_eager/")]
async fn get_language_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let language_list = web::block(move || actions::find_language_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&language_list))
}

#[web::post("/language")]
async fn add_language(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewLanguage>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let language = web::block(move || actions::insert_new_language(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&language))
}

// -- End language

// -- Start meal_plan

#[web::get("/meal_plan/{meal_plan_id}")]
async fn get_meal_plan_by_id(
    pool: web::types::State<db::DbPool>,
    meal_plan_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let meal_plan_uid = meal_plan_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let meal_plan = web::block(move || actions::find_meal_plan_by_uid(meal_plan_uid, &mut conn)).await?;

    if let Some(meal_plan) = meal_plan {
        Ok(web::HttpResponse::Ok().json(&meal_plan))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No meal_plan found with uid: {}", meal_plan_uid));
        Ok(res)
    }
}

#[web::get("/meal_plan")]
async fn get_meal_plan(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let meal_plan_list = web::block(move || actions::find_meal_plan(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&meal_plan_list))
}

#[web::get("/meal_plan_eager/")]
async fn get_meal_plan_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let meal_plan_list = web::block(move || actions::find_meal_plan_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&meal_plan_list))
}

#[web::post("/meal_plan")]
async fn add_meal_plan(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewMealPlan>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let meal_plan = web::block(move || actions::insert_new_meal_plan(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&meal_plan))
}

// -- End meal_plan

// -- Start photo_tag

#[web::get("/photo_tag/{photo_tag_id}")]
async fn get_photo_tag_by_id(
    pool: web::types::State<db::DbPool>,
    photo_tag_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let photo_tag_uid = photo_tag_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let photo_tag = web::block(move || actions::find_photo_tag_by_uid(photo_tag_uid, &mut conn)).await?;

    if let Some(photo_tag) = photo_tag {
        Ok(web::HttpResponse::Ok().json(&photo_tag))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No photo_tag found with uid: {}", photo_tag_uid));
        Ok(res)
    }
}

#[web::get("/photo_tag")]
async fn get_photo_tag(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let photo_tag_list = web::block(move || actions::find_photo_tag(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&photo_tag_list))
}


#[web::post("/photo_tag")]
async fn add_photo_tag(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewPhotoTag>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let photo_tag = web::block(move || actions::insert_new_photo_tag(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&photo_tag))
}

// -- End photo_tag

// -- Start reservation

#[web::get("/reservation/{reservation_id}")]
async fn get_reservation_by_id(
    pool: web::types::State<db::DbPool>,
    reservation_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let reservation_uid = reservation_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let reservation = web::block(move || actions::find_reservation_by_uid(reservation_uid, &mut conn)).await?;

    if let Some(reservation) = reservation {
        Ok(web::HttpResponse::Ok().json(&reservation))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No reservation found with uid: {}", reservation_uid));
        Ok(res)
    }
}

#[web::get("/reservation")]
async fn get_reservation(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let reservation_list = web::block(move || actions::find_reservation(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&reservation_list))
}


#[web::post("/reservation")]
async fn add_reservation(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewReservation>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let reservation = web::block(move || actions::insert_new_reservation(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&reservation))
}

// -- End reservation

// -- Start role

#[web::get("/role/{role_id}")]
async fn get_role_by_id(
    pool: web::types::State<db::DbPool>,
    role_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let role_uid = role_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let role = web::block(move || actions::find_role_by_uid(role_uid, &mut conn)).await?;

    if let Some(role) = role {
        Ok(web::HttpResponse::Ok().json(&role))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No role found with uid: {}", role_uid));
        Ok(res)
    }
}

#[web::get("/role")]
async fn get_role(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let role_list = web::block(move || actions::find_role(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&role_list))
}

#[web::get("/role_eager/")]
async fn get_role_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let role_list = web::block(move || actions::find_role_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&role_list))
}

#[web::post("/role")]
async fn add_role(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewRole>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let role = web::block(move || actions::insert_new_role(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&role))
}

// -- End role

// -- Start room

#[web::get("/room/{room_id}")]
async fn get_room_by_id(
    pool: web::types::State<db::DbPool>,
    room_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let room_uid = room_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room = web::block(move || actions::find_room_by_uid(room_uid, &mut conn)).await?;

    if let Some(room) = room {
        Ok(web::HttpResponse::Ok().json(&room))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No room found with uid: {}", room_uid));
        Ok(res)
    }
}

#[web::get("/room")]
async fn get_room(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_list = web::block(move || actions::find_room(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_list))
}

#[web::get("/room_eager/")]
async fn get_room_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_list = web::block(move || actions::find_room_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_list))
}

#[web::post("/room")]
async fn add_room(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewRoom>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room = web::block(move || actions::insert_new_room(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room))
}

// -- End room

// -- Start room_facilities

#[web::get("/room_facility/{room_facility_id}")]
async fn get_room_facility_by_id(
    pool: web::types::State<db::DbPool>,
    room_facility_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let room_facility_uid = room_facility_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_facility = web::block(move || actions::find_room_facility_by_uid(room_facility_uid, &mut conn)).await?;

    if let Some(room_facility) = room_facility {
        Ok(web::HttpResponse::Ok().json(&room_facility))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No room_facility found with uid: {}", room_facility_uid));
        Ok(res)
    }
}

#[web::get("/room_facility")]
async fn get_room_facilities(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_facility_list = web::block(move || actions::find_room_facilities(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_facility_list))
}


#[web::post("/room_facility")]
async fn add_room_facility(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewRoomFacility>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_facility = web::block(move || actions::insert_new_room_facility(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_facility))
}

// -- End room_facility

// -- Start room_meal_plan


// -- End room_meal_plan

// -- Start room_photo

#[web::get("/room_photo/{room_photo_id}")]
async fn get_room_photo_by_id(
    pool: web::types::State<db::DbPool>,
    room_photo_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let room_photo_uid = room_photo_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_photo = web::block(move || actions::find_room_photo_by_uid(room_photo_uid, &mut conn)).await?;

    if let Some(room_photo) = room_photo {
        Ok(web::HttpResponse::Ok().json(&room_photo))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No room_photo found with uid: {}", room_photo_uid));
        Ok(res)
    }
}

#[web::get("/room_photo")]
async fn get_room_photo(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_photo_list = web::block(move || actions::find_room_photo(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_photo_list))
}


#[web::post("/room_photo")]
async fn add_room_photo(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewRoomPhoto>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_photo = web::block(move || actions::insert_new_room_photo(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_photo))
}

// -- End room_photo

// -- Start room_pricing_type

#[web::get("/room_pricing_type/{room_pricing_type_id}")]
async fn get_room_pricing_type_by_id(
    pool: web::types::State<db::DbPool>,
    room_pricing_type_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let room_pricing_type_uid = room_pricing_type_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_pricing_type = web::block(move || actions::find_room_pricing_type_by_uid(room_pricing_type_uid, &mut conn)).await?;

    if let Some(room_pricing_type) = room_pricing_type {
        Ok(web::HttpResponse::Ok().json(&room_pricing_type))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No room_pricing_type found with uid: {}", room_pricing_type_uid));
        Ok(res)
    }
}

#[web::get("/room_pricing_type")]
async fn get_room_pricing_type(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_pricing_type_list = web::block(move || actions::find_room_pricing_type(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_pricing_type_list))
}

#[web::get("/room_pricing_type_eager/")]
async fn get_room_pricing_type_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_pricing_type_list = web::block(move || actions::find_room_pricing_type_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_pricing_type_list))
}

#[web::post("/room_pricing_type")]
async fn add_room_pricing_type(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewRoomPricingType>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_pricing_type = web::block(move || actions::insert_new_room_pricing_type(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_pricing_type))
}

// -- End room_pricing_type

// -- Start room_to_facilities


// -- End room_to_facility

// -- Start room_type_availability_rates

#[web::get("/room_type_availability_rate/{room_type_availability_rate_id}")]
async fn get_room_type_availability_rate_by_id(
    pool: web::types::State<db::DbPool>,
    room_type_availability_rate_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let room_type_availability_rate_uid = room_type_availability_rate_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_type_availability_rate = web::block(move || actions::find_room_type_availability_rate_by_uid(room_type_availability_rate_uid, &mut conn)).await?;

    if let Some(room_type_availability_rate) = room_type_availability_rate {
        Ok(web::HttpResponse::Ok().json(&room_type_availability_rate))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No room_type_availability_rate found with uid: {}", room_type_availability_rate_uid));
        Ok(res)
    }
}

#[web::get("/room_type_availability_rate")]
async fn get_room_type_availability_rates(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_type_availability_rate_list = web::block(move || actions::find_room_type_availability_rates(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_type_availability_rate_list))
}

#[web::get("/room_type_availability_rate_eager/")]
async fn get_room_type_availability_rates_eager(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_type_availability_rate_list = web::block(move || actions::find_room_type_availability_rate_with_all(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_type_availability_rate_list))
}

#[web::post("/room_type_availability_rate")]
async fn add_room_type_availability_rate(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewRoomTypeAvailabilityRate>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let room_type_availability_rate = web::block(move || actions::insert_new_room_type_availability_rate(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&room_type_availability_rate))
}

// -- End room_type_availability_rate

// -- Start users

#[web::get("/user/{user_id}")]
async fn get_user_by_id(
    pool: web::types::State<db::DbPool>,
    user_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let user_uid = user_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &mut conn)).await?;

    if let Some(user) = user {
        Ok(web::HttpResponse::Ok().json(&user))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

#[web::get("/user")]
async fn get_users(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user_list = web::block(move || actions::find_users(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&user_list))
}


#[web::post("/user")]
async fn add_user(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewUser>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&user))
}

// -- End user

pub fn wire_routes(cfg: &mut web::ServiceConfig){
    cfg.service(get_accomodation_property_by_id)
       .service(get_accomodation_property)
       .service(get_accomodation_property_eager)
       .service(add_accomodation_property)
       .service(get_accomodation_property_facility_by_id)
       .service(get_accomodation_property_facility)
       .service(add_accomodation_property_facility)
       .service(get_accomodation_property_photo_by_id)
       .service(get_accomodation_property_photo)
       .service(add_accomodation_property_photo)
       .service(get_accomodation_property_status_by_id)
       .service(get_accomodation_property_status)
       .service(get_accomodation_property_status_eager)
       .service(add_accomodation_property_status)
       .service(get_accomodation_property_type_by_id)
       .service(get_accomodation_property_type)
       .service(add_accomodation_property_type)
       .service(get_address_by_id)
       .service(get_address)
       .service(get_address_eager)
       .service(add_address)
       .service(get_city_by_id)
       .service(get_city)
       .service(get_city_eager)
       .service(add_city)
       .service(get_country_by_id)
       .service(get_country)
       .service(get_country_eager)
       .service(add_country)
       .service(get_county_by_id)
       .service(get_county)
       .service(get_county_eager)
       .service(add_county)
       .service(get_currency_by_id)
       .service(get_currency)
       .service(get_currency_eager)
       .service(add_currency)
       .service(get_language_by_id)
       .service(get_language)
       .service(get_language_eager)
       .service(add_language)
       .service(get_meal_plan_by_id)
       .service(get_meal_plan)
       .service(get_meal_plan_eager)
       .service(add_meal_plan)
       .service(get_photo_tag_by_id)
       .service(get_photo_tag)
       .service(add_photo_tag)
       .service(get_reservation_by_id)
       .service(get_reservation)
       .service(add_reservation)
       .service(get_role_by_id)
       .service(get_role)
       .service(get_role_eager)
       .service(add_role)
       .service(get_room_by_id)
       .service(get_room)
       .service(get_room_eager)
       .service(add_room)
       .service(get_room_facility_by_id)
       .service(get_room_facilities)
       .service(add_room_facility)
       .service(get_room_photo_by_id)
       .service(get_room_photo)
       .service(add_room_photo)
       .service(get_room_pricing_type_by_id)
       .service(get_room_pricing_type)
       .service(get_room_pricing_type_eager)
       .service(add_room_pricing_type)
       .service(get_room_type_availability_rate_by_id)
       .service(get_room_type_availability_rates)
       .service(get_room_type_availability_rates_eager)
       .service(add_room_type_availability_rate)
       .service(get_user_by_id)
       .service(get_users)
       .service(add_user)
       .service(get_promotions)
       .service(get_search_res);

}
