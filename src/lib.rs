use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, size: f64, start_x: f64, start_y: f64) {
    context.move_to(start_x, start_y);
    context.begin_path();
    context.line_to(start_x - size / 2.0, start_y + size);
    context.line_to(start_x + size / 2.0, start_y + size);
    context.line_to(start_x, start_y);
    context.close_path();
    context.stroke();
}

fn draw_triangle_level(context: &web_sys::CanvasRenderingContext2d, size: f64, start_x: f64, start_y: f64, level: u32) {
    if level < 1 {
        return;
    }
    draw_triangle(&context, size, start_x, start_y);
    draw_triangle(&context, size, start_x - size / 2.0, start_y + size);
    draw_triangle(&context, size, start_x + size / 2.0, start_y + size);

    draw_triangle_level(&context, size / 2.0, start_x, start_y, level - 1);
    draw_triangle_level(&context, size / 2.0, start_x - size / 2.0, start_y + size, level - 1);
    draw_triangle_level(&context, size / 2.0, start_x + size / 2.0, start_y + size, level - 1);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    draw_triangle_level(&context, 300.0, 300.0, 0.0, 5);

    Ok(())
}
