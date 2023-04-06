use std::collections::HashMap;
use std::io::Stdout;

use crate::structs::MainStruct;
use crate::svg::render_svg;
use lazy_static::lazy_static;
use regex::Regex;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Spans, Text};
use tui::widgets::GraphType::Line as OtherLine;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph};
use tui::{symbols, Frame};

lazy_static! {
    static ref COLOR_GRADIENT: colorgrad::Gradient = colorgrad::CustomGradient::new()
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
}
pub type SvgPoints = (Vec<(f64, f64)>, String, bool);

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
            let neighbor_temp_sum = neighbor_temp_sum_fn(height, width, mainstruct, i, j);

            //mainstruct.data.log.push(format!("{:?}, {:?}", mainstruct.absorber_rods[0][0].neighbors, mainstruct.absorber_rods[1][1].neighbors));

            //mainstruct.data.log.push(format!("Temprature: {}, rgba: {:?}", 100.0-temperature, rgba));

            //let text = Text::from(format!("{}:{:.1}%", i*width+j+1,mainstruct.absorber_rods[i as usize][j as usize].absorber_rod_position));
            let text = Text::from(format!(
                "{}:{:.1}°C",
                i * width + j + 1,
                mainstruct.absorber_rods[i as usize][j as usize].fuel_temperature
                    + (neighbor_temp_sum as f32 * 0.05)
            ));

            let cell_text = Paragraph::new(text).block(
                Block::default().borders(Borders::NONE).style(
                    Style::default()
                        .bg(mainstruct.absorber_rods[i as usize][j as usize].temperature_color),
                ),
            );
            frame.render_widget(cell_text, column_rects[j as usize]);
        }
    }
}

fn neighbor_temp_sum_fn(
    height: i32,
    width: i32,
    mainstruct: &mut MainStruct,
    i: i32,
    j: i32,
) -> f64 {
    const MIN: f64 = 0.0;
    const MAX: f64 = 100.0;
    // sum of neighbor temperatures times by 0.05
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
    let absorber_rod = &mainstruct.absorber_rods[i as usize][j as usize];
    let neighbor_temp_sum = absorber_rod
        .neighbors
        .1
        .iter()
        .enumerate()
        .filter_map(|(k, active)| {
            if *active {
                let pos = absorber_rod.neighbors.0[k];
                let abs_rod_pos =
                    mainstruct.absorber_rods[pos.0 as usize][pos.1 as usize].absorber_rod_position;
                let fuel_temp = ((MAX - MIN) * (1.0 - abs_rod_pos as f64 / 100.0) + MIN).round();
                let distance: f64 = ((i as f64 + 1.0 - center.0 as f64).abs().powf(2.0)
                    + (j as f64 + 1.0 - center.1 as f64).abs().powf(2.0))
                .sqrt();
                Some(fuel_temp * -0.05 * distance)
            } else {
                None
            }
        })
        .sum();
    neighbor_temp_sum
}

