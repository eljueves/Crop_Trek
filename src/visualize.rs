use plotters::prelude::*;
use crate::fileIO::{OutputObject};
use std::fs;

pub fn draw_frame(
    tracked_objects: &Vec<OutputObject>,
    frame_id: usize,
    vis_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(vis_dir)?;

    let filepath = format!("{}/frame_{:03}.png", vis_dir, frame_id);
    let root = BitMapBackend::new(&filepath, (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Frame {}", frame_id), ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(0f64..1f64, 1f64..0f64)?; // Flip Y-axis

    chart.configure_mesh().disable_mesh().draw()?;

    for det in tracked_objects {
        let (id, x, y, w, h) = (det.id, det.x, det.y, det.width, det.height);
        let rect = Rectangle::new(
            [(x - w / 2.0, y - h / 2.0), (x + w / 2.0, y + h / 2.0)],
            ShapeStyle::from(&BLUE).stroke_width(1),
        );
        chart.draw_series(std::iter::once(rect))?;

        chart.draw_series(std::iter::once(Text::new(
            format!("{}", id),
            (x, y),
            ("sans-serif", 12).into_font().color(&RED),
        )))?;
    }

    Ok(())
}
