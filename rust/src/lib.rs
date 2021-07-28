use serde::{Deserialize, Serialize};
use serde_json::{json,Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::WebGlShader;
use web_sys::window;
use web_sys::{Request, RequestInit, RequestMode, Response, HtmlElement, WebGlRenderingContext};

mod twgl;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn random_num() -> i32 {
    10001
}

const MY_VERTEX_SHADER: &str = r#"
    attribute vec4 position;
    uniform mat4 mvp;
    void main() {
        gl_Position = position;
    }
"#;

const MY_FRAGMENT_SHADER: &str = r#"
    void main() {
        gl_Color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

#[wasm_bindgen]
pub async fn webgl_painting(canvas_id: String) -> Result<JsValue, JsValue> {

    log!("Hello Console Log2");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    let canvas = document
        .get_element_by_id(&canvas_id)
        .expect("should have #loading on the page");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let m = twgl::create_program_from_sources(&gl, (MY_VERTEX_SHADER, MY_FRAGMENT_SHADER))?;
    let k = twgl::_uniform_getter(&gl, m);
    log!("{:?}", k);

    Ok(JsValue::from_serde(&json!({ "hi": "message3245" })).unwrap())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .expect("Unable to create shader object");

    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or("Unknown error creating shader"))
    }
}