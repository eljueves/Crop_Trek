mod tracker;
mod visualize;

use tracker::{Detection, ObjectTracker};
use visualize::draw_frame;

use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs};

#[derive(Debug, Deserialize)]
struct InputFrame {
    frame_id: usize,
    timestamp: String,
    detections: Vec<Detection>,
}

#[derive(Debug, Serialize)]
struct OutputObject {
    id: usize,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Debug, Serialize)]
struct OutputFrame {
    frame_id: usize,
    timestamp: String,
    tracked_objects: Vec<OutputObject>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 7 || args[1] != "--input" || args[3] != "--output" || args[5] != "--vis-dir" {
        eprintln!(
            "Usage: {} --input <input.json> --output <output.json> --vis-dir <dir>",
            args[0]
        );
        std::process::exit(1);
    }

    let input_path = &args[2];
    let output_path = &args[4];
    let vis_dir = &args[6];

    let input_data = fs::read_to_string(input_path)?;
    let input_frames: Vec<InputFrame> = serde_json::from_str(&input_data)?;

    let mut tracker = ObjectTracker::new(3, 0.05);
    let mut output_frames = Vec::new();

    for frame in input_frames {
        let tracked = tracker.update(&frame.detections, frame.frame_id);
        let tracked_objects: Vec<OutputObject> = tracked
            .into_iter()
            .map(|(id, det)| OutputObject {
                id,
                x: det.x,
                y: det.y,
                width: det.width,
                height: det.height,
            })
            .collect();

        draw_frame(&tracked_objects.iter().map(|o| (o.id, Detection {
            x: o.x,
            y: o.y,
            width: o.width,
            height: o.height,
        })).collect::<Vec<_>>(), frame.frame_id, vis_dir)?;

        output_frames.push(OutputFrame {
            frame_id: frame.frame_id,
            timestamp: frame.timestamp,
            tracked_objects,
        });
    }

    let output_json = serde_json::to_string_pretty(&output_frames)?;
    fs::write(output_path, output_json)?;

    Ok(())
}
