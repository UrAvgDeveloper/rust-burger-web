extern crate rocket;
extern crate base64;

use rocket::{
    Request,
    Outcome,
    outcome::IntoOutcome,
    http::Status,
    request::FromRequest,
    request
};

pub struct BasicAuthRaw {
    pub username: String,
    pub password: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for BasicAuthRaw {
    type Error = ();

    fn from_request(request: &Request) -> Outcome<Self, (Status, <Self as FromRequest<'a, 'r>>::Error), ()> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            let split = auth_header.split_whitespace().collect::<Vec<_>>();
            if split.len() != 2 {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
            let (basic, payload) = (split[0], split[1]);
            if basic != "Basic" {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
            let decoded = base64::decode(payload)
                .ok()
                .into_outcome((Status::BadRequest, ()))?;

            let decoded_str = String::from_utf8(decoded)
                .ok()
                .into_outcome((Status::BadRequest, ()))?;

            let split = decoded_str.split(":").collect::<Vec<_>>();

            // If exactly username & password pair are present
            if split.len() != 2 {
                return Outcome::Failure((Status::BadRequest, ()));
            }

            let (username, password) = (split[0].to_string(), split[1].to_string());

            Outcome::Success(BasicAuthRaw {
                username,
                password
            })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

pub struct Admin(String);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &Request) -> request::Outcome<Self, Self::Error> {
        let basic = BasicAuthRaw::from_request(request)?;
        let admin_username = std::env::var("ADMIN_USERNAME").expect("ADMIN username must be set");
        let admin_password = std::env::var("ADMIN_PASSWORD").expect("ADMIN password must be set");
        if basic.username == admin_username && basic.password == admin_password {
            return Outcome::Success(Admin(basic.username));
        } else {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
    }
}