/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Separate module just for the constant lists, since this will probably be a
//! very long and frequently-updated list.

use crate::frameworks::{
    avf_audio, core_animation, core_foundation, core_graphics, foundation, media_player, opengles,
    uikit,
};
use crate::libc;

/// All the lists of constants that the linker should search through.
pub const CONSTANT_LISTS: &[super::ConstantExports] = &[
    libc::ctype::CONSTANTS,
    libc::stdio::CONSTANTS,
    avf_audio::av_audio_session::CONSTANTS,
    core_animation::ca_transform_3d::CONSTANTS,
    core_foundation::cf_allocator::CONSTANTS,
    core_foundation::cf_run_loop::CONSTANTS,
    core_graphics::cg_affine_transform::CONSTANTS,
    core_graphics::cg_color_space::CONSTANTS,
    core_graphics::cg_geometry::CONSTANTS,
    foundation::ns_file_manager::CONSTANTS,
    foundation::ns_error::CONSTANTS,
    foundation::ns_exception::CONSTANTS,
    foundation::ns_run_loop::CONSTANTS,
    libc::mach::mach_task::CONSTANTS,
    media_player::movie_player::CONSTANTS,
    opengles::eagl::CONSTANTS,
    uikit::ui_application::CONSTANTS,
];
