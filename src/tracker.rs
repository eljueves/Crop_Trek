use crate::fileIO::Detection;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrackedObject {
    pub id: usize,
    pub detection: Detection,
    pub last_seen: usize,
    pub disappeared: usize,
    pub history: Vec<Detection>,
}

pub struct ObjectTracker {
    next_id: usize,
    max_disappeared: usize,
    distance_threshold: f64,
    objects: HashMap<usize, TrackedObject>,
}

impl ObjectTracker {
    pub fn new(max_disappeared: usize, distance_threshold: f64) -> Self {
        ObjectTracker {
            next_id: 0,
            max_disappeared,
            distance_threshold,
            objects: HashMap::new(),
        }
    }

    fn distance(d1: &Detection, d2: &Detection) -> f64 {
        ((d1.x - d2.x).powi(2) + (d1.y - d2.y).powi(2)).sqrt()
    }

    pub fn update(&mut self, detections: &[Detection], frame_id: usize) -> Vec<(usize, Detection)> {
        let mut matched_ids = vec![];
        let mut unmatched_detections: Vec<&Detection> = detections.iter().collect();

        for detection in detections {
            let mut best_match = None;
            let mut best_distance = f64::MAX;

            for (&id, obj) in &self.objects {
                if matched_ids.contains(&id) {
                    continue;
                }

                let dist = Self::distance(detection, &obj.detection);
                if dist < best_distance && dist < self.distance_threshold {
                    best_distance = dist;
                    best_match = Some(id);
                }
            }

            if let Some(id) = best_match {
                let obj = self.objects.get_mut(&id).unwrap();
                obj.detection = detection.clone();
                obj.last_seen = frame_id;
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
                last_seen: frame_id,
                disappeared: 0,
                history: vec![detection.clone()],
            };
            self.objects.insert(self.next_id, obj);
            matched_ids.push(self.next_id);
            self.next_id += 1;
        }

        // Remove disappeared objects
        self.objects.retain(|_, obj| obj.disappeared <= self.max_disappeared);

        // Output tracked objects
        self.objects
            .values()
            .map(|obj| (obj.id, obj.detection.clone()))
            .collect()
    }
}
