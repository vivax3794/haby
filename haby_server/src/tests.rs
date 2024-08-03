use rocket::local::asynchronous::Client;
use rocket::uri;

use super::*;

#[sqlx::test]
async fn version_returns_core_version(pool: sqlx::PgPool) {
    let client = Client::tracked(rocket_with_pool(pool)).await.unwrap();
    let response = client.get(uri!(get_version)).dispatch().await;

    let ver = response.into_string().await.unwrap();
    assert_eq!(ver, haby_core::VERSION);
}

#[sqlx::test]
async fn clear_db_clears_db(pool: sqlx::PgPool) {
    let client = Client::tracked(rocket_with_pool(pool)).await.unwrap();

    let habit = haby_core::api::CreateHabit {
        name: String::from("Test Habit"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    client
        .post(uri!(create_habit))
        .json(&habit)
        .dispatch()
        .await;
    client.post(uri!(clear_db)).dispatch().await;

    let response = client.get(uri!(get_habits)).dispatch().await;
    let res: Vec<haby_core::Habit> = response.into_json().await.unwrap();
    assert_eq!(res, vec![]);
}

#[sqlx::test]
async fn habit_table_starts_empty(pool: sqlx::PgPool) {
    let client = Client::tracked(rocket_with_pool(pool)).await.unwrap();
    let response = client.get(uri!(get_habits)).dispatch().await;

    let res: Vec<haby_core::Habit> = response.into_json().await.unwrap();
    assert_eq!(res, vec![]);
}

#[sqlx::test]
async fn habit_get_returns_inserted_habits(pool: sqlx::PgPool) {
    let client = Client::tracked(rocket_with_pool(pool)).await.unwrap();

    let habit = haby_core::api::CreateHabit {
        name: String::from("Test Habit"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    let res = client
        .post(uri!(create_habit))
        .json(&habit)
        .dispatch()
        .await;
    let status = res.status().class();
    assert!(status.is_success(), "Expected success, got {:?}", status);

    let id = res.into_string().await.unwrap().parse().unwrap();

    let response = client.get(uri!(get_habits)).dispatch().await;
    let res: Vec<haby_core::Habit> = response.into_json().await.unwrap();

    assert_eq!(res.len(), 1, "Returns only one habit after habit creation");
    let res = res.into_iter().next().unwrap();
    assert_eq!(res, habit.with_id(id));
}

#[sqlx::test]
async fn habit_insert_dupplicates(pool: sqlx::PgPool) {
    let client = Client::tracked(rocket_with_pool(pool)).await.unwrap();

    let habit = haby_core::api::CreateHabit {
        name: String::from("Test Habit"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    client
        .post(uri!(create_habit))
        .json(&habit)
        .dispatch()
        .await;

    let res = client
        .post(uri!(create_habit))
        .json(&habit)
        .dispatch()
        .await
        .status()
        .class();
    assert!(
        res.is_client_error(),
        "Expected client error, got {:?}",
        res
    );
}

#[sqlx::test]
async fn habit_update(pool: sqlx::PgPool) {
    let client = Client::tracked(rocket_with_pool(pool)).await.unwrap();

    let mut habit = haby_core::api::CreateHabit {
        name: String::from("Test Habit"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    let res = client
        .post(uri!(create_habit))
        .json(&habit)
        .dispatch()
        .await;
    let id = res.into_string().await.unwrap().parse().unwrap();

    habit.name = String::from("Nice!");
    habit.color.r = 255;

    client
        .put(uri!(update_habit(id)))
        .json(&habit)
        .dispatch()
        .await;

    let response = client.get(uri!(get_habits)).dispatch().await;
    let res: Vec<haby_core::Habit> = response.into_json().await.unwrap();

    assert_eq!(res.len(), 1);
    let res = res.into_iter().next().unwrap();
    assert_eq!(res, habit.with_id(id));
}

#[sqlx::test]
async fn habit_update_dupplicates(pool: sqlx::PgPool) {
    let client = Client::tracked(rocket_with_pool(pool)).await.unwrap();

    let mut habit = haby_core::api::CreateHabit {
        name: String::from("1"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    client
        .post(uri!(create_habit))
        .json(&habit)
        .dispatch()
        .await;

    habit.name = String::from("2");

    let res = client
        .post(uri!(create_habit))
        .json(&habit)
        .dispatch()
        .await;
    let id: i32 = res.into_string().await.unwrap().parse().unwrap();

    habit.name = String::from("1");
    let res = client
        .put(uri!(update_habit(id)))
        .json(&habit)
        .dispatch()
        .await
        .status()
        .class();
    assert!(
        res.is_client_error(),
        "Expected request to fail, got {:?}",
        res
    );
}
