#[macro_use] extern crate rocket;

use rocket::form::Form;

#[derive(FromForm)]
struct FormData {
    form_data: Option<String>,
}

#[post("/", data = "<form_data>")]
fn bug(form_data: Form<FormData>) -> Option<String> {
    form_data.into_inner().form_data
}

mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::ContentType;

    #[test]
    fn form_optional() {
        let client = Client::debug_with(routes![bug]).unwrap();
        let response = client.post("/")
            .header(ContentType::Form)
            .body("_method=post&form_data=")
            .dispatch();
        assert_eq!(response.into_string(), None);
    }
}
