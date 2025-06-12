mod tracker;
mod visualize;
mod interactive_map;
mod file_io;

use tracker::ObjectTracker;
use visualize::draw_frame;
use interactive_map::run_interactive_map_view;
use file_io::{InputFrame, OutputFrame, OutputObject, parse_input_frames_json_file, generate_output_json_from};

use std::{error::Error};
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

    let input_frames: Vec<InputFrame> = parse_input_frames_json_file(input_path);
    let mut output_frames = Vec::new();
    let all_tracked_objects = Vec::new();

    
    let mut tracker = ObjectTracker::new();
    
    for frame in input_frames {
        tracker.evaluate_frame(&frame);

        let tracked_objects: Vec<OutputObject> = tracker.get_all_tracked_objects_in_frame(frame.frame_id)
            .into_iter()
            .map(|obj| OutputObject {
                id: obj.id,
                x: obj.detection.x,
                y: obj.detection.y,
                width: obj.detection.width,
                height: obj.detection.height,
            })
            .collect();

        draw_frame(&tracked_objects, frame.frame_id, vis_dir)?;

        output_frames.push(OutputFrame {
            frame_id: frame.frame_id,
            timestamp: frame.timestamp,
            tracked_objects: tracked_objects,
        });
    }

    generate_output_json_from(output_frames, output_path);

    //Run interactive map
    if args.interactive {
        run_interactive_map_view(&all_tracked_objects);
    }

    Ok(())
}
