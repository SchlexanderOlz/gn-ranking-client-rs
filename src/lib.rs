use std::{collections::HashMap, env, fs, path::PathBuf};

use reqwest;
use serde::de::DeserializeOwned;

pub mod models;


pub struct RankingClient {
    client: reqwest::Client,
    api_key: String,
    routes: HashMap<String, models::Route>,
}

impl RankingClient {

    pub fn new(api_key: String) -> Self {
        let routes = Self::load_default_routes();

        Self {
            client: reqwest::Client::new(),
            api_key,
            routes,
        }
    }

    fn load_default_routes() -> HashMap<String, models::Route> {
        let content = Self::substitute_env_vars(include_str!("../routes.yml"));
        serde_yaml::from_str(&content).expect("Failed to parse routes file")
    }

    fn substitute_env_vars(content: &str) -> String {
        let mut substituted = content.to_string();
        for (key, value) in env::vars() {
            let placeholder = format!("${{{}}}", key);
            substituted = substituted.replace(&placeholder, &value);
        }
        substituted
    }


    pub fn load_route_conf(&mut self, path: &str) {
        let routes = Self::load_routes(PathBuf::from(path));
        self.routes = routes;
    }

    pub async fn create<'a, M, R>(&self, model: M, route_id: &str) -> Result<R, Box<dyn std::error::Error>>
    where
        M: serde::Serialize,
        R: DeserializeOwned
    {
        let url = &self.routes.get(route_id).unwrap().path;

        let response = self.client.post(url).json(&model).header("access_token", &self.api_key).send().await?.error_for_status()?;

        Ok(response.json().await?)
    }

    pub async fn read<'a, F, R>(&self, filter: F, route_id: &str) -> Result<R, Box<dyn std::error::Error>>
    where
        F: serde::Serialize,
        R: DeserializeOwned
    {
        let url = &self.routes.get(route_id).unwrap().path;

        let response = self.client.get(url).query(&filter).header("access_token", &self.api_key).send().await?.error_for_status()?;

        Ok(response.json().await?)
    }

    pub async fn game_init(&self, game: models::create::Game) -> Result<models::read::Game, Box<dyn std::error::Error>> {
        Ok(self.create(game, "game_init").await?)
    }

    pub async fn game_read(&self, filter: models::filter::Game) -> Result<Vec<models::read::Game>, Box<dyn std::error::Error>> {
        Ok(self.read(filter, "game_read").await?)
    }

    pub async fn match_init(&self, _match: models::create::Match) -> Result<models::read::Match, Box<dyn std::error::Error>> {
        Ok(self.create(_match, "match_init").await?)
    }

    pub async fn player_games_read(&self, filter: models::filter::PlayerGame) -> Result<Vec<models::read::PlayerGame>, Box<dyn std::error::Error>> {
        Ok(self.read(filter, "player_games_read").await?)
    }

    pub async fn player_stars(&self, player_id: &str, game_name: &str, game_mode: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let filter = models::filter::PlayerGame {
            player_id: Some(player_id.to_string()),
            game_name: Some(game_name.to_string()),
            game_mode: Some(game_mode.to_string()),
            in_order: Some(true),
        };
        Ok(self.player_games_read(filter).await?.first().ok_or_else(|| "Game not found")?.game_stars)
    }

    fn load_routes(path: PathBuf) -> HashMap<String, models::Route> {
        let content = fs::read_to_string(path).expect("Failed to read routes file");
        let content = Self::substitute_env_vars(content.as_str());
        serde_yaml::from_str(&content).expect("Failed to parse routes file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
