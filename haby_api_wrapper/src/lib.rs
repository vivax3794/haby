pub use haby_core as core;
pub use haby_core::VERSION;

#[cfg(not(debug_assertions))]
const HOST: &str = "https://haby.vivax.dev/api";

#[cfg(debug_assertions)]
const HOST: &str = "http://localhost:8000";

#[derive(Default)]
pub struct ApiWrapper {
    client: reqwest::Client,
}

impl ApiWrapper {
    pub async fn clear_db(&self) {
        self.client
            .post(format!("{HOST}/test/clear"))
            .send()
            .await
            .unwrap();
    }

    pub async fn get_version(&self) -> String {
        let response = self
            .client
            .get(format!("{HOST}/version"))
            .send()
            .await
            .unwrap();
        response.text().await.unwrap()
    }

    pub async fn get_habits(&self) -> Vec<haby_core::Habit> {
        let response = self
            .client
            .get(format!("{HOST}/habits"))
            .send()
            .await
            .unwrap();
        response.json().await.unwrap()
    }

    pub async fn create_habit(
        &self,
        habit: haby_core::api::CreateHabit,
    ) -> Result<haby_core::Habit, String> {
        let response = self
            .client
            .post(format!("{HOST}/habits"))
            .json(&habit)
            .send()
            .await
            .unwrap();

        let success = response.status().is_success();
        let text = response.text().await.unwrap();

        if !success {
            return Err(text);
        }

        let id = text.parse().unwrap();
        Ok(habit.with_id(id))
    }

    pub async fn update_habit(&self, habit: &haby_core::Habit) -> Result<(), String> {
        let response = self
            .client
            .put(format!("{HOST}/habit/{}", habit.id))
            .json(&habit.as_create())
            .send()
            .await
            .unwrap();

        let success = response.status().is_success();

        if !success {
            let text = response.text().await.unwrap();
            return Err(text);
        }

        Ok(())
    }
}
