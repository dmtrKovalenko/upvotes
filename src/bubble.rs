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
    pub upvotes_count: &'a str,
    pub author: &'a str,
    pub when: &'a str,
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

        const X: usize = 90;
        const Y: usize = 400;

        let x = X.to_string();
        let y = Y.to_string();

        let opts = BreakLinesOpts {
            x: x.as_str(),
            y: y.as_str(),
            font_family: "KyivType Serif",
            font_weight: 900,
            font_size: 80,
            line_height: 1.2,
            width: 900,
            align: TextAlign::Left,
            fill: "#000",
            ..Default::default()
        };

        let subtitle = format!("Posted by {} {}", self.author, self.when);
        let upvotes = svgr!(
            <image
                href={ctx.get_image_link("upvote_arrow.png")}
                x={X}
                y={Y + 620}
                width="60"
                height="60"
            />

            <text x={X + 70} y={Y + 630} fill="black" font-size="50" font-family="Nunito" font-weight="bold" dominant-baseline="hanging">
                {self.upvotes_count}" upvotes"
            </text>
        );

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
                    y={940}
                    x={0}
                    width={1080}
                    href={ctx.get_image_link("avatar.png")}
                />

                <g transform={format!("translate({text_translate})")}>
                    <text
                       x={X}
                       y={Y - 110}
                       font-family="Nunito"
                       font-weight="bold"
                       font-size="50"
                       fill="#3f3f46"
                    >
                        {
                            match self.variant {
                                Variant::Title { .. } => subtitle.as_str(),
                                Variant::Exit => "Like stories?",
                            }
                        }
                    </text>

                    {
                        match self.variant {
                            Variant::Title { .. } => {
                                svgr!(
                                    {frame.text_break_lines(ctx, self.title, &opts).unwrap_or_default()}
                                    {upvotes.clone()}
                                )
                            },
                            Variant::Exit => svgr!(
                                <text
                                   x={X}
                                   y={Y + 50}
                                   font-family={opts.font_family}
                                   font-weight="black"
                                   font-size="140"
                                >
                                    "Subscribe"
                                    <tspan x={X} y={Y + 50} dy="1em">
                                     "for more"
                                    </tspan>
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
                                   id="subtitle"
                                   x={X}
                                   y={Y - 110}
                                   font-family="Nunito"
                                   font-weight="bold"
                                   font-size="50"
                                   fill="#3f3f46"
                                >
                                    {subtitle}
                                </text>

                                {frame.text_break_lines(ctx, self.title, &opts).unwrap_or_default()}
                                {upvotes}
                            </g>
                        ),
                        _ => Svgr::default()
                    }
                }
            </g>
        )
    }
}
