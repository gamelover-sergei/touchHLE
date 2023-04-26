/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! EAGL.

use super::{GLES1OnGL2, GLES};
use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::foundation::ns_string::get_static_str;
use crate::frameworks::foundation::NSUInteger;
use crate::objc::{id, msg, nil, objc_classes, release, retain, ClassExports, HostObject};
use crate::window::gles11;
use crate::window::{Matrix, Window};

// These are used by the EAGLDrawable protocol implemented by CAEAGLayer.
// Since these have the ABI of constant symbols rather than literal constants,
// the values shouldn't matter, and haven't been checked against real iPhone OS.
pub const kEAGLDrawablePropertyColorFormat: &str = "ColorFormat";
pub const kEAGLDrawablePropertyRetainedBacking: &str = "RetainedBacking";
pub const kEAGLColorFormatRGBA8: &str = "RGBA8";
pub const kEAGLColorFormatRGB565: &str = "RGB565";

pub const CONSTANTS: ConstantExports = &[
    (
        "_kEAGLDrawablePropertyColorFormat",
        HostConstant::NSString(kEAGLDrawablePropertyColorFormat),
    ),
    (
        "_kEAGLDrawablePropertyRetainedBacking",
        HostConstant::NSString(kEAGLDrawablePropertyRetainedBacking),
    ),
    (
        "_kEAGLColorFormatRGBA8",
        HostConstant::NSString(kEAGLColorFormatRGBA8),
    ),
    (
        "_kEAGLColorFormatRGB565",
        HostConstant::NSString(kEAGLColorFormatRGB565),
    ),
];

type EAGLRenderingAPI = u32;
const kEAGLRenderingAPIOpenGLES1: EAGLRenderingAPI = 1;
#[allow(dead_code)]
const kEAGLRenderingAPIOpenGLES2: EAGLRenderingAPI = 2;
#[allow(dead_code)]
const kEAGLRenderingAPIOpenGLES3: EAGLRenderingAPI = 3;

pub(super) struct EAGLContextHostObject {
    pub(super) gles_ctx: Option<Box<dyn GLES>>,
}
impl HostObject for EAGLContextHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation EAGLContext: NSObject

+ (id)alloc {
    let host_object = Box::new(EAGLContextHostObject { gles_ctx: None });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)currentContext {
    env.framework_state.opengles.current_ctx_for_thread(env.current_thread).unwrap_or(nil)
}
+ (bool)setCurrentContext:(id)context { // EAGLContext*
    retain(env, context);

    // Clear flag value, we're changing context anyway.
    let _ = env.window.is_app_gl_ctx_no_longer_current();

    let current_ctx = env.framework_state.opengles.current_ctx_for_thread(env.current_thread);

    if let Some(old_ctx) = std::mem::take(current_ctx) {
        release(env, old_ctx);
        env.framework_state.opengles.current_ctx_thread = None;
    }

    // reborrow
    let current_ctx = env.framework_state.opengles.current_ctx_for_thread(env.current_thread);

    if context != nil {
        let host_obj = env.objc.borrow_mut::<EAGLContextHostObject>(context);
        host_obj.gles_ctx.as_mut().unwrap().make_current(&mut env.window);
        *current_ctx = Some(context);
        env.framework_state.opengles.current_ctx_thread = Some(env.current_thread);
    }

    true
}

- (id)initWithAPI:(EAGLRenderingAPI)api {
    assert!(api == kEAGLRenderingAPIOpenGLES1);

    let gles1_ctx = Box::new(GLES1OnGL2::new(&mut env.window));

    *env.objc.borrow_mut(this) = EAGLContextHostObject {
        gles_ctx: Some(gles1_ctx),
    };

    this
}

- (bool)renderbufferStorage:(NSUInteger)target
               fromDrawable:(id)drawable { // EAGLDrawable (always CAEAGLayer*)
    assert!(target == gles11::RENDERBUFFER_OES);

    let props: id = msg![env; drawable drawableProperties];

    let format_key = get_static_str(env, kEAGLDrawablePropertyColorFormat);
    let format_rgba8 = get_static_str(env, kEAGLColorFormatRGBA8);
    let format_rgb565 = get_static_str(env, kEAGLColorFormatRGB565);

    let format: id = msg![env; props objectForKey:format_key];
    // Theoretically this should map formats like:
    // - kColorFormatRGBA8 => RGBA8_OES
    // - kColorFormatRGB565 => RGB565_OES
    // However, the specification of EXT_framebuffer_object allows the
    // implementation to arbitrarily restrict which formats can be rendered to,
    // and it seems like RGB565 isn't supported, at least on a machine with
    // Intel HD Graphics 615 running macOS Monterey. I don't think RGBA8 is
    // guaranteed either, but it at least seems to work.
    if !msg![env; format isEqualTo:format_rgba8] && !msg![env; format isEqualTo:format_rgb565] {
        log!("[renderbufferStorage:{:?} fromDrawable:{:?}] Warning: unhandled format {:?}, using RGBA8", target, drawable, format);
    }
    let internalformat = gles11::RGBA8_OES;

    // FIXME: get width and height from the layer!
    let (width, height) = env.window.size_unrotated_scalehacked();

    // Unclear from documentation if this method requires an appropriate context
    // to already be active, but that seems to be the case in practice?
    let gles = super::sync_context(&mut env.framework_state.opengles, &mut env.objc, &mut env.window, env.current_thread);
    unsafe {
        gles.RenderbufferStorageOES(target, internalformat, width.try_into().unwrap(), height.try_into().unwrap())
    }

    true
}

