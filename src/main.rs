use domrs::{HtmlBodyElement, HtmlDocument, HtmlElement, HtmlHeadElement, SvgDocument, SvgNumber};
use std::f64::consts::PI;
use std::fmt::Write;

type Palette = (&'static str, &'static str);

//const PALETTE_GREEN: Palette = ("#00C853", "#1B5E20");
const PALETTE_LIGHT_GREEN: Palette = ("#64DD17", "#33691E");
//const PALETTE_BLUE: Palette = ("#2962FF", "#0D47A1");
const PALETTE_LIGHT_BLUE: Palette = ("#00B0FF", "#01579B");
const PALETTE_PURPLE: Palette = ("#EA80FC", "#4A148C");
//const PALETTE_DEEP_PURPLE: Palette = ("#651FFF", "#311B92");
const PALETTE_YELLOW: Palette = ("#FFEA00", "#F57F17");
const PALETTE_DEEP_ORANGE: Palette = ("#FF9E80", "#BF360C");

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

fn create_svg(width: f64, height: f64, line_width: f64, palette: Palette) -> HtmlElement {
  let mut svg: HtmlElement = SvgDocument::new()
    .default_namespace()
    .width(SvgNumber::new(width, 1))
    .height(SvgNumber::new(height, 1))
    .into();

  let w_2 = width / 2.0;
  let h_2 = height / 2.0;
  let radius = (if w_2 < h_2 { w_2 } else { h_2 }) * 0.7;

  let points = create_path_points(w_2, h_2, radius);
  let coeff = points[3].1 - points[2].1;

  svg.add_child(create_svg_rect(0.0, 0.0, width, height, palette.1));
  svg.add_child(create_svg_path(&points, palette.0));
  svg.add_child(create_svg_line(
    points[2].0,
    points[2].1 + 0.15 * coeff,
    points[4].0 + 0.35 * coeff,
    points[4].1,
    palette.1,
    line_width,
  ));
  svg.add_child(create_svg_line(
    points[1].0 - 0.23 * coeff,
    points[1].1,
    points[5].0,
    points[5].1 + 0.12 * coeff,
    palette.1,
    line_width,
  ));
  svg.add_child(create_svg_line(points[1].0 + 0.11 * coeff, points[1].1, points[3].0, points[4].1, palette.1, line_width));
  svg.add_child(create_svg_line(
    points[1].0 + 0.65 * coeff,
    points[1].1,
    points[3].0 + 0.35 * coeff,
    points[4].1,
    palette.1,
    line_width,
  ));
  svg.add_child(create_svg_line(
    points[0].0 + 2.0,
    points[5].1 - 0.45 * coeff,
    points[4].0 + 0.35 * coeff,
    points[4].1,
    palette.1,
    line_width,
  ));
  svg
}

fn main() {
  let head = HtmlHeadElement::default().charset("UTF-8").title("DSNTK LOGO");
  let mut body = HtmlBodyElement::default();
  //body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_GREEN));
  body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_LIGHT_GREEN));
  //body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_BLUE));
  body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_LIGHT_BLUE));
  //body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_DEEP_PURPLE));
  body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_PURPLE));
  body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_DEEP_ORANGE));
  body.add_child(create_svg(700.0, 700.0, 7.0, PALETTE_YELLOW));
  let doc = HtmlDocument::new().default_doctype().default_language().default_namespace().head(head).body(body);
  doc.save("./out/dsntk-logo.html", 0, 2).expect("writing file failed");
}
