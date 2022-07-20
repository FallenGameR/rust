use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Starting server on http://localhost:3000/");

    server
        .bind("127.0.0.1:3000").expect("error binding server")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GDC</button>
                </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GdcParameters>) -> HttpResponse{
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("One of the arguments is 0");
    }

    let response =
        format!("The greatest common divisor of {} and {} is {}",
            form.n,
            form.m,
            gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

#[derive(Deserialize)]
struct GdcParameters {
    n: u64,
    m: u64,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            let t = b;
            b = a;
            a = t;
        }
        b = b % a;
    }
    a
}