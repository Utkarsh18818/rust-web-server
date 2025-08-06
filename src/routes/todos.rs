
use actix_web::{web::Data, get , post , Responder};

use Sqlx::FromRow;
use Sqlx::Error;

#[derive(Serialize,Deserialize)]
pub struct CreateNewTodo{
    title:String,
    description: Option<String>

}

#[derive(Serialize,Deserialize,FromRow)]

pub struct Todo{
    id:i32,
    title:String,
    description:Option<String>,
    status:String
}
#[derive(Serialize)]
pub struct TypeDbError{
    error:String
}

#[post("/todo/create")]
pub async fn create_new_todo(db:Data<MySqlPool>, body: Json<CreateNewTodo>) -> impl Responder{
    let response = Sqlx::query(
        "INSERT INTO todos(title,description) values(?,?)"
    ).bind(&body.title)
    .bind(&body.description)
    .execute(&**db)
    .await;

    match response{
        Ok(id)=>{
            HttpResponse::Created().json(Todo{
                id:id.last_insert_id() as i32,
                description:body.description.clone(),
                title:body.title.clone(),
                status:"New".to_string()
            })
        },
        Err(_e)=>{
            HttpResponse::InternalServerError().json(TypeDbError{
                error:_e.to_string(),
            })
        }
    }
}
#[get("/todos/all")]

pub async fn get_all_todos(db: Data<MySqlPool>) -> impl Responder{
    let Result<vec<todo>,Error> = Sqlx::query_as("select,
    id, title, description, status from todos
    ").fetch_all(&**db).await;
    
    match res {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Error(_e) =>HttpResponse::InternalServerError().json(TypeDbError{
            error: _e.to_string(),
        })
    }
}