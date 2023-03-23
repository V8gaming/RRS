use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;
use crate::structs::MainStruct;

pub fn fuel_rod_table(
    width: i32,
    height: i32,
    layout: Rect,
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    mainstruct: &mut MainStruct,
) {
    let column_constraints = std::iter::repeat(Constraint::Percentage((100 / width) as u16))
        .take((width) as usize)
        .collect::<Vec<_>>();
    let row_constraints = std::iter::repeat(Constraint::Percentage((100 / height) as u16))
        .take((height) as usize)
        .collect::<Vec<_>>();

    let row_rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .margin(2)
        .split(layout);

    let color_gradient = colorgrad::CustomGradient::new()
        .html_colors(&[
            "rgba(0,0,0,1)",
            "rgba(255,192,0,1)",
            "rgba(255,126,0,1)",
            "rgba(255,67,0,1)",
            "rgba(255,0,0,1)",
        ])
        .domain(&[0.0, 100.0])
        .build()
        .unwrap();

    for i in 0..height {
        let column_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(column_constraints.clone())
            .split(row_rects[i as usize]);
        for j in 0..width {
            if i > 0 {
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.1[0] = true;
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.0[0] =
                    ((i - 1) as u16, j as u16);
            }
            if i < height - 1 {
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.1[1] = true;
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.0[1] =
                    ((i + 1) as u16, j as u16);
            }

            if j > 0 {
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.1[2] = true;
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.0[2] =
                    (i as u16, (j - 1) as u16);
            }
            if j < width - 1 {
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.1[3] = true;
                mainstruct.absorber_rods[i as usize][j as usize].neighbors.0[3] =
                    (i as u16, (j + 1) as u16);
            }
            let MIN = 0.0;
            let MAX = 100.0;
            // sum of neighbor temperatures times by 0.05
            let mut neighbor_temp_sum = 0.0;
            //mainstruct.data.log.push(format!("{}, {}",i+1,j+1));
            let mut adjusted_height = height;
            let mut adjusted_width = width;
            if height % 2 != 0 {
                adjusted_height = height + 1;
            }
            if width % 2 != 0 {
                adjusted_width = width + 1;
            }
            let center = (adjusted_height / 2, adjusted_width / 2);
            //mainstruct.data.log.push(format!("{}, {}",center.0,center.1));
            for k in 0..mainstruct.absorber_rods[i as usize][j as usize]
                .neighbors
                .1
                .len()
            {
                if mainstruct.absorber_rods[i as usize][j as usize].neighbors.1[k] {
                    let pos = mainstruct.absorber_rods[i as usize][j as usize].neighbors.0[0];
                    let abs_rod_pos = mainstruct.absorber_rods[pos.0 as usize][pos.1 as usize]
                        .absorber_rod_position;
                    let fuel_temp =
                        ((MAX - MIN) * (1.0 - abs_rod_pos as f64 / 100.0) + MIN).round();
                    // for every row, column closer to the center, the temperature is 5% higher
                    //let distance = ((pos.0 - center.0 as u16).pow(2) as f64 + (pos.1 - center.1 as u16).pow(2) as f64).sqrt();
                    let distance: f64 = ((i as f64 + 1.0 - center.0 as f64).abs().powf(2.0)
                        + (j as f64 + 1.0 - center.1 as f64).abs().powf(2.0))
                    .sqrt();
                    //mainstruct.data.log.push(format!("{:?}", distance));
                    neighbor_temp_sum += fuel_temp * -0.05 * distance;

                    //neighbor_temp_sum += fuel_temp;
                }
            }

            //mainstruct.data.log.push(format!("{:?}, {:?}", mainstruct.absorber_rods[0][0].neighbors, mainstruct.absorber_rods[1][1].neighbors));

            let temperature = ((MAX - MIN)
                * (1.0
                    - mainstruct.absorber_rods[i as usize][j as usize].fuel_temperature as f64
                        / 600.0)
                + MIN
                + (neighbor_temp_sum * 0.05))
                .round();
            let rgba = color_gradient.at(100.0 - temperature).to_rgba8();
            let temperature_color;
            if temperature == 100.0 {
                temperature_color = Color::Reset;
            } else {
                temperature_color = Color::Rgb(rgba[0], rgba[1], rgba[2]);
            }

            //mainstruct.data.log.push(format!("Temprature: {}, rgba: {:?}", 100.0-temperature, rgba));

            //let text = Text::from(format!("{}:{:.1}%", i*width+j+1,mainstruct.absorber_rods[i as usize][j as usize].absorber_rod_position));
            let text = Text::from(format!(
                "{}:{:.1}°C",
                i * width + j + 1,
                mainstruct.absorber_rods[i as usize][j as usize].fuel_temperature
                    + (neighbor_temp_sum as f32 * 0.05)
            ));

            let cell_text = Paragraph::new(text).block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default().bg(temperature_color)),
            );
            frame.render_widget(cell_text, column_rects[j as usize]);
        }
    }
}