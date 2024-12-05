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
}

pub mod read {
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