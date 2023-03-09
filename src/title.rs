use fframes::{BreakLinesOpts, Easing, Scene};

use crate::shared::{upvote_arrow, TransitionTimelines};

#[derive(Debug)]
pub struct TitleScene {
    pub title: String,
    pub posted_by: String,
    pub when: String,
    pub upvote_count: String,
    pub comment_count: String,
    pub timelines: TransitionTimelines,
}

impl Scene for TitleScene {
    fn duration(&self) -> fframes::video::Duration {
        fframes::video::Duration::FromAudio("title.mp3") + fframes::video::Duration::Seconds(0.4)
    }

    fn audio_map(&self, _scene_info: &fframes::SceneInfo) -> fframes::audio_map::AudioMap {
        fframes::AudioMap::from([(
            "title.mp3",
            (
                fframes::AudioTimestamp::Second(0),
                fframes::AudioTimestamp::Eof,
            ),
        )])
    }

    fn render_frame(
        &self,
        mut frame: fframes::frame::Frame,
        ctx: &fframes::FFramesContext,
    ) -> fframes::Svgr {
        let scene_info = ctx.get_scene_info(self).unwrap();
        let (translate_timeline, skew_timeline) = self.timelines.get(&frame, &scene_info);

        let translate_x = frame.animate(translate_timeline);
        let skew_x = skew_timeline
            .as_ref()
            .map(|tl| frame.animate(tl))
            .unwrap_or(0.);

        const X_PADDING: usize = 220;
        const Y_PADDING: usize = 400;

        fframes::svgr!(
        //   <g transform={format!("translate({translate_x}, 0) skewX({skew_x})")}>
            // <rect
            //   fill="#FBAE3C"
            //   x="0"
            //   y="0"
            //   width="1080"
            //   height="1920"
            //   id="bubble"
            // />

            // <clipPath id="containerClip">
            //    <use href="#bubble" />
            // </clipPath>

            // <g clip-path="url(#containerClip)">
            //     <image
            //         y="20%"
            //         x={1080 / 2 - 600 / 2}
            //         width="600"
            //         height="600"
            //         href={ctx.get_image_link("test.png")}
            //     />

            //     <text
            //        x="50%"
            //        y="55%"
            //        dominant-baseline="middle"
            //        text-anchor="middle"
            //        font-family="Noto Sans Medium"
            //        font-size="60"
            //        fill="#3f3f46"
            //     >
            //        "r/AskReddit"
            //     </text>
            //     <text
            //        x="50%"
            //        y="62%"
            //        dominant-baseline="middle"
            //        text-anchor="middle"
            //        font-family="KyivType Serif"
            //        font-weight="black"
            //        font-size="160"
            //     >
            //        "Subscribe"
            //     </text>
            // </g>

            // <g fill="none" stroke="#3f3f46">
            //   {upvote_arrow(100, Y_PADDING + 155, 0)}
            //   {upvote_arrow(100, Y_PADDING + 300, 180)}
            // </g>

            // <text x="90" y={Y_PADDING + 280} font-size="40" font-weight="medium" fill="black" font-family="Noto Sans Medium">
            //     {self.upvote_count.as_str()}
            // </text>

            // <text y={Y_PADDING} x={X_PADDING} fill="#3f3f46" font-size="40">
            //   {format!("Posted by {} {}", self.posted_by, self.when)}
            // </text>

            // <image
            //     y={Y_PADDING + 20}
            //     x={X_PADDING - 10}
            //     width="90"
            //     height="90"
            //     href={ctx.get_image_link("brand.png")}
            // />

            // {
            //     frame.text_break_lines(
            //     ctx,
            //         self.title.as_str(),
            //         &BreakLinesOpts {
            //             width: 780,
            //             font_family: "Noto Sans Medium",
            //             font_size: 68,
            //             x: &X_PADDING.to_string(),
            //             y: &(Y_PADDING + 200).to_string(),
            //             align: fframes::TextAlign::Left,
            //             font_weight: 500,
            //             fill: "#000",
            //             line_height: 1.3,
            //             ..Default::default()
            //         },
            //     ).unwrap_or_default()
            // }

            // <svg
            //     x={X_PADDING - 10}
            //     y={Y_PADDING + 510}
            //     width="80"
            //     height="80"
            //     viewBox="0 0 400 400"
            //     fill="none"
            //     xmlns="http://www.w3.org/2000/svg"
            // >
            //     <path
            //         d="M136.944 292.438L132.526 329.847C131.727 336.615 139.214 341.219 144.893 337.452L181 313.5L218.355 285.129C219.747 284.072 221.446 283.5 223.194 283.5H311C328.673 283.5 343 269.173 343 251.5V167V86C343 68.3269 328.673 54 311 54H199.5H88C70.3269 54 56 68.3269 56 86V251.5C56 269.173 70.3269 283.5 88 283.5H101.5H129C133.79 283.5 137.506 287.681 136.944 292.438Z"
            //         stroke="#3f3f46"
            //         stroke-width="24"
            //     />
            // </svg>

            // <text x="290" y={Y_PADDING + 565} font-size="44" font-weight="medium" fill="#3f3f46" font-family="Noto Sans Medium">
            //     {self.comment_count.as_str()}
            //     " comments"
            // </text>
        //   </g>
        )
    }
}
