use crate::sim::ManhattanLayout;
use plotters::prelude::*;
use plotters_svg::SVGBackend;

// Plot the Manhattan layout with optional zoom functionality
pub fn plot_manhattan_layout_with_zoom(
    layout: &ManhattanLayout, 
    size: f64, 
    output_path: &str,
    center_x: Option<f64>,
    center_y: Option<f64>,
    zoom_size: Option<f64>
) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(output_path, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Calculate bounds based on zoom parameters
    let (x_min, x_max, y_min, y_max) = if let Some(zs) = zoom_size {
        let cx = center_x.unwrap_or(0.0);
        let cy = center_y.unwrap_or(0.0);
        (cx - zs / 2.0, cx + zs / 2.0, cy - zs / 2.0, cy + zs / 2.0)
    } else {
        (-size / 2.0, size / 2.0, -size / 2.0, size / 2.0)
    };

    // Calculate appropriate marker sizes based on zoom level
    let view_size = x_max - x_min;
    let base_station_size = (4.0 * 5000.0 / view_size).max(2.0).min(8.0) as i32;
    let receiver_size = (6.0 * 5000.0 / view_size).max(3.0).min(10.0) as i32;
    let line_width = (1.0 * 5000.0 / view_size).max(0.5).min(3.0) as u32;

    let mut chart = ChartBuilder::on(&root)
        .caption("Manhattan Network Layout", ("sans-serif", 30))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh()
        .x_desc("X (meters)")
        .y_desc("Y (meters)")
        .light_line_style(&RGBColor(240, 240, 240))
        .bold_line_style(&RGBColor(220, 220, 220))
        .draw()?;

    // Draw avenues (vertical lines) - only if they're visible in the zoom region
    for &ave in &layout.avenues {
        if ave >= x_min && ave <= x_max {
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(ave, y_min), (ave, y_max)],
                ShapeStyle::from(&BLACK).stroke_width(line_width),
            )))?;
        }
    }

    // Draw streets (horizontal lines) - only if they're visible in the zoom region
    for &st in &layout.streets {
        if st >= y_min && st <= y_max {
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x_min, st), (x_max, st)],
                ShapeStyle::from(&BLACK).stroke_width(line_width),
            )))?;
        }
    }

    // Draw base stations as red circles - only if they're visible in the zoom region
    chart.draw_series(
        layout.base_stations.iter()
            .filter(|p| p.x >= x_min && p.x <= x_max && p.y >= y_min && p.y <= y_max)
            .map(|&p| Circle::new((p.x, p.y), base_station_size, ShapeStyle::from(&RED).filled()))
    )?;

    // Draw receiver at (0, 0) if it's visible in the zoom region
    if 0.0 >= x_min && 0.0 <= x_max && 0.0 >= y_min && 0.0 <= y_max {
        chart.draw_series(std::iter::once(
            Circle::new((0.0, 0.0), receiver_size, ShapeStyle::from(&BLACK).filled())
        ))?;
    }

    root.present()?;
    Ok(())
}
