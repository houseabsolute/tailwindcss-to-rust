#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(
            fs::Files::new("/", "./public")
                .show_files_listing()
                .use_last_modified(true),
        )
    })
    .bind(("0.0.0.0", 7373))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(html!(
       <html>
         <head>
            <title>"Stuff a ndthigns"</title>
            <link rel="stylesheet" href="/assets/tailwind_compiled.css" />
        </head>
         <body>
            <div class={{ C.bg.bg_rose_500 }}>"Is this "<i>{hey("italicised?")}</i></div>
            <div class={{ M.hover }}:{{ C.spc.mt_0 }}></div>
            <div class="{{ M.hover }}:{{ C.bg.bg_rose_500 }} {{ C.bg.bg_rose_800 }}">...</div>
         </body>
       </html>
    ))
}
