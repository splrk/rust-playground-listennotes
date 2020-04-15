use std::cmp::Ordering;
use std::rc::Rc;
use url::Url;

pub struct Podcast {
    pub id: String,
    pub rss: Option<Url>,
    pub email: String,
    pub image: Option<Url>,
    pub title: String,
}

pub struct Episode {
    pub id: String,
    pub link: Option<Url>,
    pub audio: Option<Url>,
    pub image: Option<Url>,
    pub title: String,
    pub podcast: Rc<Podcast>,
    pub thumbnail: Option<Url>,
    pub description: String,
    pub pub_date_ms: u128,
    pub listennotes_url: Option<Url>,
    pub audio_length_sec: u32,
    pub explicit_content: bool,
    pub maybe_audio_invalid: bool,
    pub listennotes_edit_url: Option<Url>,
}

impl Episode {
    pub fn length(&self) -> String {
        let minutes = self.audio_length_sec / 60;
        let seconds = self.audio_length_sec % 60;

        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn cmp(&self, other: &Episode) -> Ordering {
        self.pub_date_ms.cmp(&other.pub_date_ms)
    }
}