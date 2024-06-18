// @generated automatically by Diesel CLI.

diesel::table! {
    accomodation_property (id) {
        id -> Bigint,
        version -> Bigint,
        stars -> Integer,
        currency_id -> Nullable<Bigint>,
        number_of_rooms -> Integer,
        #[max_length = 255]
        hotel_logo -> Nullable<Varchar>,
        address_id -> Nullable<Bigint>,
        checked_out -> Nullable<Bool>,
        role_id -> Nullable<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        status_id -> Nullable<Bigint>,
        preferred_language_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    accomodation_property_facility (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        hac -> Nullable<Varchar>,
        #[max_length = 255]
        booking_extended_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    accomodation_property_photo (id) {
        id -> Bigint,
        version -> Bigint,
        accomodation_property_id -> Bigint,
        #[max_length = 255]
        file_name -> Varchar,
    }
}

diesel::table! {
    accomodation_property_status (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        status -> Varchar,
    }
}

diesel::table! {
    accomodation_property_to_facility (accomodation_property_facility_id, accomodation_property_id) {
        accomodation_property_id -> Bigint,
        accomodation_property_facility_id -> Bigint,
    }
}

diesel::table! {
    accomodation_property_to_language (accomodation_property_id, language_id) {
        accomodation_property_id -> Bigint,
        language_id -> Bigint,
    }
}

diesel::table! {
    accomodation_property_to_type (accomodation_property_id, accomodation_property_type_id) {
        accomodation_property_id -> Bigint,
        accomodation_property_type_id -> Bigint,
    }
}

diesel::table! {
    accomodation_property_type (id) {
        id -> Bigint,
        version -> Bigint,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
    }
}

diesel::table! {
    address (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        region -> Nullable<Varchar>,
        #[max_length = 255]
        telephone -> Nullable<Varchar>,
        #[max_length = 255]
        fax -> Nullable<Varchar>,
        #[max_length = 255]
        additional_emails -> Nullable<Varchar>,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        on_map -> Nullable<Varchar>,
        #[max_length = 255]
        street -> Varchar,
        #[max_length = 255]
        zip_code -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        _city_id -> Bigint,
    }
}

diesel::table! {
    city (id) {
        id -> Bigint,
        #[max_length = 2]
        cc_fips -> Nullable<Varchar>,
        #[max_length = 2]
        country_iso_id -> Nullable<Varchar>,
        #[max_length = 200]
        full_name -> Nullable<Varchar>,
        version -> Bigint,
        show_city -> Bool,
        county_id -> Bigint,
    }
}

diesel::table! {
    country (id) {
        id -> Bigint,
        #[max_length = 2]
        id_fips -> Nullable<Varchar>,
        #[max_length = 2]
        id_iso -> Nullable<Varchar>,
        #[max_length = 3]
        tld -> Nullable<Varchar>,
        #[max_length = 100]
        country_name -> Nullable<Varchar>,
        version -> Bigint,
    }
}

diesel::table! {
    county (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        country_id -> Bigint,
    }
}

diesel::table! {
    currency (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 12]
        #[sql_name = "currency"]
        value -> Varchar,
    }
}

diesel::table! {
    language (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        code -> Varchar,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    meal_plan (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    photo_tag (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    reservation (id) {
        id -> Bigint,
        version -> Bigint,
        room_id -> Nullable<Bigint>,
        occupancy -> Integer,
        date -> Datetime,
        room_type_availability_id -> Bigint,
        price -> Float,
        meal_plan_id -> Nullable<Bigint>,
        status -> Integer,
    }
}

diesel::table! {
    role (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        authority -> Varchar,
    }
}

diesel::table! {
    room (id) {
        id -> Bigint,
        version -> Bigint,
        default_occupancy -> Integer,
        minimum_guests -> Integer,
        #[max_length = 255]
        name -> Varchar,
        pricing_type_id -> Bigint,
        accomodation_property_id -> Bigint,
        baby_coats -> Integer,
        maximum_guests -> Integer,
        #[max_length = 255]
        room_view -> Nullable<Varchar>,
        floor -> Nullable<Integer>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        size_measurement -> Float,
        #[max_length = 4]
        size_measurement_unit -> Varchar,
    }
}

diesel::table! {
    room_facilities (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        rma -> Nullable<Varchar>,
        #[max_length = 255]
        booking_extended_amenities -> Nullable<Varchar>,
    }
}

diesel::table! {
    room_meal_plan (room_meal_plan_id, meal_plan_id) {
        room_meal_plan_id -> Bigint,
        meal_plan_id -> Bigint,
    }
}

diesel::table! {
    room_photo (id) {
        id -> Bigint,
        version -> Bigint,
        room_id -> Bigint,
        #[max_length = 255]
        file_size -> Nullable<Varchar>,
        rank_sort -> Integer,
        #[max_length = 255]
        file_name -> Varchar,
    }
}

diesel::table! {
    room_pricing_type (id) {
        id -> Bigint,
        version -> Bigint,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    room_to_facilities (room_id, room_facility_id) {
        room_id -> Bigint,
        room_facility_id -> Bigint,
    }
}

diesel::table! {
    room_type_availability_rates (id) {
        id -> Bigint,
        available_units -> Nullable<Integer>,
        rate_amount_parameter -> Nullable<Float>,
        room_id -> Bigint,
        date -> Datetime,
    }
}

diesel::table! {
    user_to_role (user_id, role_id) {
        user_id -> Bigint,
        role_id -> Bigint,
    }
}

diesel::table! {
    users (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(accomodation_property -> accomodation_property_status (status_id));
diesel::joinable!(accomodation_property -> address (address_id));
diesel::joinable!(accomodation_property -> currency (currency_id));
diesel::joinable!(accomodation_property -> language (preferred_language_id));
diesel::joinable!(accomodation_property -> role (role_id));
diesel::joinable!(accomodation_property_photo -> accomodation_property (accomodation_property_id));
diesel::joinable!(accomodation_property_to_facility -> accomodation_property (accomodation_property_id));
diesel::joinable!(accomodation_property_to_facility -> accomodation_property_facility (accomodation_property_facility_id));
diesel::joinable!(accomodation_property_to_language -> accomodation_property (accomodation_property_id));
diesel::joinable!(accomodation_property_to_language -> language (language_id));
diesel::joinable!(accomodation_property_to_type -> accomodation_property (accomodation_property_id));
diesel::joinable!(accomodation_property_to_type -> accomodation_property_type (accomodation_property_type_id));
diesel::joinable!(address -> city (_city_id));
diesel::joinable!(city -> county (county_id));
diesel::joinable!(county -> country (country_id));
diesel::joinable!(reservation -> meal_plan (meal_plan_id));
diesel::joinable!(reservation -> room (room_id));
diesel::joinable!(reservation -> room_type_availability_rates (room_type_availability_id));
diesel::joinable!(room -> accomodation_property (accomodation_property_id));
diesel::joinable!(room -> room_pricing_type (pricing_type_id));
diesel::joinable!(room_meal_plan -> meal_plan (meal_plan_id));
diesel::joinable!(room_meal_plan -> room (room_meal_plan_id));
diesel::joinable!(room_photo -> room (room_id));
diesel::joinable!(room_to_facilities -> room (room_id));
diesel::joinable!(room_to_facilities -> room_facilities (room_facility_id));
diesel::joinable!(room_type_availability_rates -> room (room_id));
diesel::joinable!(user_to_role -> role (role_id));
diesel::joinable!(user_to_role -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accomodation_property,
    accomodation_property_facility,
    accomodation_property_photo,
    accomodation_property_status,
    accomodation_property_to_facility,
    accomodation_property_to_language,
    accomodation_property_to_type,
    accomodation_property_type,
    address,
    city,
    country,
    county,
    currency,
    language,
    meal_plan,
    photo_tag,
    reservation,
    role,
    room,
    room_facilities,
    room_meal_plan,
    room_photo,
    room_pricing_type,
    room_to_facilities,
    room_type_availability_rates,
    user_to_role,
    users,
);
