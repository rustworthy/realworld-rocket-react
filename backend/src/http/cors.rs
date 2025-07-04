use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

pub fn cors<S>(allowed_origins: &[S]) -> Cors
where
    S: AsRef<str>,
{
    let allowed_origins = AllowedOrigins::some_regex(allowed_origins);
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Patch, Method::Put]
            .into_iter()
            .map(|v| v.into())
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS fairing build successfully")
}
