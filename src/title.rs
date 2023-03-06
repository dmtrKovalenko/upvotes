use fframes::{BreakLinesOpts, Scene};

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
          <g transform={format!("translate({translate_x}, 0) skewX({skew_x})")}>
            <g fill="none" stroke="#888a8c">
              {upvote_arrow(100, Y_PADDING + 145, 0)}
              {upvote_arrow(100, Y_PADDING + 300, 180)}
            </g>

            <text x="90" y={Y_PADDING + 280} font-size="40" font-weight="medium" fill="black" font-family="Noto Sans Medium">
                {self.upvote_count.as_str()}
            </text>

            <text y={Y_PADDING} x={X_PADDING} fill="gray" font-size="40">
              {format!("Posted by {} {}", self.posted_by, self.when)}
            </text>

            <image
                y={Y_PADDING + 20}
                x={X_PADDING - 10}
                width="90"
                height="90"
                href={ctx.get_image_link("brand.png")}
            />
            <image
                y={Y_PADDING + 20}
                x="290"
                height="90"
                href={ctx.get_image_link("awards.png")}
            />

            {
                frame.text_break_lines(
                ctx,
                    self.title.as_str(),
                    &BreakLinesOpts {
                        width: 780,
                        font_family: "Noto Sans Medium",
                        font_size: 60,
                        x: &X_PADDING.to_string(),
                        y: &(Y_PADDING + 200).to_string(),
                        align: fframes::TextAlign::Left,
                        font_weight: 500,
                        fill: "#000",
                        line_height: 1.3,
                        ..Default::default()
                    },
                ).unwrap_or_default()
            }

            <image
                href={ctx.get_image_link("speechBubble.png")}
                x={X_PADDING - 10}
                y={Y_PADDING + 490}
                width="90"
                height="90"
            />

            <text x="290" y={Y_PADDING + 550} font-size="44" font-weight="medium" fill="#888a8c" font-family="Noto Sans Medium">
                {self.comment_count.as_str()}
                " Comments"
            </text>
          </g>
        )
    }
}
