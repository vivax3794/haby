use haby_api_wrapper::ApiWrapper;

#[tokio::test]
async fn version() {
    let client = ApiWrapper::default();
    client.clear_db().await;

    let version = client.get_version().await;

    assert_eq!(version, haby_core::VERSION);
}

#[tokio::test]
async fn get_habits() {
    let client = ApiWrapper::default();
    client.clear_db().await;

    let habits = client.get_habits().await;

    assert_eq!(habits, vec![]);
}

#[tokio::test]
async fn create_habit() {
    let client = ApiWrapper::default();
    client.clear_db().await;

    let habit = haby_core::api::CreateHabit {
        name: String::from("Test Habit"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    let habit = client.create_habit(habit).await.unwrap();
    let habbits = client.get_habits().await;
    assert_eq!(habbits, vec![habit]);
}

#[tokio::test]
async fn create_habit_error() {
    let client = ApiWrapper::default();
    client.clear_db().await;

    let habit = haby_core::api::CreateHabit {
        name: String::from("Test Habit"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    let habit = client.create_habit(habit).await.unwrap();
    let result = client.create_habit(habit.into()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn update_habit() {
    let client = ApiWrapper::default();
    client.clear_db().await;

    let habit = haby_core::api::CreateHabit {
        name: String::from("Test Habit"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    let mut habit = client.create_habit(habit).await.unwrap();
    habit.name = String::from("Updated Habit");
    client.update_habit(&habit).await.unwrap();

    let habits = client.get_habits().await;
    assert_eq!(habits, vec![habit]);
}

#[tokio::test]
async fn update_habit_errors() {
    let client = ApiWrapper::default();
    client.clear_db().await;

    let habit = haby_core::api::CreateHabit {
        name: String::from("1"),
        color: haby_core::Color { r: 0, g: 0, b: 0 },
        kind: haby_core::HabitKind::Habit,
        recording_type: haby_core::RecordingType::Point,
        every: Some(1),
    };

    let mut habit = client.create_habit(habit).await.unwrap();
    habit.name = String::from("2");
    let mut habit = client.create_habit(habit.into()).await.unwrap();
    habit.name = String::from("1");

    let res = client.update_habit(&habit).await;
    assert!(res.is_err());
}
