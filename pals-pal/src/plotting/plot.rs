use std::cmp::max_by;

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

/// Draw power function f(x) = x^power.
pub fn draw(canvas_id: &str, data_points: Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let max_x = data_points.iter().reduce(|a, i| if a.0 >= i.0 {a} else {i}).unwrap();
    let min_x = data_points.iter().reduce(|a, i| if a.0 <= i.0 {a} else {i}).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d((min_x.0..max_x.0).log_scale(), min_x.1..max_x.1)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(data_points, &RED))?;

    root.present()?;
    return Ok(());
}
