use crate::arcfm::{fuel_rod_svg, temperature};
use crate::svg::render_svg;
use crate::{arcfm::fuel_rod_table, structs::MainStruct};
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Stdout},
};
use tui::widgets::GraphType::Line as OtherLine;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::{self, DOT},
    text::Spans,
    widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph, Tabs},
    Frame, Terminal,
};

pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    tui_command_text: Paragraph<'_>,
    block_2: Block<'_>,
    mainstruct: &mut MainStruct,
    log_text: Paragraph,
    graph: Chart,
) -> (Result<(), io::Error>, u16) {
    let mut chunks_2 = Vec::new();
    let mut chunks_3 = Vec::new();
    let draw = terminal.draw(|frame| {
        let terminal_rect = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(terminal_rect);
        // split block 2 into 2 columns
        chunks_2 = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);
        chunks_3 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        frame.render_widget(tui_command_text, chunks[0]);

        //println!("{}", chunks_2[0].height);

        let left_tabs = Tabs::new(vec![
            Spans::from("Core"),
            Spans::from("Rod"),
            Spans::from("Turbine"),
        ])
        .block(
            Block::default()
                .title("Graphical renderer")
                .borders(Borders::all())
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(DOT)
        .select(mainstruct.data.left_tab_index);
        frame.render_widget(left_tabs, chunks_3[0]);

        //frame.render_widget(reactor_core, chunks_3[0]);
        temperature(mainstruct, 5, 5);
        match mainstruct.data.left_tab_index {
            0 => fuel_rod_table(5, 5, chunks_3[0], frame, mainstruct),
            1 => fuel_rod_svg(mainstruct, frame, chunks_3[0]),
            2 => draw_turbine(mainstruct, frame, chunks_3[0]),
            _ => {}
        }

        frame.render_widget(graph, chunks_3[1]);
        frame.render_widget(block_2, chunks_2[0]);
        frame.render_widget(log_text, chunks_2[1]);
    });
    drop(draw);
    return (Ok(()), chunks_2[0].height);
}
pub fn draw_turbine(
    mainstruct: &mut MainStruct,
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    layout: Rect,
) {
    let width = layout.width as f64;
    let height = layout.height as f64;
    let ratio = width / height;
    /*
    let square_data = draw_rectangle(20.0, 20.0, ratio);

    let circle_data = draw_circle(20.0, ratio);

    let square = Dataset::default()
        .data(&square_data)
        .marker(symbols::Marker::Braille)
        .graph_type(OtherLine);
    let circle = Dataset::default()
        .data(&circle_data)
        .marker(symbols::Marker::Braille)
        .graph_type(OtherLine);
    let center = Dataset::default()
        .style(Style::default().fg(Color::Red))
        .data(&[(50.0_f64, 50.0_f64)])
        .marker(symbols::Marker::Braille)
        .graph_type(tui::widgets::GraphType::Scatter);
    */
    let mut hash_map: HashMap<usize, (Vec<(f64, f64)>, String, bool)> = HashMap::new();
    render_svg(
        "./resources/test.svg".to_string(),
        ratio,
        mainstruct,
        &mut hash_map,
    );
    //mainstruct.data.log.push(format!("SVG data: {:?}", svg_data));
    //let picture = Dataset::default().data(&svg_data).marker(symbols::Marker::Braille).graph_type(OtherLine).style(Style::default().fg(Color::Red));
    let mut datasets = Vec::new();

    for i in hash_map.values() {
        let re = Regex::new(r"stroke:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\);").unwrap();
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
    //datasets.push(square);
    //datasets.push(circle);
    //datasets.push(picture);
    //datasets.push(center);
    let turbine = Chart::new(datasets)
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
    frame.render_widget(turbine, layout);
}
fn draw_rectangle(width: f64, height: f64, ratio: f64) -> Vec<(f64, f64)> {
    let width = width / (ratio / 2.0);
    let pos_x = 100.0 / 2.0 - width / 2.0;
    let pos_y = 100.0 / 2.0 - height / 2.0;
    let mut points = Vec::new();
    points.push((pos_x, pos_y));
    points.push((pos_x + width, pos_y));
    points.push((pos_x + width, pos_y + height));
    points.push((pos_x, pos_y + height));
    points.push((pos_x, pos_y));

    return points;
}

fn draw_circle(radius: f64, ratio: f64) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let pos_x = 100.0 / 2.0;
    let pos_y = 100.0 / 2.0;
    for i in 0..360 {
        let x = pos_x + (radius * (i as f64).to_radians().cos()) / (ratio / 2.0);
        let y = pos_y + radius * (i as f64).to_radians().sin();
        points.push((x, y));
    }
    return points;
}
