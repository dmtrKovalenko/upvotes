use crate::shared::{upvote_arrow, TransitionTimelines, EXIT_DURATION};
pub use fframes::{audio_data, fframes_context, frame, video::Video, Color};
use fframes::{
    serde::Deserialize, svgr, AnimateRuntimeInput, AnimationRuntime, AudioTimestamp, Easing, Scene,
    SceneInfo, Svgr,
};

#[derive(Deserialize, Debug)]
#[serde(crate = "fframes::serde")]
struct Word {
    word: String,
    start: f32,
    end: f32,
}

#[derive(Debug)]
pub struct PostText {
    whole: String,
    words: Vec<Word>,
}

impl PostText {
    pub fn decode(file: &str) -> Self {
        let words = serde_json::from_str::<Vec<Word>>(file).unwrap();
        let whole = words
            .iter()
            .map(|w| w.word.as_str())
            .collect::<Vec<&str>>()
            .join("")
            .trim()
            .to_owned();

        Self { whole, words }
    }
}

#[derive(Debug)]
pub struct Post<'a> {
    pub title: &'a str,
    pub audio_file: &'a str,
    pub avatar: &'a str,
    pub timelines: TransitionTimelines,
    pub posted_when: &'a str,
    pub upvote_count: &'a str,
    pub replies_count: &'a str,
}

const BREAK_OPTS: fframes::BreakLinesOpts = fframes::BreakLinesOpts {
    width: 680,
    line_height: 1.3,
    font_size: 54,
    font_family: "Noto Sans Medium",
    align: fframes::TextAlign::Left,
    fill: "black",
    font_stretch: fframes::FontStretch::Normal,
    font_style: fframes::FontStyle::Normal,
    font_weight: 500,
    x: "220",
    y: "360",
    dominant_baseline: "auto",
    text_anchor: "start",
};

impl Scene for Post<'_> {
    fn audio_map(&self, scene_info: &SceneInfo) -> fframes::AudioMap {
        fframes::AudioMap::from_iter(
            vec![
                Some((
                    self.audio_file,
                    (AudioTimestamp::Second(0), AudioTimestamp::Eof),
                )),
                (scene_info.index != scene_info.total_scenes_in_video - 2).then_some((
                    "woosh.mp3",
                    (
                        AudioTimestamp::Frame(scene_info.duration_in_frames - 20),
                        AudioTimestamp::Eof,
                    ),
                )),
                Some((
                    "click.mp3",
                    (
                        AudioTimestamp::Frame(scene_info.duration_in_frames - 80),
                        AudioTimestamp::Eof,
                    ),
                )),
            ]
            .into_iter()
            .flatten(),
        )
    }

    fn overlap(&self) -> fframes::Overlap {
        fframes::Overlap::Next(EXIT_DURATION)
    }

    fn duration(&self) -> fframes::Duration {
        fframes::Duration::FromAudio(self.audio_file) + fframes::Duration::Seconds(1.0)
    }

    fn render_frame(&self, mut frame: frame::Frame, ctx: &fframes_context::FFramesContext) -> Svgr {
        let range_subtitles = ctx.get_subtitles(format!("{}.vtt", self.audio_file));
        let word_subtitles = ctx.get_subtitles(format!("{}.word.vtt", self.audio_file));

        let text_structure = frame.text_break_lines_strcuture(
            ctx,
            range_subtitles
                .get_cue_stack(&frame, 750)
                .into_iter()
                .map(|c| c.text.as_str())
                .fold(String::new(), |acc, s| acc + s)
                .as_str(),
            &BREAK_OPTS,
        );

        // for some reason the dot is not rendered correctly in the editor preview canvas
        let delimiter = match ctx.mode {
            fframes::FFramesMode::EditorTimelinePreview => " ",
            _ => " · ",
        };

        let scene_info = ctx.get_scene_info(self).unwrap();
        let (translate_timeline, skew_timeline) = self.timelines.get(&frame, scene_info);

        let translate_x = frame.animate(translate_timeline);
        let skew_x = skew_timeline
            .as_ref()
            .map(|tl| frame.animate(tl))
            .unwrap_or(0.);

        let id = format!("avatar-{}", scene_info.index);

        fframes::svgr!(
            <g transform={format!("translate({translate_x}, 0) skewX({skew_x})")}>
                <circle
                    cx="140"
                    cy="260"
                    r="50"
                    fill={format!("url(#{})", id)}
                />

                 <pattern id={id} x="0%" y="0%" height="100%" width="100%" viewBox="0 0 100 100">
                   <image x="0%" y="0%" width="100" height="100" href={ctx.get_image_link(self.avatar)} />
                </pattern>

                <text x="220" y="265" dominant-baseline="middle" font-size="40">
                    {self.title}{delimiter}<tspan font-size="30" fill="#6b7280">{self.posted_when}</tspan>
                </text>

                {
                    if let Some(text_structure) = text_structure {
                        self.render_text(&frame, scene_info, &text_structure, word_subtitles)
                    } else {
                        Svgr::default()
                    }
                }

                {if scene_info.index == 1 {
                    svgr!(
                        <use href="#bubble" />
                        <use href="#test" />
                    )
                } else {
                    Svgr::default()
                }}
           </g>
        )
    }
}

