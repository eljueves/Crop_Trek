mod tracker;
mod visualize;
mod interactive_map;
mod fileIO;

use tracker::ObjectTracker;
use visualize::draw_frame;
use interactive_map::run_interactive_map_view;
use fileIO::{InputFrame, OutputFrame, OutputObject, parseInputFrameJSONFile, generateOutputJSONFrom};

use std::{env, error::Error};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    input: String,
    #[arg(long)]
    output: String,
    #[arg(long)]
    vis_dir: String,
    #[arg(long, default_value_t = false)]
    interactive: bool,
}



fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input_path = &args.input;
    let output_path = &args.output;
    let vis_dir = &args.vis_dir;

    let input_frames: Vec<InputFrame> = parseInputFrameJSONFile(input_path);

    let mut tracker = ObjectTracker::new(3, 0.05);
    let mut output_frames = Vec::new();
    let mut all_tracked_objects = Vec::new();

    for frame in input_frames {
        let tracked = tracker.update(&frame.detections, frame.frame_id);
        let mut tracked_objects: Vec<OutputObject> = tracked
            .into_iter()
            .map(|(id, det)| OutputObject {
                id: id,
                x: det.x,
                y: det.y,
                width: det.width,
                height: det.height,
            })
            .collect();

        all_tracked_objects.append(&mut tracked_objects);

        draw_frame(&tracked_objects, frame.frame_id, vis_dir)?;

        output_frames.push(OutputFrame {
            frame_id: frame.frame_id,
            timestamp: frame.timestamp,
            tracked_objects: tracked_objects,
        });
    }

    generateOutputJSONFrom(output_frames, output_path);

    //Run interactive map
    if args.interactive {
        run_interactive_map_view(&all_tracked_objects);
    }

    Ok(())
}
