/*
 * Best shop ever API
 *
 * Test project for Rust education purpose
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: cyberpunk.perosn@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// Address : User's address

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Address {
    /// Address identificator
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "street")]
    pub street: String,
    #[serde(rename = "zip")]
    pub zip: i32,
}

impl Address {
    /// User's address
    pub fn new(id: String, country: String, city: String, street: String, zip: i32) -> Address {
        Address {
            id,
            country,
            city,
            street,
            zip,
        }
    }
}