- (bool)presentRenderbuffer:(NSUInteger)target {
    assert!(target == gles11::RENDERBUFFER_OES);

    // Unclear from documentation if this method requires an appropriate context
    // to already be active, but that seems to be the case in practice?
    let gles = super::sync_context(&mut env.framework_state.opengles, &mut env.objc, &mut env.window, env.current_thread);
    unsafe {
        present_renderbuffer(gles, &mut env.window);
    }

    true
}

@end

};

/// Copies the renderbuffer provided by the app to the window's framebuffer,
/// rotated if necessary, and presents that framebuffer.
unsafe fn present_renderbuffer(gles: &mut dyn GLES, window: &mut Window) {
    use gles11::types::*;

    // We are partially bypassing the OpenGL ES 1.1 abstraction layer, because
    // OpenGL ES 1.1 doesn't have the state attribute stack functions.
    // TODO: Replace these with alternatives that don't need this bypass.
    use crate::window::gl21compat as gl21;

    // We can't directly copy the content of the renderbuffer to the default
    // framebuffer (the window), but if we attach it to a framebuffer object, we
    // can use glCopyTexImage2D() to copy it to a texture, which we can then
    // draw to the default framebuffer via a textured quad, which can be
    // rotated, scaled or letterboxed as appropriate.

    let mut renderbuffer: GLuint = 0;
    let mut width: GLint = 0;
    let mut height: GLint = 0;
    gles.GetIntegerv(
        gles11::RENDERBUFFER_BINDING_OES,
        &mut renderbuffer as *mut _ as *mut _,
    );
    gles.GetRenderbufferParameterivOES(
        gles11::RENDERBUFFER_OES,
        gles11::RENDERBUFFER_WIDTH_OES,
        &mut width,
    );
    gles.GetRenderbufferParameterivOES(
        gles11::RENDERBUFFER_OES,
        gles11::RENDERBUFFER_HEIGHT_OES,
        &mut height,
    );

    // To avoid confusing the guest app, we need to be able to undo any
    // state changes we make.
    let mut old_framebuffer: GLuint = 0;
    let mut old_texture_2d: GLuint = 0;
    gles.GetIntegerv(
        gles11::FRAMEBUFFER_BINDING_OES,
        &mut old_framebuffer as *mut _ as *mut _,
    );
    gles.GetIntegerv(
        gles11::TEXTURE_BINDING_2D,
        &mut old_texture_2d as *mut _ as *mut _,
    );

    // Create a framebuffer we can use to read from the renderbuffer
    let mut src_framebuffer = 0;
    gles.GenFramebuffersOES(1, &mut src_framebuffer);
    gles.BindFramebufferOES(gles11::FRAMEBUFFER_OES, src_framebuffer);
    gles.FramebufferRenderbufferOES(
        gles11::FRAMEBUFFER_OES,
        gles11::COLOR_ATTACHMENT0_OES,
        gles11::RENDERBUFFER_OES,
        renderbuffer,
    );

    // Create a texture with a copy of the pixels in the framebuffer
    let mut texture: GLuint = 0;
    gles.GenTextures(1, &mut texture);
    gles.BindTexture(gles11::TEXTURE_2D, texture);
    gles.CopyTexImage2D(
        gles11::TEXTURE_2D,
        0,
        gles11::RGB as _,
        0,
        0,
        width,
        height,
        0,
    );
    // The texture will not have any mip levels so we must ensure the filter
    // does not use them, else rendering will fail.
    gles.TexParameteri(
        gles11::TEXTURE_2D,
        gles11::TEXTURE_MIN_FILTER,
        gles11::LINEAR as _,
    );

    // Clean up the framebuffer object since we no longer need it.
    // This also sets the framebuffer bindings back to zero, so rendering
    // will go to the default framebuffer (the window).
    gles.DeleteFramebuffersOES(1, &src_framebuffer);

    // There are a huge number of pieces of state that can affect rendering.
    // Backing up and then clearing all of it is the easiest way to ensure
    // that drawing the quad works.
    gl21::PushClientAttrib(gl21::CLIENT_ALL_ATTRIB_BITS);
    for array in super::gles1_on_gl2::ARRAYS {
        gles.DisableClientState(array.name);
    }
    gl21::PushAttrib(gl21::ALL_ATTRIB_BITS);
    for &cap in super::gles1_on_gl2::CAPABILITIES {
        gles.Disable(cap);
    }
    let mut old_matrix_mode: GLenum = 0;
    gles.GetIntegerv(
        gles11::MATRIX_MODE,
        &mut old_matrix_mode as *mut _ as *mut _,
    );
    for mode in [gles11::MODELVIEW, gles11::PROJECTION, gles11::TEXTURE] {
        gles.MatrixMode(mode);
        gles.PushMatrix();
        gles.LoadIdentity();
    }
    let mut old_array_buffer: GLuint = 0;
    gles.GetIntegerv(
        gles11::ARRAY_BUFFER_BINDING,
        &mut old_array_buffer as *mut _ as *mut _,
    );
    gles.Color4f(1.0, 1.0, 1.0, 1.0);

    // Draw the quad
    let viewport = window.viewport();
    gles.Viewport(
        viewport.0 as _,
        viewport.1 as _,
        viewport.2 as _,
        viewport.3 as _,
    );
    gles.ClearColor(0.0, 0.0, 0.0, 1.0);
    gles.Clear(gles11::COLOR_BUFFER_BIT | gles11::DEPTH_BUFFER_BIT | gles11::STENCIL_BUFFER_BIT);
    gles.BindBuffer(gles11::ARRAY_BUFFER, 0);
    let vertices: [f32; 12] = [
        -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0,
    ];
    gles.EnableClientState(gles11::VERTEX_ARRAY);
    gles.VertexPointer(2, gles11::FLOAT, 0, vertices.as_ptr() as *const GLvoid);
    let tex_coords: [f32; 12] = [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    gles.EnableClientState(gles11::TEXTURE_COORD_ARRAY);
    gles.TexCoordPointer(2, gles11::FLOAT, 0, tex_coords.as_ptr() as *const GLvoid);
    let matrix = Matrix::<4>::from(&window.output_rotation_matrix());
    gles.MatrixMode(gles11::TEXTURE);
    gles.LoadMatrixf(matrix.columns().as_ptr() as *const _);
    gles.Enable(gles11::TEXTURE_2D);
    gles.DrawArrays(gles11::TRIANGLES, 0, 6);

    // Display virtual cursor
    if let Some((x, y, pressed)) = window.virtual_cursor_visible_at() {
        let (vx, vy, vw, vh) = viewport;
        let x = x - vx as f32;
        let y = y - vy as f32;

        gles.DisableClientState(gles11::TEXTURE_COORD_ARRAY);
        gles.Disable(gles11::TEXTURE_2D);

        gles.Enable(gles11::BLEND);
        gles.BlendFunc(gles11::ONE, gles11::ONE_MINUS_SRC_ALPHA);
        gles.Color4f(0.0, 0.0, 0.0, if pressed { 2.0 / 3.0 } else { 1.0 / 3.0 });

        let radius = 10.0;

        let mut vertices = vertices;
        for i in (0..vertices.len()).step_by(2) {
            vertices[i] = (vertices[i] * radius + x) / (vw as f32 / 2.0) - 1.0;
            vertices[i + 1] = 1.0 - (vertices[i + 1] * radius + y) / (vh as f32 / 2.0);
        }
        gles.VertexPointer(2, gles11::FLOAT, 0, vertices.as_ptr() as *const GLvoid);
        gles.DrawArrays(gles11::TRIANGLES, 0, 6);
    }

    // Clean up the texture
    gles.DeleteTextures(1, &texture);

    // Restore all the state saved before rendering
    gles.BindBuffer(gles11::ARRAY_BUFFER, old_array_buffer);
    for mode in [gles11::MODELVIEW, gles11::PROJECTION, gles11::TEXTURE] {
        gles.MatrixMode(mode);
        gles.PopMatrix();
    }
    gles.MatrixMode(old_matrix_mode);
    gl21::PopAttrib();
    gl21::PopClientAttrib();

    // SDL2's documentation warns 0 should be bound to the draw framebuffer
    // when swapping the window, so this is the perfect moment.
    window.swap_window();

    // Restore the other bindings
    gles.BindTexture(gles11::TEXTURE_2D, old_texture_2d);
    gles.BindFramebufferOES(gles11::FRAMEBUFFER_OES, old_framebuffer);

    //{ let err = gl21::GetError(); if err != 0 { panic!("{:#x}", err); } }
}