impl Post<'_> {
    fn render_text(
        &self,
        frame: &fframes::Frame,
        scene_info: &fframes::SceneInfo,
        text_structure: &fframes::WrappedTextStructure,
        word_subtitles: &fframes::Subtitles,
    ) -> Svgr {
        let current_word_index = word_subtitles
            .get_cue_with_index_for_frame(frame)
            .into_iter()
            .next()
            .map(|(index, _)| index);

        let mut flatten_word_index = 0;
        let lines = text_structure
            .lines
            .iter()
            .map(|line| {
                fframes::svgr!(
                    <tspan x={BREAK_OPTS.x} y={BREAK_OPTS.y} dx={line.dx} dy={line.dy.to_string()}>
                        {line.words.iter().map(|word| {
                            let (weight, fill) = match current_word_index {
                                Some(current_word_index) if flatten_word_index == current_word_index  => (900, "#dc2626"),
                                Some(current_word_index) if flatten_word_index > current_word_index => (400, "#4b5563"),
                                _ => (400, "black")
                            };

                            flatten_word_index += 1;

                            fframes::svgr!(
                                <tspan font-weight={weight} fill={fill}>
                                   {word.as_str()}{" "}
                                </tspan>
                            )
                        }).collect::<Vec<_>>()}
                    </tspan>
                )
            })
            .collect::<Vec<_>>();

        let start = 290;
        let end = start
            + (lines.len() as f32 * BREAK_OPTS.line_height * BREAK_OPTS.font_size as f32) as usize
            - BREAK_OPTS.font_size;

        let on_second = frame.frame_to_second(scene_info.duration_in_frames - 80);

        let upvote_fill = frame.animate_runtime(AnimateRuntimeInput {
            on_second,
            from: Color::hex("#fff"),
            to: Color::hex("#fe4500"),
            animation_runtime: &AnimationRuntime::Linear(0.2),
        });

        let upvote_stroke = frame.animate_runtime(AnimateRuntimeInput {
            on_second,
            from: Color::hex("#888a8c"),
            to: Color::hex("#fe4500"),
            animation_runtime: &AnimationRuntime::Linear(0.2),
        });

        let downvote_arrow_offset = match self.upvote_count.len() {
            4 => 30,
            5 => 55,
            count => count * 11,
        };

        fframes::svgr!(
            <line x1="140" x2="140" y1={start + 20} y2={end + 140} stroke="#EDEFF1" stroke-width="7" />

            <text
              x={BREAK_OPTS.x}
              y={BREAK_OPTS.y}
              fill={BREAK_OPTS.fill}
              font-size={BREAK_OPTS.font_size}
              font-family={BREAK_OPTS.font_family}
              font-weight={BREAK_OPTS.font_weight}
             >
                {lines}
            </text>

            <g fill={upvote_fill} stroke={upvote_stroke}>
                {upvote_arrow(200, end + 90, 0)}
            </g>

            <text x="280" y={end + 135} font-family="Noto Sans Medium" dominant-baseline="middle" font-size="40">
                {self.upvote_count}
            </text>
            <g fill="none" stroke="#888a8c">
                {upvote_arrow(340 + downvote_arrow_offset, end + 94, 180)}
            </g>

            <text x="130" y={end + 210} dominant-baseline="middle" font-size="40" fill="#747677">
                {self.replies_count}
                " more replies"
            </text>

        )
    }
}
