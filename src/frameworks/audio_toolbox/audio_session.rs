/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `AudioSession.h` (Audio Session) // TODO: is this the real name?

use crate::abi::GuestFunction;
use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::carbon_core::OSStatus;
use crate::frameworks::core_audio_types::{debug_fourcc, fourcc};
use crate::frameworks::core_foundation::cf_run_loop::{CFRunLoopMode, CFRunLoopRef};
use crate::mem::{guest_size_of, ConstVoidPtr, GuestUSize, MutPtr, MutVoidPtr};
use crate::Environment;
use crate::frameworks::foundation::ns_string::{from_rust_string, to_rust_string};
use crate::objc::{autorelease, id};

type AudioSessionInterruptionListener = GuestFunction;

const kAudioSessionBadPropertySizeError: OSStatus = fourcc(b"!siz") as _;

/// Usually a FourCC.
type AudioSessionPropertyID = u32;
const kAudioSessionProperty_OtherAudioIsPlaying: AudioSessionPropertyID = fourcc(b"othr");
const kAudioSessionProperty_AudioCategory: AudioSessionPropertyID = fourcc(b"acat");
const kAudioSessionProperty_AudioRouteChange: AudioSessionPropertyID = fourcc(b"roch");
const kAudioSessionProperty_ServerDied: AudioSessionPropertyID = fourcc(b"died");
const kAudioSessionProperty_AudioInputAvailable: AudioSessionPropertyID = fourcc(b"aiav");
const kAudioSessionProperty_PreferredHardwareIOBufferDuration: AudioSessionPropertyID = fourcc(b"iobd");
const kAudioSessionProperty_PreferredHardwareSampleRate: AudioSessionPropertyID = fourcc(b"hwsr");
const kAudioSessionProperty_AudioRoute: AudioSessionPropertyID = fourcc(b"rout");

const kAudioSessionCategory_SoloAmbientSound: u32 = fourcc(b"solo");

fn AudioSessionInitialize(
    _env: &mut Environment,
    _in_run_loop: CFRunLoopRef,
    _in_run_loop_mode: CFRunLoopMode,
    _in_interruption_listener: AudioSessionInterruptionListener,
    _in_client_data: MutVoidPtr,
) -> OSStatus {
    // TODO: actually implement this
    0 // success
}

fn AudioSessionGetProperty(
    env: &mut Environment,
    in_ID: AudioSessionPropertyID,
    io_data_size: MutPtr<u32>,
    out_data: MutVoidPtr,
) -> OSStatus {
    let required_size: GuestUSize = match in_ID {
        kAudioSessionProperty_OtherAudioIsPlaying => guest_size_of::<u32>(),
        kAudioSessionProperty_AudioCategory => guest_size_of::<u32>(),
        kAudioSessionProperty_AudioInputAvailable => guest_size_of::<u32>(),
        kAudioSessionProperty_AudioRoute => guest_size_of::<id>(),
        _ => unimplemented!("Unimplemented property ID: {}", debug_fourcc(in_ID)),
    };
    if env.mem.read(io_data_size) != required_size {
        log!("Warning: AudioSessionGetProperty() failed");
        return kAudioSessionBadPropertySizeError;
    }

    match in_ID {
        kAudioSessionProperty_OtherAudioIsPlaying => {
            let value: u32 = 0;
            env.mem.write(out_data.cast(), value);
        }
        kAudioSessionProperty_AudioCategory => {
            // This is the default value. TODO: Actually support changing it?
            let value: u32 = kAudioSessionCategory_SoloAmbientSound;
            env.mem.write(out_data.cast(), value);
        }
        kAudioSessionProperty_AudioInputAvailable => {
            let value: u32 = 0;
            env.mem.write(out_data.cast(), value);
        }
        kAudioSessionProperty_AudioRoute => {
            let val = from_rust_string(env, "Speaker".to_string());
            autorelease(env, val);
            env.mem.write(out_data.cast(), val);
        }
        _ => unreachable!(),
    }

    0 // success
}

fn AudioSessionSetProperty(
    _env: &mut Environment,
    in_ID: AudioSessionPropertyID,
    in_data_size: u32,
    _in_data: ConstVoidPtr,
) -> OSStatus {
    let required_size: GuestUSize = match in_ID {
        kAudioSessionProperty_AudioCategory => guest_size_of::<u32>(),
        kAudioSessionProperty_PreferredHardwareIOBufferDuration => guest_size_of::<f32>(),
        kAudioSessionProperty_PreferredHardwareSampleRate => guest_size_of::<f64>(),
        _ => unimplemented!("Unimplemented property ID: {}", debug_fourcc(in_ID)),
    };
    if in_data_size != required_size {
        log!("Warning: AudioSessionGetProperty() failed");
        return kAudioSessionBadPropertySizeError;
    }

    // TODO: actually implement this

    0 // success
}

fn AudioSessionAddPropertyListener(
    _env: &mut Environment,
    in_id: AudioSessionPropertyID,
    _in_proc: GuestFunction,
    _in_client_data: ConstVoidPtr
) -> OSStatus {
    match in_id {
        kAudioSessionProperty_AudioRouteChange |
        kAudioSessionProperty_AudioInputAvailable |
        kAudioSessionProperty_ServerDied => {
            // do nothing, routing never changes
        },
        _ => unimplemented!()
    }
    0
}

fn AudioSessionSetActive(_env: &mut Environment, _active: bool) -> OSStatus {
    0 // success
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(AudioSessionInitialize(_, _, _, _)),
    export_c_func!(AudioSessionAddPropertyListener(_, _, _)),
    export_c_func!(AudioSessionGetProperty(_, _, _)),
    export_c_func!(AudioSessionSetProperty(_, _, _)),
    export_c_func!(AudioSessionSetActive(_)),
];
