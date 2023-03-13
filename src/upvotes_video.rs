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
        let primary_color = "#b4b3fb";
        let title = r#"Bill Gates once said that the lazy person will always find the easiest way to do a difficult job. What's the real-life example of this?"#;
        let upvotes_count = "42.8k";
        let author = "u/lauvnoodles";
        let when = "3 years ago";

        let vec: Vec<Box<dyn Scene>> = vec![
            Box::new(BubbleTransitionScene {
                title,
                fill: primary_color,
                upvotes_count,
                author,
                when,
                variant: crate::bubble::Variant::Title {
                    audio_file: "5-001.mp3",
                },
            }),
            Box::new(Post {
                title: "Downvotesdarksouls",
                audio_file: "5-002.mp3",
                avatar: "avatar_1.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "56.5k",
                posted_when: "3 yr. ago",
                replies_count: "158",
            }),
            Box::new(Post {
                title: "january21st",
                audio_file: "5-003.mp3",
                avatar: "avatar_2.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "35.8k",
                posted_when: "3 yr. ago",
                replies_count: "109",
            }),
            Box::new(Post {
                title: "FutureRenaissanceMan",
                audio_file: "5-004.mp3",
                avatar: "avatar_3.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "17.9k",
                posted_when: "3 yr. ago",
                replies_count: "56",
            }),
            Box::new(Post {
                title: "Rino_samuel",
                audio_file: "5-005.mp3",
                avatar: "avatar_4.png",
                timelines: TransitionTimelines::init(),
                upvote_count: "13.0k",
                posted_when: "3 yr. ago",
                replies_count: "64",
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
