use crate::sim::ManhattanLayout;
use plotters::prelude::*;
use plotters_svg::SVGBackend;

// Plot the Manhattan layout including avenues, streets, base stations, and receiver (returns nothing)
pub fn plot_manhattan_layout(layout: &ManhattanLayout, size: f64, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(output_path, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let margin = size * 0.1;
    let x_min = -size / 2.0 - margin;
    let x_max = size / 2.0 + margin;
    let y_min = -size / 2.0 - margin;
    let y_max = size / 2.0 + margin;

    let mut chart = ChartBuilder::on(&root)
        .caption("Manhattan Network Layout", ("sans-serif", 30))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh()
        .x_desc("X (meters)")
        .y_desc("Y (meters)")
        .draw()?;

    // Draw avenues (vertical lines)
    for &ave in &layout.avenues {
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(ave, -size / 2.0), (ave, size / 2.0)],
            ShapeStyle::from(&BLUE).stroke_width(1),
        )))?;
    }

    // Draw streets (horizontal lines)
    for &st in &layout.streets {
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(-size / 2.0, st), (size / 2.0, st)],
            ShapeStyle::from(&GREEN).stroke_width(1),
        )))?;
    }

    // Draw base stations as red circles
    chart.draw_series(layout.base_stations.iter().map(|&p| {
        Circle::new((p.x, p.y), 4, ShapeStyle::from(&RED).filled())
    }))?;

    // Draw receiver at (0, 0) as a large black star/marker
    chart.draw_series(std::iter::once(
        Circle::new((0.0, 0.0), 6, ShapeStyle::from(&BLACK).filled())
    ))?;

    root.present()?;
    Ok(())
}
