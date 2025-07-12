use dioxus::prelude::*;
use plotters::prelude::*;

#[component]
pub fn Plot(values: Vec<(f64, f64)>) -> Element {
    let mut buf = String::new();
    {
        let drawing_area = SVGBackend::with_string(&mut buf, (600, 300)).into_drawing_area();
        let mut chart = ChartBuilder::on(&drawing_area)
            .margin(1)
            .set_label_area_size(LabelAreaPosition::Left, (5i32).percent_width())
            .set_label_area_size(LabelAreaPosition::Bottom, (10i32).percent_height())
            .build_cartesian_2d(0f64..100f64, -4.0..4.0)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;
        chart.draw_series(LineSeries::new(values, &RED))?;
    }

    rsx! {
        div {
            dangerous_inner_html: "{buf}"
        }
    }
}
