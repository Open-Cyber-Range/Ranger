mod common;

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, test, App};
    use ranger::routes::basic::{status, version};
    use semver::Version;


    use actix_web::web::Bytes;
    pub trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn test_status() {
        let app = test::init_service(App::new().service(status)).await;
        let request = test::TestRequest::get().uri("/status").to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_version() {
        let app = test::init_service(App::new().service(version)).await;
        let request = test::TestRequest::get().uri("/version").to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());
        let body = to_bytes(response.into_body()).await.unwrap();
        let version_string = body.as_str();
        Version::parse(version_string).unwrap();
    }
}