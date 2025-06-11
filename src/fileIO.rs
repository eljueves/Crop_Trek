use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize)]
pub struct InputFrame {
    pub frame_id: usize,
    pub timestamp: String,
    pub detections: Vec<Detection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Detection {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize)]
pub struct OutputObject {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize)]
pub struct OutputFrame {
    pub frame_id: usize,
    pub timestamp: String,
    pub tracked_objects: Vec<OutputObject>,
}

pub fn parseInputFrameJSONFile(input_path: &String) ->  Vec<InputFrame> {
    let binding = fs::read_to_string(input_path);
    let data = binding.as_ref().unwrap().as_str();
    serde_json::from_str(&data).unwrap_or(Vec::new())
}

pub fn generateOutputJSONFrom(output_frames: Vec<OutputFrame>, output_path: &String) {
    let output_json = serde_json::to_string_pretty(&output_frames).unwrap_or(String::new());
    fs::write(output_path, output_json);
}