pub mod create {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Game {
        #[serde(rename = "gameName")]
        pub game_name: String,
        #[serde(rename = "gameMode")]
        pub game_mode: String,
        #[serde(rename = "maxStars")]
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
        #[serde(rename = "playerId")]
        pub player_id: String,
        pub player_performances: Vec<PlayerPerformance>,
    }

    #[derive(Serialize)]
    pub struct Match {
        #[serde(rename = "gameName")]
        pub game_name: String,
        #[serde(rename = "gameMode")]
        pub game_mode: String,

        #[serde(rename = "playerMatchList")]
        pub player_match_list: Vec<PlayerMatch>,
    }
}

pub mod read {
    use chrono::NaiveDateTime;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Game {
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(rename = "gameName")]
        pub game_name: String,
        #[serde(rename = "gameMode")]
        pub game_mode: String,
        #[serde(rename = "maxStars")]
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
        #[serde(rename = "playerId")]
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
        #[serde(rename = "gameName")]
        pub game_name: String,
        #[serde(rename = "gameMode")]
        pub game_mode: String,
        pub player_matches: Vec<PlayerMatch>,
        pub timestamp: NaiveDateTime
    }


}

pub mod filter {
    #[derive(serde::Serialize)]
    pub struct Game {
        pub game_name: Option<String>,
        pub game_mode: Option<String>,
    }
}


#[derive(serde::Deserialize)]
pub(crate) struct Route {
    pub path: String,
}