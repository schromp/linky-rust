use actix_web::web::{self};

use crate::routes;

// .service(
//     actix_files::Files::new("/", "./web/")
//     .show_files_listing()
//     .index_file("index.html")
//     .use_last_modified(true),
// )

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg

    .service(web::resource("/createLink").route(web::post().to(routes::create_link)))
    .service(web::resource("/getAll").route(web::get().to(routes::get_all)))
    .service(web::resource("/{somelink}").route(web::get().to(routes::get_link)))
    .service(
        actix_files::Files::new("/", "./web")
        .show_files_listing()
        .index_file("index.html")
        .use_last_modified(true),
    );
}
