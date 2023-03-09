use crate::bubble::BubbleTransitionScene;
use crate::post::{Post, PostText};
use crate::shared::TransitionTimelines;
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
        let primary_color = "#818881";
        let title = "People who grew up in rich families which  myths about low-income were shattered for you?";
        let upvotes_count = "42.8k";
        let author = "u/niamh_mc";
        let when = "5 yr. ago";

        let vec: Vec<Box<dyn Scene>> = vec![
            Box::new(BubbleTransitionScene {
                title,
                fill: primary_color,
                upvotes_count,
                author,
                when,
                variant: crate::bubble::Variant::Title {
                    audio_file: "4-001.mp3",
                },
            }),
            Box::new(Post {
                title: "MrsDwightShrute",
                text: PostText::decode(include_str!("../words-1.json")),
                audio_file: "4-002.mp3",
                avatar: "avatar_1.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "4.7k",
                posted_when: "5 yr. ago",
                replies_count: "206",
            }),
            Box::new(Post {
                title: "[deleted]",
                text: PostText::decode(include_str!("../words-2.json")),
                audio_file: "4-003.mp3",
                avatar: "avatar_2.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "17.8k",
                posted_when: "5 yr. ago",
                replies_count: "195",
            }),
            Box::new(Post {
                title: "Back2Bach",
                text: PostText::decode(include_str!("../words-3.json")),
                audio_file: "4-004.mp3",
                avatar: "avatar_3.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "15.1k",
                posted_when: "5 yr. ago",
                replies_count: "178",
            }),
            Box::new(Post {
                title: "ZetaInk",
                text: PostText::decode(include_str!("../words-4.json")),
                audio_file: "4-005.mp3",
                avatar: "avatar_4.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "7.9k",
                posted_when: "5 yr. ago",
                replies_count: "418",
            }),
            Box::new(Post {
                title: "DeyCallMeCasper",
                text: PostText::decode(include_str!("../words-5.json")),
                audio_file: "4-006.mp3",
                avatar: "avatar_5.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "7.8k",
                posted_when: "4 yr. ago",
                replies_count: "108",
            }),
            Box::new(BubbleTransitionScene {
                title,
                upvotes_count,
                fill: primary_color,
                author,
                when,
                variant: crate::bubble::Variant::Exit,
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
