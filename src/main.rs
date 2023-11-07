use domrs::{HtmlBodyElement, HtmlDocument, HtmlElement, HtmlHeadElement, HtmlLinkElement};
use std::f64::consts::PI;
use std::fmt::Write;
use std::{fmt, fs};

fn html_start() -> String {
  r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title>DSNTK logo</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Asap:wght@500&family=Dosis:wght@600&display=swap">
</head>
<body>
"#
  .to_string()
}

const HTML_END: &str = r#"
</body>
</html>
"#;

fn svg_start(width: f64, height: f64) -> String {
  format!(r#"<svg width="{:.1}" height="{:.1}">"#, width, height)
}

const SVG_END: &str = r#"
</svg>
"#;

const BG_COLOR: &str = "#336633";
const FG_COLOR: &str = "#99CC00";
const LINE_WIDTH: f64 = 7.0;

/// Converts degrees into radians.
fn deg_to_rad(deg: f64) -> f64 {
  deg * 2.0 * PI / 360.0
}

///
fn get_path_points(x: f64, y: f64, r: f64) -> Vec<(f64, f64)> {
  let mut points = vec![];
  let a = r * deg_to_rad(30.0).cos();
  let b = r * deg_to_rad(30.0).sin();
  points.push((x + a, y - b));
  points.push((x, y - r));
  points.push((x - a, y - b));
  points.push((x - a, y + b));
  points.push((x, y + r));
  points.push((x + a, y + b));
  points
}

fn write_rect(buffer: &mut dyn Write, x: f64, y: f64, width: f64, height: f64, bg_color: &str) -> Result<(), fmt::Error> {
  writeln!(
    buffer,
    r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" stroke="none" fill="{}"/>"#,
    x, y, width, height, bg_color
  )?;
  Ok(())
}

fn write_path(buffer: &mut dyn Write, points: &[(f64, f64)]) -> Result<(), fmt::Error> {
  write!(buffer, r#"<path d=""#)?;
  write!(buffer, "M {:.1},{:.1}", points[0].0, points[0].1)?;
  write!(buffer, " L {:.1},{:.1}", points[1].0, points[1].1)?;
  write!(buffer, " L {:.1},{:.1}", points[2].0, points[2].1)?;
  write!(buffer, " L {:.1},{:.1}", points[3].0, points[3].1)?;
  write!(buffer, " L {:.1},{:.1}", points[4].0, points[4].1)?;
  write!(buffer, " L {:.1},{:.1}", points[5].0, points[5].1)?;
  writeln!(buffer, r#" Z" stroke="none" fill="{}"/>"#, FG_COLOR)?;
  Ok(())
}

fn write_line(buffer: &mut dyn Write, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<(), fmt::Error> {
  writeln!(
    buffer,
    r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="{:.1}" stroke-linecap="square"/>"#,
    x1, y1, x2, y2, BG_COLOR, LINE_WIDTH
  )?;
  Ok(())
}

fn write_text(buffer: &mut dyn Write, text: &str, x: f64, y: f64, size: u32, color: &str) -> Result<(), fmt::Error> {
  writeln!(
    buffer,
    r#"<text x="{}" y="{}" style="font-family:Dosis,sans-serif;font-size:{}pt;font-weight:600" fill="{}">{}</text>"#,
    x, y, size, color, text
  )?;
  Ok(())
}

fn write_logo(buffer: &mut dyn Write) -> Result<(), fmt::Error> {
  let width = 620.0;
  let height = 620.0;

  writeln!(buffer, "{}\n\n", html_start().trim())?;
  writeln!(buffer, "{}\n\n", svg_start(width, height).trim())?;

  let points = get_path_points(310.0, 310.0, 220.0);
  let coeff = points[3].1 - points[2].1;
  write_rect(buffer, 0.0, 0.0, width, height, BG_COLOR)?;
  write_path(buffer, &points)?;
  write_line(buffer, points[2].0, points[2].1 + 0.15 * coeff, points[4].0 + 0.35 * coeff, points[4].1)?;
  write_line(buffer, points[1].0 - 0.23 * coeff, points[1].1, points[5].0, points[5].1 + 0.12 * coeff)?;
  write_line(buffer, points[1].0 + 0.11 * coeff, points[1].1, points[3].0, points[4].1)?;
  write_line(buffer, points[1].0 + 0.65 * coeff, points[1].1, points[3].0 + 0.35 * coeff, points[4].1)?;
  write_line(buffer, points[0].0 + 2.0, points[5].1 - 0.45 * coeff, points[4].0 + 0.35 * coeff, points[4].1)?;
  //write_text(buffer,"Decision Toolkit", 111.0, 540.0, 47, FG_COLOR)?;
  writeln!(buffer, "\n\n{}", SVG_END.trim())?;
  writeln!(buffer, "\n\n{}", HTML_END.trim())?;
  Ok(())
}

fn main() {
  let mut buffer = String::new();
  write_logo(&mut buffer).unwrap();
  fs::write("./out/dsntk-logo.html", buffer).expect("writing file failed");

  let head = HtmlHeadElement::default()
    .with_charset("UTF-8")
    .with_title("DSNTK LOGO")
    .with_link(HtmlLinkElement::default().with_stylesheet("https://fonts.googleapis.com/css2?family=Asap:wght@500&family=Dosis:wght@600&display=swap"));

  let mut body = HtmlBodyElement::default();
  body.add_child(HtmlElement::new("div"));

  let html = HtmlDocument::new("en", head.into(), body.into());

  html.save("./out/logo.html").expect("writing file failed");
}
