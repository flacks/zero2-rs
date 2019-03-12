use std::collections::HashMap;

pub mod activity;
pub mod airing_schedule;
pub mod character;
pub mod media;
pub mod user;


pub type Variables = HashMap<String, String>;

#[derive(Serialize)]
pub struct QueryBody {
    pub query: String,
    pub variables: Variables,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    #[serde(rename = "airingSchedules")]
    pub airing_schedule: Option<Vec<airing_schedule::AiringSchedule>>,

    pub characters: Option<Vec<character::Character>>,

    pub media: Option<Vec<media::Media>>,

    pub users: Option<Vec<user::User>>,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            airing_schedule: None,
            characters: None,
            media: None,
            users: None,
        }
    }
}

impl Page {
    pub fn airing_schedule(self) -> Vec<airing_schedule::AiringSchedule> {
        match self.airing_schedule {
            Some(airing_schedule) => airing_schedule,
            None => vec![]
        }
    }

    pub fn media(self) -> Vec<media::Media> {
        match self.media {
            Some(media) => media,
            None => vec![]
        }
    }

    pub fn users(self) -> Vec<user::User> {
        match self.users {
            Some(user) => user,
            None => vec![]
        }
    }

    pub fn characters(self) -> Vec<character::Character> {
        match self.characters {
            Some(character) => character,
            None => vec![]
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(default, rename = "Page")]
    pub page: Page,

    #[serde(default, rename = "Activity")]
    pub activity: Option<activity::Activity>,

    #[serde(default, rename = "Character")]
    pub character: Option<character::Character>,

    #[serde(default, rename = "Media")]
    pub media: Option<media::Media>,

    #[serde(default, rename = "User")]
    pub user: Option<user::User>
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Data,
}
