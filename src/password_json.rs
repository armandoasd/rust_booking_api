use serde::{self, Deserialize, Serializer, Deserializer, de};
use crate::auth;
pub fn serialize<S>(
    password: &String,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // let s = auth::encrypt_password(password.to_owned());
    // match s {
    //     Ok(new_str)=> serializer.serialize_str(&new_str),
    //     Err(err)=>Err(ser::Error::custom(format!("error parsing password {}", err)))
    // }
    serializer.serialize_str("Null")
}

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let pass = auth::encrypt_password(s);
    match pass {
        Ok(new_str)=>  Ok(new_str),
        Err(err)=>Err(de::Error::custom(format!("error parsing password {}", err)))
    }
   
}