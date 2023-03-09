use crate::bubble::BubbleTransitionScene;
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
        let title = "People who have left third-world countries for first-world countries, what surprised you the most?";

        let vec: Vec<Box<dyn Scene>> = vec![
            Box::new(BubbleTransitionScene {
                title,
                fill: "#FBAE3C",
                variant: crate::bubble::Variant::Title {
                    audio_file: "3-001.mp3",
                },
            }),
            Box::new(Post {
                title: "kezinaur",
                text: PostText::decode(include_str!("../words-1.json")),
                audio_file: "3-003.mp3",
                avatar: "avatar_1.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "8.2k",
                posted_when: "2 yr. ago",
                replies_count: "43",
            }),
            Box::new(Post {
                title: "SharedHeadEdd",
                text: PostText::decode(include_str!("../words-0.json")),
                audio_file: "3-002.mp3",
                avatar: "avatar_2.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "5.5k",
                posted_when: "5 yr. ago",
                replies_count: "65",
            }),
            Box::new(Post {
                title: "milanesaconpapas",
                text: PostText::decode(include_str!("../words-2.json")),
                audio_file: "3-004.mp3",
                avatar: "avatar_3.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "5.4k",
                posted_when: "2 yr. ago",
                replies_count: "28",
            }),
            Box::new(Post {
                title: "mlyfen",
                text: PostText::decode(include_str!("../words-3.json")),
                audio_file: "3-005.mp3",
                avatar: "avatar_4.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "4.3k",
                posted_when: "2 yr. ago",
                replies_count: "28",
            }),
            Box::new(BubbleTransitionScene {
                fill: "#FBAE3C",
                title,
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
