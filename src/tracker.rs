use crate::file_io::{InputFrame, Detection, parse_input_frames_json_file};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrackedObject {
    pub id: usize,
    pub detection: Detection,
    pub last_seen: usize,
    
    //was plaining on using in a more advanced implementation
    pub disappeared: usize, 
    pub history: Vec<Detection>,
}

pub struct ObjectTracker {
    next_id: usize,
    distance_threshold: f64,
    objects: HashMap<usize, TrackedObject>,
}

/*
Choosing to go with a distance based approached to the tracker
Was thinking of incorparating time but I came to the conclusion that time would stay relatively constant
*/
impl ObjectTracker {
    pub fn new() -> Self {
        
        ObjectTracker {
            next_id: 0,
            distance_threshold: 0.03, // choosing 0.3 for distance between frames
            objects: HashMap::new(),
        }
    }

    fn distance(d1: &Detection, d2: &Detection) -> f64 {
        ((d1.x - d2.x).powi(2) + (d1.y - d2.y).powi(2)).sqrt()
    }

    pub fn get_all_tracked_objects(&self) -> Vec<TrackedObject> {
        self.objects
            .values()
            .map(|obj| (obj.clone()))
            .collect()
}

    pub fn get_all_tracked_objects_in_frame(&self, frame_id: usize) -> Vec<TrackedObject> {
        self.objects
            .values()
            .map(|obj| (obj.clone()))
            .filter(|obj| obj.last_seen == frame_id)
            .collect()
    }
    

    pub fn evaluate_frame(&mut self, frame: &InputFrame) {
        let mut matched_ids = vec![];
        let mut unmatched_detections: Vec<&Detection> = frame.detections.iter().collect();

        //calculates distance between detections and if a detection is within a certian distance it is considered the same obj
        for detection in &frame.detections {
            let mut best_match = None;
            let mut best_distance = f64::MAX;

            for (&id, obj) in &self.objects {
                if matched_ids.contains(&id) {
                    continue;
                }

                let dist = Self::distance(&detection, &obj.detection);
                if dist < best_distance && dist < self.distance_threshold {
                    best_distance = dist;
                    best_match = Some(id);
                }
            }

            
            if let Some(id) = best_match {
                let obj = self.objects.get_mut(&id).unwrap();
                obj.detection = detection.clone();
                obj.last_seen = frame.frame_id;
                obj.disappeared = 0;
                obj.history.push(detection.clone());
                matched_ids.push(id);
                unmatched_detections.retain(|&d| d != detection);
            }
        }

        for id in self.objects.keys().cloned().collect::<Vec<_>>() {
            if !matched_ids.contains(&id) {
                let obj = self.objects.get_mut(&id).unwrap();
                obj.disappeared += 1;
            }
        }

        for detection in unmatched_detections {
            let obj = TrackedObject {
                id: self.next_id,
                detection: detection.clone(),
                last_seen: frame.frame_id,
                disappeared: 0,
                history: vec![detection.clone()],
            };
            self.objects.insert(self.next_id, obj);
            matched_ids.push(self.next_id);
            self.next_id += 1;
        }
    }
}

#[cfg(test)]
#[allow(dead_code, unused_imports, clippy::all)]
mod tests {
    use super::*;

    #[test]
    fn test_one_object_is_deteched() {
        let mut tracker = ObjectTracker::new();
        let json_file = String::from("test_data/input_data_test_one_crop_detected.json");
        let input_frames: Vec<InputFrame> = parse_input_frames_json_file(&jsonFile);

        for frame in input_frames {
            tracker.evaluate_frame(&frame);
        }

        let tracked = tracker.get_all_tracked_objects();

        assert_eq!(tracked.len(), 1, "number of tracked items is wrong");
        assert_eq!(tracked[0].id, 0, "id did not match"); 
    }



    fn test_one_object_going_in_and_out_of_frame() {
        let mut tracker = ObjectTracker::new();
        let json_file = String::from("test_data/input_data_one_crop_in_out_frame.json");
        let input_frames: Vec<InputFrame> = parse_input_frames_json_file(&jsonFile);

        for frame in input_frames {
            tracker.evaluate_frame(&frame);
        }

        let tracked = tracker.get_all_tracked_objects();

        assert_eq!(tracked.len(), 1, "number of tracked items is wrong");
        assert_eq!(tracked[0].id, 0, "id did not match"); 
    }

    #[test]
    fn test_multiple_objects_are_detected() {
        let mut tracker = ObjectTracker::new();
        let json_file = String::from("test_data/input_data_multiple_crops.json");
        let input_frames: Vec<InputFrame> = parse_input_frames_json_file(&jsonFile);

        for frame in input_frames {
            tracker.evaluate_frame(&frame);
        }

        let tracked = tracker.get_all_tracked_objects();

        assert_eq!(tracked.len(), 3, "number of tracked items is wrong");
    }
}
