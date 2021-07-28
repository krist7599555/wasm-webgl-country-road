use wasm_bindgen::UnwrapThrowExt;
use std::collections::HashMap;
use web_sys::{WebGlActiveInfo, WebGlProgram, WebGlRenderingContext, WebGlShader};

type GL = WebGlRenderingContext;

pub fn create_program_from_sources(
  gl: &GL,
  shader_sources: (&str, &str)
) -> Result<WebGlProgram, String> {
  let (vs, fs) = shader_sources;
  let vs = compile_shader(gl, GL::VERTEX_SHADER, vs).map_err(|e| "in vertex-shader: ".to_owned() + &e)?;
  let fs = compile_shader(gl, GL::FRAGMENT_SHADER, fs).map_err(|e| "in fragment-shader: ".to_owned() + &e)?;
  let program = link_program(gl, &vs, &fs)?;
  gl.use_program(Some(&program));
  Ok(program)
}

pub fn _uniform_getter(gl: &GL, program: WebGlProgram) -> Result<HashMap<String, WebGlActiveInfo>, String> {
  // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramParameter
  let num_uniforms = gl.get_program_parameter(&program, GL::ACTIVE_UNIFORMS).as_f64().unwrap() as u32;
  let mut result = HashMap::new();
  for ii in 0..num_uniforms {
    let uniform_info = gl.get_active_uniform(&program, ii).ok_or("can not get uniform at ii")?;
    let _location =  gl.get_uniform_location(&program, &uniform_info.name()).ok_or("can not get uniform location")?;
    result.insert(uniform_info.name(), uniform_info);
    // if (isBuiltIn(uniformInfo)) { // 
    //   function isBuiltIn(info) {
    //     const name = info.name;
    //     return name.startsWith("gl_") || name.startsWith("webgl_");
    //   }
    //     continue;
    // }
    // let name = uniformInfo.name;
    // // remove the array suffix.
    // if (name.endsWith("[0]")) {
    //   name = name.substr(0, name.length - 3);
    // }
    // let location = gl.getUniformLocation(program, uniformInfo.name);
    // // the uniform will have no location if it's in a uniform block
    // if (location) {
    //   uniformSetters[name] = createUniformSetter(program, uniformInfo, location);
    // }
  }
  Ok(result)
}

fn compile_shader(
  context: &GL,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
      .create_shader(shader_type)
      .ok_or("Unable to create shader object")?;

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
          .unwrap_or("Unknown error creating shader".to_owned()))
  }
}

pub fn link_program(
  context: &GL,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
      .create_program()
      .ok_or("Unable to create shader object")?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
      .get_program_parameter(&program, GL::LINK_STATUS)
      .as_bool()
      .unwrap_or(false)
  {
      Ok(program)
  } else {
      Err(context
          .get_program_info_log(&program)
          .unwrap_or("Unknown error creating program object".to_owned()))
  }
}