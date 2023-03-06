use crate::post::{Post, PostText};
use crate::shared::TransitionTimelines;
use crate::title::TitleScene;
pub use fframes::{audio_data, fframes_context, frame, video::Video};
use fframes::{AudioMap, AudioTimestamp, Scene, Svgr};

#[derive(Debug)]
pub struct UpvotesVideo {}

impl Video for UpvotesVideo {
    const FPS: usize = 60;
    const WIDTH: usize = 1080;
    const HEIGHT: usize = 1920;

    fn audio(&self) -> AudioMap {
        AudioMap::from([(
            "background2.mp3",
            (AudioTimestamp::Second(0), AudioTimestamp::Eof),
        )])
    }

    fn define_scenes(&self) -> fframes::Scenes {
        let vec: Vec<Box<dyn Scene>> = vec![
            Box::new(TitleScene {
                title: "Multilingual Redditors, What is your \"They didn't realize I spoke their language\" story?"
                    .to_string(),
                upvote_count: "50.5k".to_owned(),
                posted_by: "u/Trumpstered".to_owned(),
                when: "5 years ago".to_string(),
                timelines: TransitionTimelines::init(),
                comment_count: "16.2k".to_owned(),
            }),
            Box::new(Post {
                title: "tuanomsok",
                text: PostText::decode(include_str!("../words-0.json")),
                audio_file: "2-001.mp3",
                avatar: "avatar_1.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "38.2k",
                posted_when: "5 yr. ago",
                replies_count: "244",
            }),
            Box::new(Post {
                title: "cninamon",
                text: PostText::decode(include_str!("../words-1.json")),
                audio_file: "2-002.mp3",
                avatar: "avatar_2.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "34.7k",
                posted_when: "5 yr. ago",
                replies_count: "162",
            }),
        ];

        fframes::Scenes::from(vec)
    }

    fn render_frame(&self, frame: frame::Frame, ctx: &fframes_context::FFramesContext) -> Svgr {
        fframes::svgr!(
           <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width={Self::WIDTH}
            height={Self::HEIGHT}
            font-family="Noto Sans"
          >
            <image href={ctx.get_image_link("background.png")} width={1080} height={1920} x="0" y="0" />
            {ctx.render_scenes(&frame)}
          </svg>
        )
    }
}
