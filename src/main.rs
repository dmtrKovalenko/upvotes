pub use fframes_renderer::{fframes_logger, render, render_backend, RenderOptions};
use hello_world_example::upvotes_video::UpvotesVideo;

fn main() {
    // debug_frame(
    //     317,
    //     UpvotesVideo {},
    //     "out.png",
    //     RenderOptions {
    //         media_dir: "./media",
    //         logger: fframes_logger::FFramesLoggerVariant::Compact,
    //         render_backend: render_backend::CpuRenderingBackend {
    //             cache_capacity: 0,
    //             ..Default::default()
    //         },
    //         preferred_codec: "libx264",
    //         ..Default::default()
    //     },
    // )
    // .unwrap();

    render(
        UpvotesVideo {},
        "out.mp4",
        RenderOptions {
            media_dir: "./media",
            logger: fframes_logger::FFramesLoggerVariant::Compact,
            render_backend: render_backend::CpuRenderingBackend {
                cache_capacity: 0,
                concurrency: 6,
                ..Default::default()
            },
            preferred_codec: "libx265",
            ..Default::default()
        },
    )
    .unwrap();
}
