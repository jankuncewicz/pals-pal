use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

/// Draw power function f(x) = x^power.
pub fn draw(canvas_id: &str, data_points: Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let max_x = data_points.iter().reduce(|a, i| if a.0 >= i.0 {a} else {i}).unwrap();
    let min_x = data_points.iter().reduce(|a, i| if a.0 <= i.0 {a} else {i}).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .set_all_label_area_size(50u32)
        .build_cartesian_2d((min_x.0..max_x.0).log_scale(), min_x.1..max_x.1)?;

    chart.configure_mesh().x_labels(5).y_labels(10)
        .x_desc("R(nm)").y_desc("tau(ns)")
        //.axis_desc_style(("sans-serif", 20))
        //.label_style(("sans-serif", 20))
        .draw()?;

    chart.draw_series(data_points.iter().map(|point| Circle::new(*point, 2, RED.filled()))).unwrap();
    //chart.draw_series(LineSeries::new(data_points, &RED))?;
    root.present()?;
    return Ok(());
}
