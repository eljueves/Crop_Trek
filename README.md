# Crop Tracking System – Agricultural Robotics Challenge

This project is a crop tracking system for agricultural robotics. It tracks objects across video frames and assigns persistent IDs, even if an object temporarily disappears. It also visualizes the tracking process.

The reason it's called Crop-trek was that I was hoping it could evolve in a interactive testing environment for detecting crops.

---

## 🚀 Features

- Centroid-based object tracker with persistent IDs
- Handles temporary occlusion/disappearance
- Simple 2D visualization of bounding boxes and IDs
- Output saved as JSON + annotated frame images
- Fully Dockerized

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

docker run -v $(pwd):/data tracking-solution --input /data/input_data.json --output /data/tracking_output.json --vis-dir /data/visualization


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

---

## WIP 
-(WIP) was planning on adding an interactive GUI to simulate a camera panning around a field. Hence the src/interactive_map.rs file
you can activate that feature by doing the following. it only works on a Linux machine with a display.

```bash
xhost +local:root  # Allow local Docker to access X server
docker run --rm -e DISPLAY=$DISPLAY -v /tmp/.X11-unix:/tmp/.X11-unix -v $(pwd):/data tracking-solution --input /data/input_data.json --output /data/tracking_output.json --vis-dir /data/visualization --interactive

```


---

## 🔓 License

MIT License – Free to use, modify, and distribute.


