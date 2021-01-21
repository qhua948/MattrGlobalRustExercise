#[cfg(test)]
mod test {
    use super::super::get_ignited_rocket;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};
    use crate::datastructures::Schema;

    #[test]
    fn rocket_simple() {
        let client = Client::new(get_ignited_rocket()).expect("valid rocket instance");
    }

    fn setup() {}

    fn get_client() -> Client {
        let client = Client::new(get_ignited_rocket()).unwrap();
        client
    }

    #[test]
    fn test_create_schema() {
        let client = get_client();
        let body =
            "{\"schema\": {
    \"a\": \"Bool\",
    \"b\": {
      \"Map\": {
        \"c\": \"Bool\"
      }
    },
    \"d\": {
      \"List\": [
        {
          \"Map\": {
            \"e\": \"Float\"
          }
        },
        \"Bool\"
      ]
    }
  }
}";
        let body_json = serde_json::from_str::<Schema>(&body).unwrap();
        let mut response = client.post("/schemas").body(body).dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.content_type().expect("No content type"), ContentType::JSON);
        let res = serde_json::from_str::<Schema>(&response.body().expect("No content body").into_string().unwrap()).expect("Can't decode json");
        assert!(res.id.unwrap() >= 0);
        assert!(res.id.unwrap() <= 2147483647);
        assert_eq!(res.schema.unwrap(), body_json.schema.unwrap())
    }
}
