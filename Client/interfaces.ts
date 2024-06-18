export interface AccomodationProperty {
    id: number,
    version: number,
    stars: number,
    currency_id?: number,
    number_of_rooms: number,
    hotel_logo?: string,
    address_id?: number,
    checked_out?: number,
    role_id?: number,
    name: string,
    status_id?: number,
    preferred_language_id?: number,
}

export interface AccomodationPropertyFacility {
    id: number,
    version: number,
    name: string,
    hac?: string,
    booking_extended_code?: string,
}

export interface AccomodationPropertyPhoto {
    id: number,
    version: number,
    accomodation_property_id: number,
    file_name: string,
}

export interface AccomodationPropertyStatu {
    id: number,
    version: number,
    status: string,
}

export interface AccomodationPropertyToFacility {
    accomodation_property_id: number,
    accomodation_property_facility_id: number,
}

export interface AccomodationPropertyToLanguage {
    accomodation_property_id: number,
    language_id: number,
}

export interface AccomodationPropertyToType {
    accomodation_property_id: number,
    accomodation_property_type_id: number,
}

export interface AccomodationPropertyType {
    id: number,
    version: number,
    type_: string,
}

export interface Addres {
    id: number,
    version: number,
    region?: string,
    telephone?: string,
    fax?: string,
    additional_emails?: string,
    location?: string,
    city: string,
    on_map?: string,
    street: string,
    zip_code?: string,
    email?: string,
    website?: string,
    _city_id: number,
}

export interface City {
    id: number,
    cc_fips?: string,
    country_iso_id?: string,
    full_name?: string,
    version: number,
    show_city: number,
    county_id: number,
}

export interface Country {
    id: number,
    id_fips?: string,
    id_iso?: string,
    tld?: string,
    country_name?: string,
    version: number,
}

export interface County {
    id: number,
    version: number,
    name: string,
    country_id: number,
}

export interface Currency {
    id: number,
    version: number,
    value: string,
}

export interface Language {
    id: number,
    version: number,
    code: string,
    name: string,
}

export interface MealPlan {
    id: number,
    name: string,
    description?: string,
}

export interface PhotoTag {
    id: number,
    version: number,
    name: string,
}

export interface Reservation {
    id: number,
    version: number,
    room_id?: number,
    occupancy: number,
    date: Date,
    room_type_availability_id: number,
    price: number,
    meal_plan_id?: number,
    status: number,
}

export interface Role {
    id: number,
    version: number,
    authority: string,
}

export interface Room {
    id: number,
    version: number,
    default_occupancy: number,
    minimum_guests: number,
    name: string,
    pricing_type_id: number,
    accomodation_property_id: number,
    baby_coats: number,
    maximum_guests: number,
    room_view?: string,
    floor?: number,
    description?: string,
    size_measurement: number,
    size_measurement_unit: string,
}

export interface RoomFacility {
    id: number,
    version: number,
    name: string,
    rma?: string,
    booking_extended_amenities?: string,
}

export interface RoomMealPlan {
    room_meal_plan_id: number,
    meal_plan_id: number,
}

export interface RoomPhoto {
    id: number,
    version: number,
    room_id: number,
    file_size?: string,
    rank_sort: number,
    file_name: string,
}

export interface RoomPricingType {
    id: number,
    version: number,
    name: string,
}

export interface RoomToFacility {
    room_id: number,
    room_facility_id: number,
}

export interface RoomTypeAvailabilityRate {
    id: number,
    available_units?: number,
    rate_amount_parameter?: number,
    room_id: number,
    date: Date,
}

export interface UserToRole {
    user_id: number,
    role_id: number,
}

export interface User {
    id: number,
    name: string,
    password: string,
}


