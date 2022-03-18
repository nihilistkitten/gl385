use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{bind_buffer, fill_buffer, make_shader_program};

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("the current html has a window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// Setup web-ready error handling.
fn setup_error_handling() {
    // report rust stack traces to the console across wasm/js ffi boundaries
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // log errors to the console
    console_log::init().expect("could not initialize logger");
}

/// Get the webgl context object for the target canvas.
fn get_webgl_context() -> Result<web_sys::WebGl2RenderingContext, JsValue> {
    Ok(web_sys::window()
        .expect("the current html has a window")
        .document()
        .expect("the current window has a document")
        .get_element_by_id("glcanvas")
        .expect("the current document has an element with id glcanvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()?
        .get_context("webgl2")?
        .expect("the browser supports webgl2")
        .dyn_into()?)
}

/// Set up the webgl program.
fn create_program(context: &WebGl2RenderingContext) -> WebGlProgram {
    let program = make_shader_program(
        context,
        include_str!("vertex.glsl"),
        include_str!("fragment.glsl"),
    )
    .unwrap(); // we error early if shader creation fails
    context.use_program(Some(&program));

    program
}

struct AttributeLocations {
    position: u32,
}

impl AttributeLocations {
    fn lookup(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<Self, JsValue> {
        let position = context
            .get_attrib_location(program, "position")
            .try_into()
            .map_err(|_| JsValue::from_str("position attribute not found"))?;

        Ok(Self { position })
    }
}

pub trait App: Sized + 'static {
    fn update(&mut self) -> bool;
    fn render(&self) -> Vec<(f32, f32, f32)>;

    /// # Errors
    /// On a javascript error.
    fn run(mut self) -> Result<(), JsValue> {
        setup_error_handling();
        let context = get_webgl_context()?;
        let program = create_program(&context);

        let locs = AttributeLocations::lookup(&context, &program)?;

        // some stuff to get the borrow checker to let us meet the javascript api, this is kind of
        // janky but it's the best solution I can come up with. We need a clone of the
        // reference-counted function pointer so we can move one copy of the function into the
        // closure for the recursive call.
        let frame = Rc::new(RefCell::new(None));
        let frame_clone = frame.clone();

        *frame_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if self.update() {
                let vertices = self.render();
                draw(&vertices, &context, &locs);
            }
            request_animation_frame(frame.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(frame_clone.borrow().as_ref().unwrap());
        Ok(())
    }
}

fn draw(vertices: &[(f32, f32, f32)], context: &WebGl2RenderingContext, locs: &AttributeLocations) {
    let vertices = vertices
        .iter()
        .fold(Vec::with_capacity(vertices.len() * 3), |mut v, t| {
            v.push(t.0);
            v.push(t.1);
            v.push(t.2);
            v
        });

    bind_buffer(context).unwrap();
    fill_buffer(context, &vertices);

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")
        .unwrap();
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        locs.position,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(locs.position);

    context.bind_vertex_array(Some(&vao));

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    // vertices should be sufficiently small
    let vert_count = (vertices.len() / 3) as i32;

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}
