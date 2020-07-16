use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn player_collect(pl: *mut player, pcm: *mut libc::c_short,
                      samples: libc::c_uint);
    #[no_mangle]
    fn timecoder_submit(tc: *mut timecoder, pcm: *mut libc::c_short,
                        npcm: size_t);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type pid_t = __pid_t;
pub type pthread_spinlock_t = libc::c_int;
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free
 * Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut device_ops,
    pub timecoder: *mut timecoder,
    pub player: *mut player,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player {
    pub sample_dt: libc::c_double,
    pub lock: spin,
    pub track: *mut track,
    pub position: libc::c_double,
    pub target_position: libc::c_double,
    pub offset: libc::c_double,
    pub last_difference: libc::c_double,
    pub pitch: libc::c_double,
    pub sync_pitch: libc::c_double,
    pub volume: libc::c_double,
    pub timecoder: *mut timecoder,
    pub timecode_control: bool,
    pub recalibrate: bool,
}
/* samples since we last crossed zero */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecoder {
    pub def: *mut timecode_def,
    pub speed: libc::c_double,
    pub dt: libc::c_double,
    pub zero_alpha: libc::c_double,
    pub threshold: libc::c_int,
    pub forwards: bool,
    pub primary: timecoder_channel,
    pub secondary: timecoder_channel,
    pub pitch: pitch,
    pub ref_level: libc::c_int,
    pub bitstream: bits_t,
    pub timecode: bits_t,
    pub valid_counter: libc::c_uint,
    pub timecode_ticker: libc::c_uint,
    pub mon: *mut libc::c_uchar,
    pub mon_size: libc::c_int,
    pub mon_counter: libc::c_int,
}
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free
 * Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */
