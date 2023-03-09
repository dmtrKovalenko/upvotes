use fframes::{
    svgr, AudioMap, AudioTimestamp, BreakLinesOpts, Easing, KeyFrame, KeyFramesAnimation, Scene,
    Svgr, TextAlign,
};

#[derive(Debug)]
pub enum Variant<'a> {
    Exit,
    Title { audio_file: &'a str },
}

#[derive(Debug)]
pub struct BubbleTransitionScene<'a> {
    pub fill: &'a str,
    pub variant: Variant<'a>,
    pub title: &'a str,
}

impl Scene for BubbleTransitionScene<'_> {
    fn audio_map(&self, _: &fframes::SceneInfo) -> fframes::audio_map::AudioMap {
        match self.variant {
            Variant::Exit => AudioMap::none(),
            Variant::Title { audio_file, .. } => {
                AudioMap::from([(audio_file, (AudioTimestamp::Second(0), AudioTimestamp::Eof))])
            }
        }
    }

    fn duration(&self) -> fframes::video::Duration {
        match self.variant {
            Variant::Exit => fframes::Duration::Seconds(2.7),
            Variant::Title { audio_file, .. } => fframes::Duration::FromAudio(audio_file),
        }
    }

    fn overlap(&self) -> fframes::Overlap {
        match self.variant {
            Variant::Title { .. } => fframes::Overlap::Next(0.94),
            Variant::Exit => fframes::Overlap::Next(0.9),
        }
    }

    fn render_frame(
        &self,
        mut frame: fframes::frame::Frame,
        ctx: &fframes::FFramesContext,
    ) -> fframes::Svgr {
        const SPRING: Easing = Easing::Spring2(1.0, 80., 16.);
        let scene_info = ctx.get_scene_info(self).unwrap();
        let scale = match self.variant {
            Variant::Exit => frame.animate(&fframes::timeline!(
                on 0.1, val 0.1 => 12., &SPRING
            )),
            Variant::Title { .. } => {
                let start = frame.frame_to_second(scene_info.duration_in_frames) - 0.9;
                let timeline = KeyFramesAnimation::new(vec![KeyFrame {
                    easing: &SPRING,
                    start,
                    from: 16.,
                    to: 0.,
                }]);

                frame.animate(&timeline)
            }
        };
        let text_translate = match self.variant {
            Variant::Exit => {
                let start = frame.frame_to_second(scene_info.duration_in_frames) - 1.5;
                let timeline = KeyFramesAnimation::new(vec![KeyFrame {
                    easing: &SPRING,
                    start,
                    from: 0.,
                    to: 1080.,
                }]);

                frame.animate(&timeline)
            }
            Variant::Title { .. } => 0.,
        };

        let translate = match self.variant {
            Variant::Exit => "",
            Variant::Title { .. } => "translate(700, 1299)",
        };

        const X: usize = 1080 / 2;
        const Y: usize = 1920 / 2;

        let x = X.to_string();
        let y = Y.to_string();

        let opts = BreakLinesOpts {
            x: x.as_str(),
            y: y.as_str(),
            font_family: "KyivType Serif",
            font_weight: 900,
            font_size: 80,
            line_height: 1.2,
            dominant_baseline: "middle",
            text_anchor: "middle",
            width: 900,
            align: TextAlign::Left,
            fill: "#000",
            ..Default::default()
        };

        svgr!(
            <path
               d="M126 -140.6C155.5 -125.3 166.4 -78.3 174.5 -31C182.6 16.3 187.8 63.9 170.3 102.2C152.8 140.4 112.5 169.4 68.8 182.4C25.1 195.5 -22 192.6 -46.2 166.6C-70.4 140.6 -71.7 91.4 -85.7 55.3C-99.6 19.2 -126.1 -3.9 -136.7 -38C-147.2 -72.1 -141.7 -117.2 -116.3 -133.3C-90.8 -149.3 -45.4 -136.1 1.4 -137.8C48.2 -139.5 96.4 -155.9 126 -140.6"
               fill={self.fill}
               id="bubble"
               transform={format!("{translate} scale({scale})")}
            />

            <clipPath id="containerClip">
               <use href="#bubble" />
            </clipPath>

            <g id="test" clip-path="url(#containerClip)">
                <image
                    y="10%"
                    x={1080 / 2 - 600 / 2}
                    width="600"
                    height="600"
                    href={ctx.get_image_link("avatar.png")}
                />

                <g transform={format!("translate({text_translate})")}>
                    <text
                       x="50%"
                       y="45%"
                       dominant-baseline="middle"
                       text-anchor="middle"
                       font-family="Noto Sans Medium"
                       font-size="60"
                       fill="#3f3f46"
                    >
                        {
                            match self.variant {
                                Variant::Title { .. } => "r/AskReddit",
                                Variant::Exit => "Like stories?",
                            }
                        }
                    </text>

                    {
                        match self.variant {
                            Variant::Title { .. } => {
                                frame.text_break_lines(ctx, self.title, &opts).unwrap_or_default()
                            },
                            Variant::Exit => svgr!(
                                <text
                                   x="50%"
                                   y="52%"
                                   dominant-baseline="middle"
                                   text-anchor="middle"
                                   font-family="KyivType Serif"
                                   font-weight="black"
                                   font-size="160"
                                >
                                    "Subscribe"
                                </text>
                            ),
                        }
                    }
                </g>

                {
                    match self.variant {
                        Variant::Exit => svgr!(
                            <g transform={format!("translate({})", text_translate - 1080.)}>
                                <text
                                   x="50%"
                                   y="45%"
                                   dominant-baseline="middle"
                                   text-anchor="middle"
                                   font-family="Noto Sans Medium"
                                   font-size="60"
                                   fill="#3f3f46"
                                >
                                    "r/AskReddit"
                                </text>

                                {frame.text_break_lines(ctx, self.title, &opts).unwrap_or_default()}
                            </g>
                        ),
                        _ => Svgr::default()
                    }
                }
            </g>
        )
    }
}
