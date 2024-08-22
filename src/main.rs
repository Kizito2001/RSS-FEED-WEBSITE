use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use askama::Template;
use serde::Serialize;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    entertainment_items: Vec<RSSItem<'a>>,
    health_items: Vec<RSSItem<'a>>,
    education_items: Vec<RSSItem<'a>>,
}

#[derive(Serialize)]
struct RSSItem<'a> {
    title: &'a str,
    link: &'a str,
    image_url: &'a str,
}

async fn index() -> impl Responder {
    let entertainment_items = vec![
        RSSItem { title: "Item 1", link: "https://example.com/item1", image_url: "https://i.pinimg.com/564x/88/bb/a6/88bba6902fbc6f905cd502b41d6f4b1d.jpg" },
        RSSItem { title: "Item 2", link: "https://example.com/item2", image_url: "https://i.pinimg.com/564x/56/f0/8b/56f08bb60d2dd335c2187fd14525bd6e.jpg" },
        RSSItem { title: "Item 3", link: "https://example.com/item3", image_url: "https://i.pinimg.com/564x/df/4c/1c/df4c1c09e924d89dd20b8cecab195127.jpg" },
        // Add more items as needed
    ];

    let health_items = vec![
        RSSItem { title: "Item 1", link: "https://example.com/item1", image_url: "https://i.pinimg.com/736x/aa/1f/ee/aa1fee1544bbe0c4b652a32fe7ee9b5b.jpg" },
        RSSItem { title: "Item 2", link: "https://example.com/item2", image_url: "https://i.pinimg.com/564x/52/6d/d4/526dd4ac615a46bdf9255710c6de4c84.jpg" },
        RSSItem { title: "Item 3", link: "https://example.com/item3", image_url: "https://i.pinimg.com/736x/55/ed/b2/55edb275f6d8b5c85ae178f53be8dffb.jpg" },
        // Add more items as needed
    ];

    let education_items = vec![
        RSSItem { title: "Item 1", link: "https://example.com/item1", image_url: "https://i.pinimg.com/564x/f8/6f/fc/f86ffc78a7f28befddd587f37d4bf8c1.jpg" },
        RSSItem { title: "Item 2", link: "https://example.com/item2", image_url: "https://i.pinimg.com/564x/14/23/1e/14231e7f518a98fe3f8999ceb2f32cca.jpg" },
        RSSItem { title: "Item 3", link: "https://example.com/item3", image_url: "https://i.pinimg.com/564x/35/d4/74/35d474753df6a5fa03db5fd5d47a0d48.jpg" },
        // Add more items as needed
    ];

    let template = IndexTemplate {
        entertainment_items,
        health_items,
        education_items,
    };

    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
