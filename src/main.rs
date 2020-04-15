use url::Url;
use std::ops::Add;
use std::rc::Rc;

mod listen_notes;
use listen_notes::{Podcast, Episode};


fn parse_url(url: &str) -> Option<Url> {
    match Url::parse(url) {
        Err(_) => None,
        Ok(parsed_url) => Some(parsed_url),
    }
}


fn main() {
    let star_wars_7x7 = Rc::new(Podcast {
        id: String::from("4d3fe717742d4963a85562e9f84d8c79"),
        rss: parse_url("https://www.listennotes.com/c/r/4d3fe717742d4963a85562e9f84d8c79"),
        email: String::from("fx7@sw7x7.com"),
        image: parse_url("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.1400x1400.jpg"),
        title: String::from("Star Wars 7x7 | Star Wars News, Interviews, and More!"),
     });

    let episodes = vec![
        Episode {
            id: String::from("02f0123246c944e289ee2bb90804e41b"),
            link: parse_url("http://sw7x7.libsyn.com/1775-happy-75th-birthday-george-lucas?utm_source=listennotes.com&utm_campaign=Listen+Notes&utm_medium=website"),
            audio: parse_url("https://www.listennotes.com/e/p/02f0123246c944e289ee2bb90804e41b/"),
            image: parse_url("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.1400x1400.jpg"),
            title: String::from("1,775: Happy 75th Birthday, George Lucas!"),
            podcast: Rc::clone(&star_wars_7x7),
            thumbnail: parse_url("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.300x300.jpg"),
            description: String::from("<p>On the momentous occasion of George Lucas' 75th birthday, an extended episode in which I'm charting his involvement in the Star Wars franchise after the sale of Lucasfilm to Disney. Turns out, it isn't just that he can't leave it behind - it's that he's trying to guide the next generation of filmmakers in their efforts to tell new stories in the galaxy far, far away. Punch it!</p> <p>***I'm listener supported! Join the community at http://Patreon.com/sw7x7 to get access to bonus episodes and other insider rewards.*** </p>"),
            pub_date_ms: 1557817200274,
            listennotes_url: parse_url("https://www.listennotes.com/e/02f0123246c944e289ee2bb90804e41b/"),
            audio_length_sec: 865,
            explicit_content: false,
            maybe_audio_invalid: false,
            listennotes_edit_url: parse_url("https://www.listennotes.com/e/02f0123246c944e289ee2bb90804e41b/#edit")
        },

        Episode {
            id: String::from("4e7c59e10e4640b98f2f3cb1777dbb43"),
            link: parse_url("http://sw7x7.libsyn.com/864-part-2-of-my-new-conversation-with-bobby-roberts?utm_source=listennotes.com&utm_campaign=Listen+Notes&utm_medium=website"),
            audio: parse_url("https://www.listennotes.com/e/p/4e7c59e10e4640b98f2f3cb1777dbb43/"),
            image: parse_url("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.1400x1400.jpg"),
            title: String::from("864: Part 2 of My (New) Conversation With Bobby Roberts"),
            podcast: Rc::clone(&star_wars_7x7),
            thumbnail: parse_url("https://cdn-images-1.listennotes.com/podcasts/star-wars-7x7-star-wars-news-interviews-and-AIg3cZVKCsL.300x300.jpg"),
            description: String::from("<p>The second half of my latest conversation with Bobby Roberts, Podcast Emeritus from Full of Sith and now Star Wars \"Podcast Force Ghost at Large.\" Punch it!</p> <p>***We’re listener supported! Go to http://Patreon.com/sw7x7 to donate to the Star Wars 7x7 podcast, and you’ll get some fabulous rewards for your pledge.*** </p> <p>Check out SW7x7.com for full Star Wars 7x7 show notes and links, and to comment on any of the content of this episode! If you like what you've heard, please leave us a rating or review on iTunes or Stitcher, which will also help more people discover this Star Wars podcast.</p> <p>Don't forget to join the Star Wars 7x7 fun on Facebook at Facebook.com/SW7x7, and follow the breaking news Twitter feed at Twitter.com/SW7x7Podcast. We're also on Pinterest and Instagram as \"SW7x7\" too, and we'd love to connect with you there!</p>"),
            pub_date_ms: 1479110401045,
            listennotes_url: parse_url("https://www.listennotes.com/e/4e7c59e10e4640b98f2f3cb1777dbb43/"),
            audio_length_sec: 2447,
            explicit_content: false,
            maybe_audio_invalid: false,
            listennotes_edit_url: None,
        },
    ];

    let display = String::from(&episodes[0].title);
    let display = Add::add(display, " (");
    let display = Add::add(display, &episodes[0].length().to_string());
    let display = Add::add(display, ")");


    println!(
        "{} {:?} {}, {}",
        display,
        episodes[0].cmp(&episodes[1]),
        episodes[1].title,
        episodes[1].length()
    );
}
