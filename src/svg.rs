use std::{collections::HashMap, fs, f64::consts::PI};

use regex::Regex;

use crate::structs::MainStruct;

pub fn render_svg(
    svg: String,
    ratio: f64,
    mainstruct: &mut MainStruct,
    hash_map: &mut HashMap<usize, (Vec<(f64, f64)>, String)>,
) {
    let mut buf = String::new();
    let mut f = fs::read_to_string(svg).unwrap();
    let mut view_box = Vec::new();
    // read the whole file
    let parser = xml::reader::EventReader::from_str(f.as_str());
    let mut objects = Vec::new();
    for event in parser {
        match event.unwrap() {
            xml::reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                if name.local_name == "svg" {
                    let viewbox = &attributes[0].value;
                    let viewbox_data = viewbox.split(" ").collect::<Vec<&str>>();
                    let width = viewbox_data[2].parse::<f64>().unwrap();
                    let height = viewbox_data[3].parse::<f64>().unwrap();
                    view_box.push(width);
                    view_box.push(height);
                }
                if name.local_name == "path" {
                    let mut variables = ("".to_string(), "".to_string(), "".to_string());
                    for i in &attributes {
                        match i.name.local_name.as_str() {
                            "d" => {
                                variables.0 = i.value.to_owned();
                            }
                            "style" => {
                                variables.1 = i.value.to_owned();
                            }
                            "transform" => {
                                variables.2 = i.value.to_owned();
                            }

                            _ => {}
                        }
                    }
                    objects.push(variables);
                }
            }
            _ => {}
        }
    }

    for object in objects.iter().enumerate() {
        let points = draw_path(
            object.1 .0.to_owned(),
            view_box.clone(),
            mainstruct,
            ratio,
            &object.1 .2,
        );
        //mainstruct.data.log.push(format!("Points: {:?}", points));
        let style = object.1 .1.to_owned();
        hash_map.insert(object.0, (points, style));
    }
}
fn draw_path(
    strings: String,
    view_box: Vec<f64>,
    mainstruct: &mut MainStruct,
    ratio: f64,
    transform_str: &str,
) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let x_scale = view_box[0] / 100.0;
    let y_scale = view_box[1] / 100.0;

    let re = Regex::new(r"[A-Z]\s*((?:\d+\.\d+\s+)*\d+\.\d+)*").unwrap();

    let mut start = (0.0, 0.0);
    let mut prev_point = (0.0, 0.0);
    let mut prev_command = "";
    let mut prev_match = "".to_string();

    for i in split_keep(&re, &strings) {
        //mainstruct.data.log.push(format!("i: {:?}", i));
        if i == " " {
            continue;
        }
        if i == "Z" {
            points.push(start);
            continue;
        } else {
            let data = i.split(" ").collect::<Vec<&str>>();
            //println!("{:?}", data);
            let command = data[0];

            match command {
                "M" => {
                    let x = data[1].parse::<f64>().unwrap();
                    let y = data[2].parse::<f64>().unwrap();
                    let mut x = x / x_scale;
                    let mut y = y / y_scale;
                    if Some(transform_str) == None {
                        continue;
                    } else {
                        let transformed_points = transform(x, y, transform_str, (x_scale, y_scale));
                        x = transformed_points.0;
                        y = transformed_points.1;
                    }
                    start = (x, y);
                    points.push(start);
                    prev_point = start;
                    prev_command = command;
                    prev_match = format!("{}, {}, {}", command, x, y);
                }
                "L" => {
                    let x = data[1].parse::<f64>().unwrap();
                    let y = data[2].parse::<f64>().unwrap();
                    let mut x = x / x_scale;
                    let mut y = y / y_scale;
                    if Some(transform_str) == None {
                        continue;
                    } else {
                        let transformed_points = transform(x, y, transform_str, (x_scale, y_scale));
                        x = transformed_points.0;
                        y = transformed_points.1;
                    }
                    points.push((x, y));
                    prev_point = (x, y);
                    prev_command = command;
                    prev_match = format!("{}, {}, {}", command, x, y);
                }
                "Q" => {
                    // Quadratic Bezier Curve

                    let control_point_x = data[1].parse::<f64>().unwrap() / x_scale;
                    let control_point_y = 100.0 - data[2].parse::<f64>().unwrap() / y_scale;
                    let end_point_x = data[3].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = 100.0 - data[4].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = quadratic_bezier_curve(
                            &prev_point,
                            &(control_point_x, control_point_y),
                            &(end_point_x, end_point_y),
                            t,
                            ratio,
                            Some(transform_str.clone()),
                            (x_scale, y_scale)
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y);
                    prev_command = command;
                    prev_match = format!(
                        "{}, {}, {}, {}, {}",
                        command, control_point_x, control_point_y, end_point_x, end_point_y
                    );
                }
                "C" => {
                    // Cubic Bezier Curve
                    let control_point_1_x = data[1].parse::<f64>().unwrap() / x_scale;
                    let control_point_1_y = data[2].parse::<f64>().unwrap() / y_scale;
                    let control_point_2_x = data[3].parse::<f64>().unwrap() / x_scale;
                    let control_point_2_y = data[4].parse::<f64>().unwrap() / y_scale;
                    let end_point_x = data[5].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = data[6].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = cubic_bezier_curve(
                            &prev_point,
                            &(control_point_1_x, control_point_1_y),
                            &(control_point_2_x, control_point_2_y),
                            &(end_point_x, end_point_y),
                            t,
                            ratio,
                            Some(transform_str.clone()),
                            (x_scale, y_scale)
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y);
                    prev_command = command;
                    prev_match = format!(
                        "{}, {}, {}, {}, {}, {}, {}",
                        command,
                        control_point_1_x,
                        control_point_1_y,
                        control_point_2_x,
                        control_point_2_y,
                        end_point_x,
                        end_point_y
                    );
                }
                "S" => {
                    let mut control_point_1_x = 0.0;
                    let mut control_point_1_y = 0.0;
                    if prev_command == "S" || prev_command == "C" {
                        // first control point is reflection of second control point on the previous command relative to the current point
                        let data = prev_match.split(", ").collect::<Vec<&str>>();
                        mainstruct.data.log.push(format!("data: {:?}", data));
                        control_point_1_x =
                            prev_point.0 + (prev_point.0 - data[3].parse::<f64>().unwrap());
                        control_point_1_y =
                            prev_point.1 + (prev_point.1 - data[4].parse::<f64>().unwrap());
                    } else {
                        control_point_1_x = prev_point.0;
                        control_point_1_y = prev_point.1;
                    }
                    // Smooth Cubic Bezier Curve
                    let control_point_2_x = data[1].parse::<f64>().unwrap() / x_scale;
                    let control_point_2_y = data[2].parse::<f64>().unwrap() / y_scale;
                    let end_point_x = data[3].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = data[4].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = cubic_bezier_curve(
                            &prev_point,
                            &(control_point_1_x, control_point_1_y),
                            &(control_point_2_x, control_point_2_y),
                            &(end_point_x, end_point_y),
                            t,
                            ratio,
                            Some(transform_str.clone()),
                            (x_scale, y_scale)
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y);
                    prev_command = command;
                }
                "T" => {
                    // Smooth Quadratic Bezier Curve
                    let mut control_point_x = 0.0;
                    let mut control_point_y = 0.0;
                    if prev_command == "T" || prev_command == "Q" {
                        // first control point is reflection of second control point on the previous command relative to the current point
                        let data = prev_match.split(", ").collect::<Vec<&str>>();
                        mainstruct.data.log.push(format!("data: {:?}", data));
                        control_point_x =
                            prev_point.0 + (prev_point.0 - data[1].parse::<f64>().unwrap());
                        control_point_y =
                            prev_point.1 + (prev_point.1 - data[2].parse::<f64>().unwrap());
                    } else {
                        control_point_x = prev_point.0;
                        control_point_y = prev_point.1;
                    }
                    let end_point_x = data[1].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = data[2].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = quadratic_bezier_curve(
                            &prev_point,
                            &(control_point_x, control_point_y),
                            &(end_point_x, end_point_y),
                            t,
                            ratio,
                            Some(transform_str.clone()),
                            (x_scale, y_scale)
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y);
                    prev_command = command;
                }
                "A" => {
                    // Elliptical Arc
                    let rx = data[1].parse::<f64>().unwrap() / x_scale;
                    let ry = data[2].parse::<f64>().unwrap() / y_scale;
                    let x_axis_rotation = data[3].parse::<f64>().unwrap();
                    let large_arc_flag = data[4].parse::<f64>().unwrap();
                    let sweep_flag = data[5].parse::<f64>().unwrap();
                    let end_point_x = data[6].parse::<f64>().unwrap() / x_scale;
                    let end_point_y = data[7].parse::<f64>().unwrap() / y_scale;
                    for i in 0..100 {
                        let t = i as f64 / 100.0;
                        let point = elliptical_arc(
                            &prev_point,
                            &(rx, ry),
                            x_axis_rotation,
                            large_arc_flag,
                            sweep_flag,
                            &(end_point_x, end_point_y),
                            t,
                            ratio,
                            Some(transform_str.clone()),
                            (x_scale, y_scale)
                        );
                        points.push(point);
                    }
                    prev_point = (end_point_x, end_point_y);
                    prev_command = command;
                }
                "H" => {
                    // Horizontal Line
                    let mut end_point_x = data[1].parse::<f64>().unwrap() / x_scale;
                    let mut end_point_y = prev_point.1;
                    if Some(transform_str) == None {
                        continue;
                    } else {
                        let transformed_points = transform(end_point_x, end_point_y, transform_str, (x_scale, y_scale));
                        end_point_x = transformed_points.0;
                        end_point_y = transformed_points.1;
                    }
                    points.push((end_point_x, end_point_y));
                    prev_point = (end_point_x, end_point_y);
                    prev_command = command;
                }
                "V" => {
                    // Vertical Line
                    let mut end_point_x = prev_point.0;
                    let mut end_point_y = data[1].parse::<f64>().unwrap() / y_scale;
                    if Some(transform_str) == None {
                        continue;
                    } else {
                        let transformed_points = transform(end_point_x, end_point_y, transform_str, (x_scale, y_scale));
                        end_point_x = transformed_points.0;
                        end_point_y = transformed_points.1;
                    }
                    points.push((end_point_x, end_point_y));
                    prev_point = (end_point_x, end_point_y);
                    prev_command = command;
                }

                _ => {}
            }
        }

        //println!("{:?}", data);
    }

    return points;
}
fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(r) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}
fn quadratic_bezier_curve(
    start: &(f64, f64),
    control: &(f64, f64),
    end: &(f64, f64),
    t: f64,
    ratio: f64,
    transform_str: Option<&str>,
    scales: (f64, f64),
) -> (f64, f64) {
    let x = (1.0 - t).powi(2) * start.0 + 2.0 * (1.0 - t) * t * control.0 + t.powi(2) * end.0;
    let y = (1.0 - t).powi(2) * start.1 + 2.0 * (1.0 - t) * t * control.1 + t.powi(2) * end.1;
    // if transform exists use transform, else return x,y
    if transform_str == None {
        (x, y)
    } else {
        transform(x, y, transform_str.unwrap(), scales)
    }
}

