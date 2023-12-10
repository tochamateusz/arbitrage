use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use arbitage::crawler::Crawler;

struct HtmlTag {
    name: String,
}

#[derive(Default)]
struct Parser {
    last: char,
    inner: String,
    html_tags: Vec<HtmlTag>,
}

impl Parser {
    fn new(last: char, html_tags: Vec<HtmlTag>) -> Self {
        Parser::default()
    }
}

async fn manual_hello() -> impl Responder {
    let crawler = Crawler::new();

    let req = crawler.client.get("http://localhost:8080/test2");

    let mut send = req.send().await.unwrap();
    let body = send.body().await;

    match body {
        Ok(b) => {
            let col = b.to_vec().iter().fold(vec![], |mut prev, curr| {
                prev.append(&mut [*curr].to_vec());
                prev
            });

            col.into_iter()
                .fold::<Parser, _>(Parser::new(' ', vec![]), |mut prev, curr| {
                    println!("{} {:#?}", curr as char, curr);
                    let name: Vec<HtmlTag> = vec![];

                    match (curr as char, prev.last as char) {
                        ('<', _) => {
                            println!("first");
                            prev.last = curr as char;
                        }
                        ('>', '<') => {}
                        (_, _) => (),
                    };
                    prev
                });
        }
        Err(e) => {
            eprintln!("Eror: {:#?}", e);
        }
    }

    HttpResponse::Ok().body("Hey there!")
}

async fn test2() -> impl Responder {
    HttpResponse::Ok().body("<html><body><div style=\"color:red\">test</div></body></html>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hey", web::get().to(manual_hello))
            .route("/test2", web::get().to(test2))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
