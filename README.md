# Crop Tracking System – Agricultural Robotics Challenge

This project is a crop tracking system for agricultural robotics. It tracks objects across video frames and assigns persistent IDs, even if an object temporarily disappears. It also visualizes the tracking process.

---

## 🚀 Features

- Centroid-based object tracker with persistent IDs
- Handles temporary occlusion/disappearance (1–3 frames)
- Simple 2D visualization of bounding boxes and IDs
- Output saved as JSON + annotated frame images
- Fully Dockerized

---

## 📁 Project Structure
tracking-solution/
├── src/
│   ├── main.rs            # CLI + coordination
│   ├── tracker.rs         # Tracking logic
│   └── visualize.rs       # PNG visualization
├── Cargo.toml
├── Dockerfile
├── sample_input.json      # Example input data
├── tracking_output.json   # Output (generated after running)
└── visualization/         # Output visualizations (PNG)

---

## ⚙️ Dependencies

- Rust (`cargo`)
- Docker
- Crates:
  - `serde`, `serde_json` (for JSON I/O)
  - `plotters` (for visualization)
  - `yeslogic-fontconfig-sys` (native dependency for font rendering)

---

## 🐳 Docker Usage

### 1️⃣ Build the Docker Image

```bash
docker build -t tracking-solution .

docker run --rm -v $(pwd):/data tracking-solution \
  --input /data/sample_input.json \
  --output /data/tracking_output.json \
  --vis-dir /data/visualization


	•	--input: Path to input JSON file
	•	--output: Path to output JSON file
	•	--vis-dir: Directory for PNG visualizations

---

## 🧪 Sample Input Format

```json
[
  {
    "frame_id": 0,
    "timestamp": "2025-03-24T18:00:00.000000Z",
    "detections": [
      { "x": 0.4, "y": 0.4, "width": 0.05, "height": 0.05 }
    ]
  }
]
```

---

## 📤 Output Format

```json
[
  {
    "frame_id": 0,
    "timestamp": "2025-03-24T18:00:00.000000Z",
    "tracked_objects": [
      {
        "id": 0,
        "x": 0.4,
        "y": 0.4,
        "width": 0.05,
        "height": 0.05
      }
    ]
  }
]
```

---

## 🖼 Visual Output

Frame-by-frame PNGs are saved to the specified `--vis-dir`, with bounding boxes and IDs overlaid for visual inspection.

---

## 🔓 License

MIT License – Free to use, modify, and distribute.


