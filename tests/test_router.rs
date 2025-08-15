use servidor_http::router::Router;

#[test]
fn test_default_router() {
   let default_router = Router::default();

   assert_eq!(default_router, Router::new(String::from("/")));
}
