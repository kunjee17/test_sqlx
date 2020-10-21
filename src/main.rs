use anyhow::Result;
use dotenv::dotenv;
use sqlx::{PgPool, Row, Done};
use sqlx::postgres::PgRow;

#[derive(sqlx::FromRow, Debug)]
struct Todo {
    id : i64,
    description: String,
    done: bool
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Hello, world!");
    let database_url = dotenv::var("DATABASE_URL")?;
    println!("{}",database_url);

    let pool = PgPool::connect(&database_url).await?;

    // let mut conn = pool.acquire().await?;
    // we can directly create table from code. If not migration
    let create_table_query = "CREATE TABLE IF NOT EXISTS todos
(
    id          BIGSERIAL PRIMARY KEY,
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
);";

    let create_table = sqlx::query(create_table_query).execute(&pool).await?;

    println!("Created Table {:?}", create_table);

    let select_query = "SELECT id, description, done FROM todos ORDER BY id";
    let todos_query  = sqlx::query(select_query).fetch_all(&pool).await?;

    // Have to loop through all rows to print the values
    println!("Todo query rows {:#?}", todos_query.len());

    let todos_query_map = sqlx::query(select_query).try_map(|row : PgRow|  Ok(Todo {
        id : row.try_get("id")?,
        description : row.try_get("description")?,
        done : row.try_get("done")?
    })).fetch_all(&pool)
        .await?;

    println!(" Todo query map {:#?}", todos_query_map);

    let todos_query_as: Vec<Todo> = sqlx::query_as(select_query).fetch_all(&pool).await?;

    println!(" Todo query as {:#?}", todos_query_as);

    // let todo_insert_query = "INSERT INTO todos ( description ) VALUES ( $1 ) RETURNING id";
    //
    // let insert_query: i64 = sqlx::query_scalar(todo_insert_query)
    //     .bind("some task".to_string())
    //     .fetch_one(&pool)
    //     .await?;
    //
    // println!("{:?}", insert_query);

    Ok(())
}
