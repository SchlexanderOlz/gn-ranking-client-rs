pub mod create {
    use serde::Serialize;
    use serde_json::Value;

    #[derive(Serialize)]
    pub struct Game {
        pub game_name: String,
        pub game_mode: String,
        pub max_stars: i32,
        pub description: String,
        pub performances: Vec<Performance>,
    }

    #[derive(Serialize)]
    pub struct Performance {
        pub name: String,
        pub weight: i32,
    }

    #[derive(Serialize)]
    pub struct PlayerPerformance {
      pub name: String,
      pub count: i32,
    }

    #[derive(Serialize)]
    pub struct PlayerMatch {
        pub player_id: String,
        pub player_performances: Vec<PlayerPerformance>,
    }

    #[derive(Serialize)]
    pub struct Match {
        pub game_name: String,
        pub game_mode: String,

        pub player_match_list: Vec<PlayerMatch>,
    }

    #[derive(Serialize)]
    pub struct ReplayData {
        pub match_id: String,
        pub replay_data: Value,
    }
}

pub mod read {
    use chrono::NaiveDateTime;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Game {
        pub id: String,
        pub game_name: String,
        pub game_mode: String,
        pub max_stars: i32,
        pub description: String,
        pub performances: Vec<Performance>,
    }

    #[derive(Deserialize)]
    pub struct Performance {
        pub name: String,
        pub weight: i32,
    }

    #[derive(Deserialize)]
    pub struct PlayerPerformance {
        pub name: String,
        pub count: i32,
    }

    #[derive(Deserialize)]
    pub struct PlayerMatch {
        pub player_id: String,
        pub before_game_stars: i32,
        pub after_game_stars: i32,
        pub before_stars: i32,
        pub after_stars: i32,
        pub player_performances: Vec<PlayerPerformance>,
    }

    #[derive(Deserialize)]
    pub struct Match {
        pub _id: String,
        pub game_name: String,
        pub game_mode: String,
        pub player_matches: Vec<PlayerMatch>,
        pub timestamp: NaiveDateTime
    }

    #[derive(Deserialize)]
    pub struct PlayerGame {
        pub _id: String,
        pub game_name: String,
        pub game_mode: String,
        pub player_id: String,
        pub game_stars: i32,
        pub performances: Vec<PlayerPerformance>,
    }


}

pub mod filter {
    #[derive(serde::Serialize)]
    pub struct Game {
        pub game_name: Option<String>,
        pub game_mode: Option<String>,
    }

    #[derive(serde::Serialize)]
    pub struct PlayerGame {
        pub player_id: Option<String>,
        pub game_name: Option<String>,
        pub game_mode: Option<String>,
        pub in_order: Option<bool>,
    }
}


#[derive(serde::Deserialize)]
pub(crate) struct Route {
    pub path: String,
}