use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, launch, post, put, routes, State};

struct Db(sqlx::PgPool);

const DB_HOST: &str = "postgresql://postgres:viv@db:5432";

impl Db {
    async fn prod() -> Self {
        let pool = sqlx::PgPool::connect(DB_HOST).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        Self(pool)
    }

    fn with(pool: sqlx::PgPool) -> Self {
        Self(pool)
    }
}

/// Get the `core` version that is in use
#[get("/version")]
fn get_version() -> &'static str {
    haby_core::VERSION
}

#[get("/habits")]
async fn get_habits(pool: &State<Db>) -> Json<Vec<haby_core::Habit>> {
    let habits = sqlx::query_as!(
        haby_core::Habit,
        r#"SELECT id,
                name,
                color,
                kind AS "kind: haby_core::HabitKind",
                recording_type AS "recording_type: haby_core::RecordingType",
                every
        FROM habits"#
    )
    .fetch_all(&pool.0)
    .await
    .unwrap();

    Json(habits)
}

#[post("/habits", data = "<habit>")]
async fn create_habit(
    habit: Json<haby_core::api::CreateHabit>,
    pool: &State<Db>,
) -> Result<String, (Status, String)> {
    let res = sqlx::query!(
        r#"
        INSERT INTO habits (name, color, kind, recording_type, every)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        habit.name,
        habit.color.to_hex(),
        habit.kind as haby_core::HabitKind,
        habit.recording_type as haby_core::RecordingType,
        habit.every,
    )
    .fetch_one(&pool.0)
    .await;

    match res {
        Ok(res) => Ok(res.id.to_string()),
        Err(err) => Err((Status::BadRequest, err.to_string())),
    }
}

#[put("/habit/<id>", data = "<habit>")]
async fn update_habit(
    habit: Json<haby_core::api::CreateHabit>,
    id: i32,
    pool: &State<Db>,
) -> Result<(), (Status, String)> {
    let res = sqlx::query!(
        r#"
            UPDATE habits
            SET name=$2,color=$3,kind=$4,recording_type=$5,every=$6
            WHERE id=$1
        "#,
        id,
        habit.name,
        habit.color.to_hex(),
        habit.kind as haby_core::HabitKind,
        habit.recording_type as haby_core::RecordingType,
        habit.every,
    )
    .execute(&pool.0)
    .await;

    match res {
        Ok(_) => Ok(()),
        Err(err) => Err((Status::BadRequest, err.to_string())),
    }
}

#[post("/test/clear")]
async fn clear_db(pool: &State<Db>) {
    sqlx::query!("TRUNCATE TABLE events, habits;",)
        .execute(&pool.0)
        .await
        .unwrap();
}

fn rocket_no_db() -> rocket::Rocket<rocket::Build> {
    let cors = rocket_cors::CorsOptions::default();
    rocket::Rocket::build()
        .mount(
            "/",
            routes![
                get_version,
                get_habits,
                create_habit,
                update_habit,
                clear_db
            ],
        )
        .attach(cors.to_cors().unwrap())
}

fn rocket_with_pool(pool: sqlx::PgPool) -> rocket::Rocket<rocket::Build> {
    rocket_no_db().manage(Db::with(pool))
}

#[launch]
async fn rocket() -> _ {
    rocket_no_db().manage(Db::prod().await)
}

#[cfg(test)]
mod tests;