pub type bits_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pitch {
    pub dt: libc::c_double,
    pub x: libc::c_double,
    pub v: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecoder_channel {
    pub positive: bool,
    pub swapped: bool,
    pub zero: libc::c_int,
    pub crossing_ticker: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecode_def {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub bits: libc::c_int,
    pub resolution: libc::c_int,
    pub flags: libc::c_int,
    pub seed: bits_t,
    pub taps: bits_t,
    pub length: libc::c_uint,
    pub safe: libc::c_uint,
    pub lookup: bool,
    pub lut: lut,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lut {
    pub slot: *mut slot,
    pub table: *mut slot_no_t,
    pub avail: slot_no_t,
}
pub type slot_no_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slot {
    pub timecode: libc::c_uint,
    pub next: slot_no_t,
}
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free
 * Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct track {
    pub tracks: list,
    pub refcount: libc::c_uint,
    pub rate: libc::c_int,
    pub importer: *const libc::c_char,
    pub path: *const libc::c_char,
    pub bytes: size_t,
    pub length: libc::c_uint,
    pub blocks: libc::c_uint,
    pub block: [*mut track_block; 64],
    pub rig: list,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub pe: *mut pollfd,
    pub terminated: bool,
    pub ppm: libc::c_ushort,
    pub overview: libc::c_uint,
}
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free
 * Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */
/*
 * Double-linked lists
 */
/* offsetof() */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub prev: *mut list,
    pub next: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct track_block {
    pub pcm: [libc::c_short; 4194304],
    pub ppm: [libc::c_uchar; 32768],
    pub overview: [libc::c_uchar; 1024],
}
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free
 * Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */
/*
 * Spinlock routines for synchronising with the realtime thread
 */
pub type spin = pthread_spinlock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device_ops {
    pub pollfds: Option<unsafe extern "C" fn(_: *mut device, _: *mut pollfd,
                                             _: size_t) -> ssize_t>,
    pub handle: Option<unsafe extern "C" fn(_: *mut device) -> libc::c_int>,
    pub sample_rate: Option<unsafe extern "C" fn(_: *mut device)
                                -> libc::c_uint>,
    pub start: Option<unsafe extern "C" fn(_: *mut device) -> ()>,
    pub stop: Option<unsafe extern "C" fn(_: *mut device) -> ()>,
    pub clear: Option<unsafe extern "C" fn(_: *mut device) -> ()>,
}
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free
 * Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301, USA.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn device_init(mut dv: *mut device,
                                     mut ops: *mut device_ops) {
    (*dv).fault = 0 as libc::c_int != 0;
    (*dv).ops = ops;
}
/*
 * Clear (destruct) the device. The corresponding constructor is
 * specific to each particular audio system
 */
#[no_mangle]
pub unsafe extern "C" fn device_clear(mut dv: *mut device) {
    if (*(*dv).ops).clear.is_some() {
        (*(*dv).ops).clear.expect("non-null function pointer")(dv);
    };
}
#[no_mangle]
pub unsafe extern "C" fn device_connect_timecoder(mut dv: *mut device,
                                                  mut tc: *mut timecoder) {
    (*dv).timecoder = tc;
}
#[no_mangle]
pub unsafe extern "C" fn device_connect_player(mut dv: *mut device,
                                               mut pl: *mut player) {
    (*dv).player = pl;
}
/*
 * Return: the sample rate of the device in Hz
 */
#[no_mangle]
pub unsafe extern "C" fn device_sample_rate(mut dv: *mut device)
 -> libc::c_uint {
    if (*(*dv).ops).sample_rate.is_some() {
    } else {
        __assert_fail(b"dv->ops->sample_rate != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"device.c\x00" as *const u8 as *const libc::c_char,
                      62 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"unsigned int device_sample_rate(struct device *)\x00")).as_ptr());
    }
    return (*(*dv).ops).sample_rate.expect("non-null function pointer")(dv);
}
/*
 * Start the device inputting and outputting audio
 */
#[no_mangle]
pub unsafe extern "C" fn device_start(mut dv: *mut device) {
    if (*(*dv).ops).start.is_some() {
        (*(*dv).ops).start.expect("non-null function pointer")(dv);
    };
}
/*
 * Stop the device
 */
#[no_mangle]
pub unsafe extern "C" fn device_stop(mut dv: *mut device) {
    if (*(*dv).ops).stop.is_some() {
        (*(*dv).ops).stop.expect("non-null function pointer")(dv);
    };
}
/*
 * Get file descriptors which should be polled for this device
 *
 * Do not return anything for callback-based audio systems. If the
 * return value is > 0, there must be a handle() function available.
 *
 * Return: the number of pollfd filled, or -1 on error
 */
#[no_mangle]
pub unsafe extern "C" fn device_pollfds(mut dv: *mut device,
                                        mut pe: *mut pollfd, mut z: size_t)
 -> ssize_t {
    if (*(*dv).ops).pollfds.is_some() {
        return (*(*dv).ops).pollfds.expect("non-null function pointer")(dv,
                                                                        pe, z)
    } else { return 0 as libc::c_int as ssize_t };
}
/*
 * Handle any available input or output on the device
 *
 * This function can be called when there is activity on any file
 * descriptor, not specifically one returned by this device.
 */
#[no_mangle]
pub unsafe extern "C" fn device_handle(mut dv: *mut device) {
    if (*dv).fault { return }
    if (*(*dv).ops).handle.is_none() { return }
    if (*(*dv).ops).handle.expect("non-null function pointer")(dv) !=
           0 as libc::c_int {
        (*dv).fault = 1 as libc::c_int != 0;
        fputs(b"Error handling audio device; disabling it\n\x00" as *const u8
                  as *const libc::c_char, stderr);
    };
}
/*
 * Send audio from a device for processing
 *
 * Pre: buffer pcm contains n stereo samples
 */
#[no_mangle]
pub unsafe extern "C" fn device_submit(mut dv: *mut device,
                                       mut pcm: *mut libc::c_short,
                                       mut n: size_t) {
    if !(*dv).timecoder.is_null() {
    } else {
        __assert_fail(b"dv->timecoder != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"device.c\x00" as *const u8 as *const libc::c_char,
                      132 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"void device_submit(struct device *, short *, size_t)\x00")).as_ptr());
    }
    timecoder_submit((*dv).timecoder, pcm, n);
}
/*
 * Collect audio from the processing to send to a device
 *
 * Post: buffer pcm is filled with n stereo samples
 */
#[no_mangle]
pub unsafe extern "C" fn device_collect(mut dv: *mut device,
                                        mut pcm: *mut libc::c_short,
                                        mut n: size_t) {
    if !(*dv).player.is_null() {
    } else {
        __assert_fail(b"dv->player != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"device.c\x00" as *const u8 as *const libc::c_char,
                      144 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"void device_collect(struct device *, short *, size_t)\x00")).as_ptr());
    }
    player_collect((*dv).player, pcm, n as libc::c_uint);
}