pub fn fuel_rod_svg(
    mainstruct: &mut MainStruct,
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    layout: Rect,
) {
    let width = layout.width as f64;
    let height = layout.height as f64;
    let ratio = width / height;
    /*
    <?xml version="1.0" encoding="utf-8"?>
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
        <path d="M 50.000 0.000 L 50.000 100.000" style="stroke: rgb(0, 0, 0); stroke-width: 1; fill: none;" />
        <path d="M 50.000 0.000 L 50.000 {}.000" style="stroke: rgb(0, 0, 0); stroke-width: 1; fill: none;" />
    </svg>
     */
    let header_1 = r#"<?xml version="1.0" encoding="utf-8"?>"#;
    let header_2 = r#"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">"#;
    let footer = r#"</svg>"#;
    let selected_fuel_rod = mainstruct.core.selected_rod;

    let mut pos = (0, 0);
    for i in 0..mainstruct.absorber_rods.len() {
        for j in 0..mainstruct.absorber_rods[i].len() {
            if i * 5 + j == selected_fuel_rod {
                pos = (i, j);
            }
        }
    }
    mainstruct
        .data
        .log
        .push(format!("{:?}", (pos.0 + 1, pos.1 + 1)));

    let abs_rod_pos = mainstruct.absorber_rods[pos.0][pos.0].absorber_rod_position / 2.0 + 15.0;
    let absorber_rod = format!(
        r#"<path d="M 50.000 10.000 L 50.000 {abs_rod_pos}.000" style="stroke: rgb(0, 0, 0); stroke-width: 1; fill: none;" />"#
    );
    let fuel_rod_container = r#"<rect x="37" y="25" width="25" height="60" style="fill: rgb(255, 0, 0); stroke-width: 3; stroke: rgb(0,0,0);" />"#;
    let mut fuel_rod_svg = String::new();
    fuel_rod_svg.push_str(header_1);
    fuel_rod_svg.push_str(header_2);
    fuel_rod_svg.push_str(fuel_rod_container);
    fuel_rod_svg.push_str(absorber_rod.as_str());
    fuel_rod_svg.push_str(footer);
    //save svg to file

    let mut hash_map: HashMap<usize, SvgPoints> = HashMap::new();
    render_svg(fuel_rod_svg, ratio, mainstruct, &mut hash_map);
    let mut datasets = Vec::new();
    let re = Regex::new(r"stroke:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();
    let bg_re = Regex::new(r"fill:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();
    for i in hash_map.values() {
        if i.2 {
            let bg_color = Color::Rgb(
                bg_re
                    .captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                bg_re
                    .captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                bg_re
                    .captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
            );
            let dataset = Dataset::default()
                .data(&i.0)
                .marker(symbols::Marker::Blocks(symbols::Blocks::FULL))
                .graph_type(OtherLine)
                .style(Style::default().fg(bg_color));
            datasets.push(dataset);
        } else {
            let color = Color::Rgb(
                re.captures(&i.1)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                re.captures(&i.1)
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                re.captures(&i.1)
                    .unwrap()
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
            );
            let dataset = Dataset::default()
                .data(&i.0)
                .marker(symbols::Marker::Braille)
                .graph_type(OtherLine)
                .style(Style::default().fg(color));
            datasets.push(dataset);
        }
    }
    let fuel_rod = Chart::new(datasets)
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0]),
        );
    frame.render_widget(fuel_rod, layout);
    let fuel_rod = Paragraph::new(vec![
        Spans::from(format!("Fuel rod: {}", selected_fuel_rod + 1)),
        Spans::from(format!(
            "Fuel temp: {:.1}°C",
            mainstruct.absorber_rods[pos.0][pos.1].fuel_temperature
        )),
        Spans::from(format!(
            "C-Rod pos: {:.1}%",
            mainstruct.absorber_rods[pos.0][pos.1].absorber_rod_position
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Data")
            .style(Style::default().bg(mainstruct.absorber_rods[pos.0][pos.1].temperature_color)),
    );
    // render text in the top right corner
    let text_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(28),
                Constraint::Percentage(2),
            ]
            .as_ref(),
        )
        .split(layout);
    let vert_alignment = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(88),
                Constraint::Percentage(2),
            ]
            .as_ref(),
        )
        .split(text_chunk[1]);

    frame.render_widget(fuel_rod, vert_alignment[1]);
}

pub fn temperature(mainstruct: &mut MainStruct, width: i32, height: i32) {
    const MIN: f64 = 0.0;
    const MAX: f64 = 100.0;

    for i in 0..mainstruct.absorber_rods.len() {
        for j in 0..mainstruct.absorber_rods[0].len() {
            let temperature = ((MAX - MIN)
                * (1.0
                    - mainstruct.absorber_rods[i][j].fuel_temperature as f64
                        / 600.0)
                + MIN
                + (neighbor_temp_sum_fn(height, width, mainstruct, i as i32, j as i32) * 0.05))
                .round();
            let rgba = COLOR_GRADIENT.at(100.0 - temperature).to_rgba8();
            let temperature_color;
            if temperature == 100.0 {
                temperature_color = Color::Reset;
                mainstruct.absorber_rods[i][j].temperature_color =
                    temperature_color;
            } else {
                temperature_color = Color::Rgb(rgba[0], rgba[1], rgba[2]);
                mainstruct.absorber_rods[i][j].temperature_color =
                    temperature_color;
            }
        }
    }
}
