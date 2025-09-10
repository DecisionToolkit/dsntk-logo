use domrs::{HtmlBodyElement, HtmlDocument, HtmlElement, HtmlHeadElement, SvgDocument, SvgNumber, ToText};
use std::f64::consts::PI;
use std::fmt::Write;
use std::fs;

type Palette = (&'static str, &'static str);

const PALETTE_GREY: Palette = ("#90a4ae", "#37474f");
const PALETTE_GREEN: Palette = ("#64DD17", "#33691E");
const PALETTE_BLUE: Palette = ("#00B0FF", "#01579B");
const PALETTE_PURPLE: Palette = ("#EA80FC", "#4A148C");
const PALETTE_ORANGE: Palette = ("#FF9E80", "#BF360C");
const PREFIX_BLUE: &str = "blue";
const PREFIX_GREEN: &str = "green";
const PREFIX_ORANGE: &str = "orange";
const PREFIX_GREY: &str = "grey";
const PREFIX_PURPLE: &str = "purple";

/// Converts degrees into radians.
fn deg_to_rad(deg: f64) -> f64 {
  deg * 2.0 * PI / 360.0
}

fn create_path_points(x: f64, y: f64, r: f64) -> Vec<(f64, f64)> {
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

fn create_svg_rect(x: f64, y: f64, width: f64, height: f64, bg_color: &str) -> HtmlElement {
  let mut rect = HtmlElement::new("rect");
  rect.set_attribute("x", format!("{:.1}", x));
  rect.set_attribute("y", format!("{:.1}", y));
  rect.set_attribute("width", format!("{:.1}", width));
  rect.set_attribute("height", format!("{:.1}", height));
  rect.set_attribute("stroke", "none");
  rect.set_attribute("fill", bg_color);
  rect
}

fn create_svg_path(points: &[(f64, f64)], fg_color: &str) -> HtmlElement {
  let mut d = String::new();
  let _ = write!(&mut d, "M {:.1},{:.1}", points[0].0, points[0].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[1].0, points[1].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[2].0, points[2].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[3].0, points[3].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[4].0, points[4].1);
  let _ = write!(&mut d, " L {:.1},{:.1}", points[5].0, points[5].1);
  let _ = write!(&mut d, " Z");
  let mut path = HtmlElement::new("path");
  path.set_attribute("d", d);
  path.set_attribute("stroke", "none");
  path.set_attribute("fill", fg_color);
  path
}

fn create_svg_line(x1: f64, y1: f64, x2: f64, y2: f64, bg_color: &str, line_width: f64) -> HtmlElement {
  let mut line = HtmlElement::new("line");
  line.set_attribute("x1", format!("{:.1}", x1));
  line.set_attribute("y1", format!("{:.1}", y1));
  line.set_attribute("x2", format!("{:.1}", x2));
  line.set_attribute("y2", format!("{:.1}", y2));
  line.set_attribute("stroke", bg_color);
  line.set_attribute("stroke-width", format!("{:.1}", line_width));
  line.set_attribute("stroke-linecap", "square");
  line
}

fn create_svg(width: f64, height: f64, line_width: f64, palette: Palette, radius_coeff: f64) -> HtmlElement {
  let mut svg: HtmlElement = SvgDocument::new()
    .default_namespace()
    .width(SvgNumber::new(width, 1))
    .height(SvgNumber::new(height, 1))
    .into();

  let w_2 = width / 2.0;
  let h_2 = height / 2.0;
  let radius = (if w_2 < h_2 { w_2 } else { h_2 }) * radius_coeff;

  let points = create_path_points(w_2, h_2, radius);
  let coefficient = points[3].1 - points[2].1;

  svg.add_child(create_svg_rect(0.0, 0.0, width, height, palette.1));
  svg.add_child(create_svg_path(&points, palette.0));
  svg.add_child(create_svg_line(
    points[2].0,
    points[2].1 + 0.15 * coefficient,
    points[4].0 + 0.35 * coefficient,
    points[4].1,
    palette.1,
    line_width,
  ));
  svg.add_child(create_svg_line(
    points[1].0 - 0.23 * coefficient,
    points[1].1,
    points[5].0,
    points[5].1 + 0.12 * coefficient,
    palette.1,
    line_width,
  ));
  svg.add_child(create_svg_line(points[1].0 + 0.11 * coefficient, points[1].1, points[3].0, points[4].1, palette.1, line_width));
  svg.add_child(create_svg_line(
    points[1].0 + 0.65 * coefficient,
    points[1].1,
    points[3].0 + 0.35 * coefficient,
    points[4].1,
    palette.1,
    line_width,
  ));
  svg.add_child(create_svg_line(
    points[0].0 + 2.0,
    points[5].1 - 0.45 * coefficient,
    points[4].0 + 0.35 * coefficient,
    points[4].1,
    palette.1,
    line_width,
  ));
  svg
}

fn save_svg(svg: HtmlElement, file_name: &str) {
  fs::write(file_name, svg.to_text(0, 2)).expect("writing output file failed");
}

fn file_name(prefix: &str, size: f64, radius_coeff: f64, ext: &str) -> String {
  format!("./out/{}-{:.0}-{:02.0}.{}", prefix, size, radius_coeff * 10.0, ext)
}

fn main() {
  let head = HtmlHeadElement::default().charset("UTF-8").title("LOGO");
  let mut body = HtmlBodyElement::default();
  // green
  let green_700_09 = create_svg(700.0, 700.0, 7.0, PALETTE_GREEN, 0.9);
  let green_700_07 = create_svg(700.0, 700.0, 7.0, PALETTE_GREEN, 0.7);
  // blue
  let blue_700_09 = create_svg(700.0, 700.0, 7.0, PALETTE_BLUE, 0.9);
  let blue_700_07 = create_svg(700.0, 700.0, 7.0, PALETTE_BLUE, 0.7);
  // orange
  let orange_700_09 = create_svg(700.0, 700.0, 7.0, PALETTE_ORANGE, 0.9);
  let orange_700_07 = create_svg(700.0, 700.0, 7.0, PALETTE_ORANGE, 0.7);
  // purple
  let purple_700_09 = create_svg(700.0, 700.0, 7.0, PALETTE_PURPLE, 0.9);
  let purple_700_07 = create_svg(700.0, 700.0, 7.0, PALETTE_PURPLE, 0.7);
  // grey
  let grey_700_09 = create_svg(700.0, 700.0, 7.0, PALETTE_GREY, 0.9);
  let grey_700_07 = create_svg(700.0, 700.0, 7.0, PALETTE_GREY, 0.7);

  body.add_child(green_700_09.clone());
  body.add_child(green_700_07.clone());
  body.add_child(blue_700_09.clone());
  body.add_child(blue_700_07.clone());
  body.add_child(orange_700_09.clone());
  body.add_child(orange_700_07.clone());
  body.add_child(purple_700_09.clone());
  body.add_child(purple_700_07.clone());
  body.add_child(grey_700_09.clone());
  body.add_child(grey_700_07.clone());

  let doc = HtmlDocument::new().default_doctype().default_language().default_namespace().head(head).body(body);
  doc.save("./out/dsntk-logo.html", 0, 2).expect("writing output file failed");

  save_svg(blue_700_09, &file_name(PREFIX_BLUE, 700.0, 0.9, "svg"));
  save_svg(blue_700_07, &file_name(PREFIX_BLUE, 700.0, 0.7, "svg"));

  save_svg(green_700_09, &file_name(PREFIX_GREEN, 700.0, 0.9, "svg"));
  save_svg(green_700_07, &file_name(PREFIX_GREEN, 700.0, 0.7, "svg"));

  save_svg(orange_700_09, &file_name(PREFIX_ORANGE, 700.0, 0.9, "svg"));
  save_svg(orange_700_07, &file_name(PREFIX_ORANGE, 700.0, 0.7, "svg"));

  save_svg(purple_700_09, &file_name(PREFIX_PURPLE, 700.0, 0.9, "svg"));
  save_svg(purple_700_07, &file_name(PREFIX_PURPLE, 700.0, 0.7, "svg"));

  save_svg(grey_700_09, &file_name(PREFIX_GREY, 700.0, 0.9, "svg"));
  save_svg(grey_700_07, &file_name(PREFIX_GREY, 700.0, 0.7, "svg"));
}
