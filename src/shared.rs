use fframes::{self, KeyFramesAnimation};
use fframes::{KeyFrame, Svgr};
use once_cell::sync::OnceCell;
use std::fmt::Formatter;

pub fn upvote_arrow(x: usize, y: usize, rotate: usize) -> Svgr {
    fframes::svgr!(
        <svg y={y} x={x} width="70" height="70" viewBox="0 0 400 400" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M153.5 248V367C153.5 371.418 157.082 375 161.5 375H202H242.5C246.918 375 250.5 371.418 250.5 367V248C250.5 243.582 254.082 240 258.5 240H295.25H324.89C331.312 240 335.117 232.815 331.507 227.503L206.617 43.7359C203.442 39.0648 196.558 39.0648 193.383 43.7359L68.4929 227.503C64.8832 232.815 68.6876 240 75.1095 240H106.75H145.5C149.918 240 153.5 243.582 153.5 248Z"
                stroke-width="20"
                transform={format!("rotate({rotate})")} transform-origin="center center"
            />
        </svg>
    )
}

pub const EXIT_DURATION: f32 = 0.3;
pub struct TransitionTimelines(
    OnceCell<(KeyFramesAnimation<f32>, Option<KeyFramesAnimation<f32>>)>,
);

impl TransitionTimelines {
    pub fn init() -> Self {
        Self(OnceCell::new())
    }

    pub fn get(
        &self,
        frame: &fframes::Frame,
        scene_info: &fframes::SceneInfo,
    ) -> &(KeyFramesAnimation<f32>, Option<KeyFramesAnimation<f32>>) {
        self.0.get_or_init(|| {
            let easing = fframes::Easing::Linear(EXIT_DURATION);
            let enter_keyframe = KeyFrame {
                from: 1080.,
                to: 0.,
                start: 0.,
                easing: &easing,
            };

            let exit_keyframe = KeyFrame {
                start: frame.frame_to_second(scene_info.duration_in_frames) - EXIT_DURATION,
                from: 0.,
                to: -1080.,
                easing: &easing,
            };

            let translate_timeline = KeyFramesAnimation::new(if scene_info.index == 0 {
                vec![exit_keyframe]
            } else if scene_info.is_last {
                vec![enter_keyframe]
            } else {
                vec![enter_keyframe, exit_keyframe]
            });

            let skew_timeline =
                (!scene_info.is_last).then_some(KeyFramesAnimation::new(vec![KeyFrame {
                    start: frame.frame_to_second(scene_info.duration_in_frames) - EXIT_DURATION,
                    from: 0.,
                    to: 10.,
                    easing: &easing,
                }]));

            (translate_timeline, skew_timeline)
        })
    }
}

impl std::fmt::Debug for TransitionTimelines {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TransitionTimelines")
    }
}