fn cubic_bezier_curve(
    start: &(f64, f64),
    control_1: &(f64, f64),
    control_2: &(f64, f64),
    end: &(f64, f64),
    t: f64,
    ratio: f64,
    transform_str: Option<&str>,
    scales: (f64, f64)
) -> (f64, f64) {
    let x = (1.0 - t).powi(3) * start.0
        + 3.0 * (1.0 - t).powi(2) * t * control_1.0
        + 3.0 * (1.0 - t) * t.powi(2) * control_2.0
        + t.powi(3) * end.0;
    let y = 100.0
        - ((1.0 - t).powi(3) * start.1
            + 3.0 * (1.0 - t).powi(2) * t * control_1.1
            + 3.0 * (1.0 - t) * t.powi(2) * control_2.1
            + t.powi(3) * end.1);
    // if transform exists use transform, else return x,y
    if transform_str == None {
        (x, y)
    } else {
        transform(x, y, transform_str.unwrap(), scales)
    }
}

fn elliptical_arc(
    start: &(f64, f64),
    radii: &(f64, f64),
    x_axis_rotation: f64,
    large_arc_flag: f64,
    sweep_flag: f64,
    end: &(f64, f64),
    t: f64,
    ratio: f64,
    transform_str: Option<&str>,
    scales: (f64, f64),
) -> (f64, f64) {
    let x1 = start.0;
    let y1 = start.1;
    let x2 = end.0;
    let y2 = end.1;
    let rx = radii.0;
    let ry = radii.1;
    let phi = x_axis_rotation;
    let fA = large_arc_flag;
    let fS = sweep_flag;
    let x1p = (x1 - x2) / 2.0 * phi.cos() + (y1 - y2) / 2.0 * phi.sin();
    let y1p = -(x1 - x2) / 2.0 * phi.sin() + (y1 - y2) / 2.0 * phi.cos();
    let rxs = rx.powi(2);
    let rys = ry.powi(2);
    let x1ps = x1p.powi(2);
    let y1ps = y1p.powi(2);
    let lambda = x1ps / rxs + y1ps / rys;
    let c = if lambda > 1.0 { lambda.sqrt() } else { 1.0 };
    let cxp = c * rx * y1p / ry;
    let cyp = c * -ry * x1p / rx;
    let cx = cxp * phi.cos() - cyp * phi.sin() + (x1 + x2) / 2.0;
    let cy = cxp * phi.sin() + cyp * phi.cos() + (y1 + y2) / 2.0;
    let theta = (x1p - cxp) / rx;
    let delta = (x1p * -y1p / (rx * ry)).atan2((1.0 - x1ps / rxs - y1ps / rys).sqrt());
    let t1 = if theta < 0.0 { theta + 2.0 * PI } else { theta };
    let t2 = if fS == 0.0 { t1 + delta } else { t1 - delta };
    let t = t1 + (t2 - t1) * t;
    let x = cx + rx * t.cos();
    let y = cy + ry * t.sin();
    // if transform exists use transform, else return x,y
    if transform_str == None {
        (x, y)
    } else {
        transform(x, y, transform_str.unwrap(), scales)
    }
}
fn transform(x: f64, y: f64, transform: &str, scales: (f64, f64)) -> (f64, f64) {
    if transform.starts_with("matrix") {
        //matrix(0.963456, 0.267867, -0.267867, 0.963456, -67.327988, 51.384823)
        let data = transform.strip_prefix("matrix(").unwrap();
        let data = data.strip_suffix(")").unwrap();
        let data = data.split(", ").collect::<Vec<&str>>();
        let a = data[0].parse::<f64>().unwrap();
        let b = data[1].parse::<f64>().unwrap();
        let c = data[2].parse::<f64>().unwrap();
        let d = data[3].parse::<f64>().unwrap();
        let e = data[4].parse::<f64>().unwrap();
        let f = data[5].parse::<f64>().unwrap();
        let x = x * a + y * c + e;
        let y = x * b + y * d + f;
        return (x, y);
    } else if transform.starts_with("translate") {
        let data = transform.strip_prefix("translate(").unwrap();
        let data = data.strip_suffix(")").unwrap();
        let data = data.split(" ").collect::<Vec<&str>>();
        let x = x + data[0].parse::<f64>().unwrap() / scales.0;
        let y = y + data[1].parse::<f64>().unwrap() / scales.1;
        return (x, y);
    } else if transform.starts_with("scale") {
        let data = transform.strip_prefix("scale(").unwrap();
        let data = data.strip_suffix(")").unwrap();
        let data = data.split(" ").collect::<Vec<&str>>();
        let x = x * data[0].parse::<f64>().unwrap();
        let y = y * data[1].parse::<f64>().unwrap();
        return (x, y);
    } else if transform.starts_with("rotate") {
        let data = transform.strip_prefix("rotate(").unwrap();
        let data = data.strip_suffix(")").unwrap();
        let data = data.split(" ").collect::<Vec<&str>>();
        let angle = data[0].parse::<f64>().unwrap();
        let x = x * angle.cos() - y * angle.sin();
        let y = x * angle.sin() + y * angle.cos();
        return (x, y);
    } else if transform.starts_with("skewX") {
        let data = transform.strip_prefix("skewX(").unwrap();
        let data = data.strip_suffix(")").unwrap();
        let data = data.split(" ").collect::<Vec<&str>>();
        let angle = data[0].parse::<f64>().unwrap();
        let x = x + y * angle.tan();
        return (x, y);
    } else if transform.starts_with("skewY") {
        let data = transform.strip_prefix("skewY(").unwrap();
        let data = data.strip_suffix(")").unwrap();
        let data = data.split(" ").collect::<Vec<&str>>();
        let angle = data[0].parse::<f64>().unwrap();
        let y = y + x * angle.tan();
        return (x, y);
    } else {
        return (x, y);
    }
}