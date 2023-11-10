use anyhow::Result;
use plotters::prelude::*;

// create density plot
pub fn density_plot(data: &[f32]) -> Result<()> {
    // Define the dimensions of the plot
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Create a drawing backend
    let root = BitMapBackend::new("density_plot.png", (WIDTH, HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Density Plot", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..1.0, 0.0..10.0)?; // Adjust the range accordingly

    chart.configure_mesh().draw()?;

    // Calculate the density
    let min = *data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = *data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let range = max - min;
    let step = range / WIDTH as f32;

    let mut densities = vec![0; WIDTH as usize];

    for &value in data {
        let index = ((value - min) / step).floor() as usize;
        densities[index] = densities[index].saturating_add(1);
    }

    // Normalize the densities
    let max_density = *densities.iter().max().unwrap() as f32;
    for density in &mut densities {
        *density = (*density as f32 / max_density * 10.0).round() as i32; // Scale it to fit the Y axis
    }

    // Draw the densities
    chart.draw_series(densities.into_iter().enumerate().map(|(x, y)| {
        Rectangle::new(
            [
                (x as f32 * step + min, 0.0),
                (x as f32 * step + min + step, y as f32),
            ],
            RED.filled(),
        )
    }))?;

    // Make sure the data is rendered
    root.present()?;

    Ok(())
}

// Function to create a histogram
pub fn histogram(data: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const NUM_BINS: usize = 50; // Adjust the number of bins as needed

    // Create a drawing backend
    let root = BitMapBackend::new("histogram.png", (WIDTH, HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min, max) = data
        .iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), &val| {
            (min.min(val), max.max(val))
        });

    let bin_size = (max - min) / NUM_BINS as f32;
    let mut bins = vec![0; NUM_BINS];

    for &value in data {
        let bin = ((value - min) / bin_size).min(NUM_BINS as f32 - 1.0) as usize;
        bins[bin] += 1;
    }

    let max_count = *bins.iter().max().unwrap() as f32;

    // Create a chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min..max, 0.0..max_count)?;

    chart.configure_mesh().draw()?;

    // Plot the bins
    chart.draw_series(Histogram::vertical(&chart).style(RED.filled()).data(
        bins.into_iter().enumerate().map(|(i, count)| {
            let x0 = min + i as f32 * bin_size;
            (x0..x0 + bin_size, count as f32)
        }),
    ))?;

    // Make sure the data is rendered
    root.present()?;

    Ok(())
}
