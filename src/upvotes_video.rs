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
        let primary_color = "#fcf8f5";
        let title = r#"What yells I have no life?"#;
        let upvotes_count = "24.8k";
        let author = "u/11pxny4";
        let when = "2 days ago";

        let vec: Vec<Box<dyn Scene>> = vec![
            Box::new(BubbleTransitionScene {
                title,
                fill: primary_color,
                upvotes_count,
                author,
                when,
                variant: crate::bubble::Variant::Title {
                    audio_file: "7-001.mp3",
                },
            }),
            Box::new(Post {
                title: "purringfox",
                audio_file: "7-002.mp3",
                avatar: "avatar_1.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "4.3k",
                posted_when: "2 d. ago",
                replies_count: "158",
            }),
            Box::new(Post {
                title: "Elin-Calliel",
                audio_file: "7-006.mp3",
                avatar: "avatar_6.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "20.0k",
                posted_when: "2 d. ago",
                replies_count: "82",
            }),
            Box::new(Post {
                title: "nowhereman531",
                audio_file: "7-003.mp3",
                avatar: "avatar_2.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "12.9k",
                posted_when: "2 d. ago",
                replies_count: "116",
            }),
            Box::new(Post {
                title: "Portarossa",
                audio_file: "7-004.mp3",
                avatar: "avatar_3.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "13.5k",
                posted_when: "2 d. ago",
                replies_count: "204",
            }),
            Box::new(Post {
                title: "hello_friend",
                audio_file: "7-005.mp3",
                avatar: "avatar_4.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "10.1k",
                posted_when: "2 d. ago",
                replies_count: "80",
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
