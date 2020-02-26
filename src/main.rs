use url::Url;

struct Podcast {
    id: String,
    rss: Result<Url, url::ParseError>,
    email: String,
    image: Result<Url, url::ParseError>,
    title: String,
}

struct Episode {
    id: String,
    link: Result<Url, url::ParseError>,
    audio: Result<Url, url::ParseError>,
    image: Result<Url, url::ParseError>,
    title: String,
    podcast: Podcast,
    thumbnail: Result<Url, url::ParseError>,
    description: String,
    pub_date_ms: u128,
    listennotes_url: Result<Url, url::ParseError>,
    audio_length_sec: u32,
    explicit_content: bool,
    maybe_audio_invalid: bool,
    listennotes_edit_url: Result<Url, url::ParseError>,
}

impl Episode {
    fn length(&self) -> String {
        let minutes = self.audio_length_sec / 60;
        let seconds = self.audio_length_sec % 60;

        format!("{:02}:{:02}", minutes, seconds)
    }
}

fn main() {
    let star_wars_7x7 = Podcast {
        id: String::from("4d3fe717742d4963a85562e9f84d8c79"),
        rss: Url::parse("https://www.listennotes.com/c/r/4d3fe717742d4963a85562e9f84d8c79"),
        email: String::from("fx7@sw7x7.com"),
        image: Url::parse("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.1400x1400.jpg"),
        title: String::from("Star Wars 7x7 | Star Wars News, Interviews, and More!"),
    };

    let episode775 = Episode {
        id: String::from("02f0123246c944e289ee2bb90804e41b"),
        link: Url::parse("http://sw7x7.libsyn.com/1775-happy-75th-birthday-george-lucas?utm_source=listennotes.com&utm_campaign=Listen+Notes&utm_medium=website"),
        audio: Url::parse("https://www.listennotes.com/e/p/02f0123246c944e289ee2bb90804e41b/"),
        image: Url::parse("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.1400x1400.jpg"),
        title: String::from("1,775: Happy 75th Birthday, George Lucas!"),
        podcast: star_wars_7x7,
        thumbnail: Url::parse("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.300x300.jpg"),
        description: String::from("<p>On the momentous occasion of George Lucas' 75th birthday, an extended episode in which I'm charting his involvement in the Star Wars franchise after the sale of Lucasfilm to Disney. Turns out, it isn't just that he can't leave it behind - it's that he's trying to guide the next generation of filmmakers in their efforts to tell new stories in the galaxy far, far away. Punch it!</p> <p>***I'm listener supported! Join the community at http://Patreon.com/sw7x7 to get access to bonus episodes and other insider rewards.*** </p>"),
        pub_date_ms: 1557817200274,
        listennotes_url: Url::parse("https://www.listennotes.com/e/02f0123246c944e289ee2bb90804e41b/"),
        audio_length_sec: 865,
        explicit_content: false,
        maybe_audio_invalid: false,
        listennotes_edit_url: Url::parse("https://www.listennotes.com/e/02f0123246c944e289ee2bb90804e41b/#edit")
    };

    println!(
        "{:#?}",
        episode775.length(),
    );
}
