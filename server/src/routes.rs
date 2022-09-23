use crate::{db, errors::MyError, link::Link};
use actix_web::{http, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use url::Url;

///
/// fetches the link from the database
/// redirects to the longlink
///
/// TODO return error page instead of not found
///
pub async fn get_link(
    somelink: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client = db_pool.get().await.map_err(MyError::PoolError)?;

    let lookup = db::get_link(&client, &somelink).await;

    //change ? to match and return a not found code/website
    let ret = match lookup {
        Ok(l) => {
            Ok(HttpResponse::TemporaryRedirect()
            .status(http::StatusCode::TEMPORARY_REDIRECT)
            .insert_header(("location", l.longlink))
            .finish())
        },
        Err(_) => Ok(HttpResponse::NotFound().finish())
    };
    ret
}

//TODO handle shortlink having wrong characters
//TODO handle link too long or shortling too long
pub async fn create_link(
    json: web::Json<Link>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", &json);
    let link_info: Link = json.into_inner();


    //only very basic handling. still accepts domains just not completely faulty ones
    let _url = Url::parse(&link_info.longlink).map_err(|_| return MyError::InvalidLinkError)?;

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_link = db::create_link(&client, link_info).await;

    match new_link {
        Ok(l) => return Ok(HttpResponse::Ok().json(l)),
        Err(_) => return Err(Error::from(MyError::AlreadyExistsError)),
    }
}

pub async fn get_all(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let linkk = db::get_all(&client).await?; //advanced error handling?
    Ok(HttpResponse::Ok().json(linkk))
}
