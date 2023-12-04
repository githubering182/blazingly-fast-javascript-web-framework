use web::response::Response;
use web::server::Server;

fn main() {
    let result = Server::new("0.0.0.0", 9000)
        .post("/test", |r| {
            let mut response = Response::new();
            response.set_body(format!(
                "ADDED HANLDER: req_method:{}, req_path: {}",
                r.method.as_ref().unwrap(),
                r.route.as_ref().unwrap()
            ));
            response
        })
        .get("/", |r| {
            let mut response = Response::new();
            response.set_header("Content-type", "application/json");
            response.set_body(format!(
                "{{\"http\": {}, \"method\": {}, \"route\": {}}}",
                r.http.as_ref().unwrap(),
                r.method.as_ref().unwrap(),
                r.route.as_ref().unwrap()
            ));
            response
        })
        .listen(5);

    match result {
        Ok(_) => println!("Server shut down"),
        Err(e) => println!("Server init error: {:?}", e),
    }
}