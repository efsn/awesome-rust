use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/index")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.")
    } else {
        HttpResponse::Ok().content_type("text/html").body(format!(
            "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
            form.n,
            form.m,
            gcd(form.n, form.m)
        ))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server on http://localhost:8000...");

    HttpServer::new(|| App::new().service(greet).service(index).service(post_gcd))
        .bind(("127.0.0.1", 8000))
        .expect("error: binding server to address")
        .run()
        .await
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
