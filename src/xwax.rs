#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value, main,
           register_tool)]

       
       
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn mlockall(__flags: libc::c_int) -> libc::c_int;
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
    fn alsa_init(dv: *mut device, name: *const libc::c_char,
                 rate: libc::c_int, buffer_time: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn alsa_clear_config_cache();
    #[no_mangle]
    fn controller_clear(c: *mut controller);
    #[no_mangle]
    fn controller_add_deck(c: *mut controller, d: *mut deck);
    #[no_mangle]
    fn dicer_init(c: *mut controller, rt_0: *mut rt, hw: *const libc::c_char)
     -> libc::c_int;
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
    fn dummy_init(d: *mut device);
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
    fn interface_stop();
    #[no_mangle]
    fn interface_start(lib: *mut library, geo: *const libc::c_char,
                       decor: bool) -> libc::c_int;
    #[no_mangle]
    fn library_import(lib: *mut library, scan: *const libc::c_char,
                      path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn library_clear(li: *mut library);
    #[no_mangle]
    fn library_init(li: *mut library) -> libc::c_int;
    #[no_mangle]
    fn library_global_clear();
    #[no_mangle]
    fn library_global_init() -> libc::c_int;
    #[no_mangle]
    fn rt_init(rt_0: *mut rt);
    #[no_mangle]
    fn rt_clear(rt_0: *mut rt);
    #[no_mangle]
    fn rt_start(rt_0: *mut rt, priority: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn rt_stop(rt_0: *mut rt);
    #[no_mangle]
    fn track_use_mlock();
    #[no_mangle]
    fn timecoder_find_definition(name: *const libc::c_char)
     -> *mut timecode_def;
    #[no_mangle]
    fn timecoder_free_lookup();
    #[no_mangle]
    fn deck_init(deck_0: *mut deck, rt_0: *mut rt,
                 timecode_0: *mut timecode_def,
                 importer_0: *const libc::c_char, speed_0: libc::c_double,
                 phono_0: bool, protect_0: bool) -> libc::c_int;
    #[no_mangle]
    fn deck_clear(deck_0: *mut deck);
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
 * General helper functions for threads
 */
    #[no_mangle]
    fn thread_global_init() -> libc::c_int;
    #[no_mangle]
    fn thread_global_clear();
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
    fn rig_init() -> libc::c_int;
    #[no_mangle]
    fn rig_clear();
    #[no_mangle]
    fn rig_main() -> libc::c_int;
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
pub type pid_t = __pid_t;
pub type pthread_t = libc::c_ulong;
pub type pthread_spinlock_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
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
pub struct device {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut device_ops,
    pub timecoder: *mut timecoder,
    pub player: *mut player,
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
pub type bits_t = libc::c_uint;
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
/* Values for the filter concluded experimentally */
/* State of the pitch calculation filter */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pitch {
    pub dt: libc::c_double,
    pub x: libc::c_double,
    pub v: libc::c_double,
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
/* number of bits in string */
/* wave cycles per second */
/* LFSR value at timecode zero */
/* central LFSR taps, excluding end taps */
/* in cycles */
/* last 'safe' timecode number (for auto disconnect) */
/* true if lut has been generated */
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
/* next slot with the same hash */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deck {
    pub device: device,
    pub timecoder: timecoder,
    pub importer: *const libc::c_char,
    pub protect: bool,
    pub player: player,
    pub record: *const record,
    pub cues: cues,
    pub punch: libc::c_double,
    pub ncontrol: size_t,
    pub control: [*mut controller; 4],
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
 * Base state of a 'controller', which is a MIDI controller or HID
 * device used to control the program
 */
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
pub struct controller {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut controller_ops,
}
/*
 * Functions which must be implemented for a controller
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct controller_ops {
    pub add_deck: Option<unsafe extern "C" fn(_: *mut controller,
                                              _: *mut deck) -> libc::c_int>,
    pub pollfds: Option<unsafe extern "C" fn(_: *mut controller,
                                             _: *mut pollfd, _: size_t)
                            -> ssize_t>,
    pub realtime: Option<unsafe extern "C" fn(_: *mut controller)
                             -> libc::c_int>,
    pub clear: Option<unsafe extern "C" fn(_: *mut controller) -> ()>,
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
 * A set of cue points
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cues {
    pub position: [libc::c_double; 16],
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
/* A single music track in our listings */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct record {
    pub pathname: *mut libc::c_char,
    pub artist: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub match_0: *mut libc::c_char,
    pub bpm: libc::c_double,
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
 * State data for the realtime thread, maintained during rt_start and
 * rt_stop
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rt {
    pub ph: pthread_t,
    pub sem: sem_t,
    pub finished: bool,
    pub priority: libc::c_int,
    pub ndv: size_t,
    pub dv: [*mut device; 3],
    pub nctl: size_t,
    pub ctl: [*mut controller; 3],
    pub npt: size_t,
    pub pt: [pollfd; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_longlong,
}
/* or 0.0 if not known */
/* Index points to records, but does not manage those pointers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index {
    pub record: *mut *mut record,
    pub size: size_t,
    pub entries: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub observers: list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observer {
    pub event: list,
    pub func: Option<unsafe extern "C" fn(_: *mut observer,
                                          _: *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listing {
    pub by_artist: index,
    pub by_bpm: index,
    pub by_order: index,
    pub addition: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crate_0 {
    pub is_fixed: bool,
    pub is_busy: bool,
    pub name: *mut libc::c_char,
    pub listing: *mut listing,
    pub on_addition: observer,
    pub on_completion: observer,
    pub activity: event,
    pub refresh: event,
    pub addition: event,
    pub scan: *const libc::c_char,
    pub path: *const libc::c_char,
    pub excrate: *mut excrate,
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
pub struct excrate {
    pub excrates: list,
    pub refcount: libc::c_uint,
    pub search: *const libc::c_char,
    pub listing: listing,
    pub storage: *mut listing,
    pub completion: event,
    pub rig: list,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub pe: *mut pollfd,
    pub terminated: bool,
    pub rb: rb,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb {
    pub buf: [libc::c_char; 4096],
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct library {
    pub storage: listing,
    pub all: crate_0,
    pub crate_0: *mut *mut crate_0,
    pub crates: size_t,
}
#[no_mangle]
pub static mut banner: *mut libc::c_char =
    b"xwax 1.6-beta2-31-g3f97662 (C) Copyright 2018 Mark Hills <mark@xwax.org>\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ndeck: size_t = 0;
#[no_mangle]
pub static mut deck: [deck; 3] =
    [deck{device:
              device{fault: false,
                     local: 0 as *const libc::c_void as *mut libc::c_void,
                     ops: 0 as *const device_ops as *mut device_ops,
                     timecoder: 0 as *const timecoder as *mut timecoder,
                     player: 0 as *const player as *mut player,},
          timecoder:
              timecoder{def: 0 as *const timecode_def as *mut timecode_def,
                        speed: 0.,
                        dt: 0.,
                        zero_alpha: 0.,
                        threshold: 0,
                        forwards: false,
                        primary:
                            timecoder_channel{positive: false,
                                              swapped: false,
                                              zero: 0,
                                              crossing_ticker: 0,},
                        secondary:
                            timecoder_channel{positive: false,
                                              swapped: false,
                                              zero: 0,
                                              crossing_ticker: 0,},
                        pitch: pitch{dt: 0., x: 0., v: 0.,},
                        ref_level: 0,
                        bitstream: 0,
                        timecode: 0,
                        valid_counter: 0,
                        timecode_ticker: 0,
                        mon: 0 as *const libc::c_uchar as *mut libc::c_uchar,
                        mon_size: 0,
                        mon_counter: 0,},
          importer: 0 as *const libc::c_char,
          protect: false,
          player:
              player{sample_dt: 0.,
                     lock: 0,
                     track: 0 as *const track as *mut track,
                     position: 0.,
                     target_position: 0.,
                     offset: 0.,
                     last_difference: 0.,
                     pitch: 0.,
                     sync_pitch: 0.,
                     volume: 0.,
                     timecoder: 0 as *const timecoder as *mut timecoder,
                     timecode_control: false,
                     recalibrate: false,},
          record: 0 as *const record,
          cues: cues{position: [0.; 16],},
          punch: 0.,
          ncontrol: 0,
          control: [0 as *const controller as *mut controller; 4],}; 3];
static mut nctl: size_t = 0;
static mut ctl: [controller; 2] =
    [controller{fault: false,
                local: 0 as *const libc::c_void as *mut libc::c_void,
                ops: 0 as *const controller_ops as *mut controller_ops,}; 2];
static mut rt: rt =
    rt{ph: 0,
       sem: sem_t{__size: [0; 32],},
       finished: false,
       priority: 0,
       ndv: 0,
       dv: [0 as *const device as *mut device; 3],
       nctl: 0,
       ctl: [0 as *const controller as *mut controller; 3],
       npt: 0,
       pt: [pollfd{fd: 0, events: 0, revents: 0,}; 32],};
static mut speed: libc::c_double = 0.;
static mut protect: bool = false;
static mut phono: bool = false;
static mut importer: *const libc::c_char = 0 as *const libc::c_char;
static mut timecode: *mut timecode_def =
    0 as *const timecode_def as *mut timecode_def;
unsafe extern "C" fn usage(mut fd: *mut FILE) {
    fprintf(fd,
            b"Usage: xwax [<options>]\n\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fd,
            b"Program-wide options:\n  -k             Lock real-time memory into RAM\n  -q <n>         Real-time priority (0 for no priority, default %d)\n  -g <s>         Set display geometry (see man page)\n  --no-decor     Request a window with no decorations\n  -h             Display this message to stdout and exit\n\n\x00"
                as *const u8 as *const libc::c_char, 80 as libc::c_int);
    fprintf(fd,
            b"Music library options:\n  -l <path>      Location to scan for audio tracks\n  -s <program>   Library scanner (default \'%s\')\n\n\x00"
                as *const u8 as *const libc::c_char,
            b"/home/pi/libexec/xwax-scan\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fd,
            b"Deck options:\n  -t <name>      Timecode name\n  -33            Use timecode at 33.3RPM (default)\n  -45            Use timecode at 45RPM\n  -c             Protect against certain operations while playing\n  -u             Allow all operations when playing\n  --line         Line level signal (default)\n  --phono        Tolerate cartridge level signal (\'software pre-amp\')\n  -i <program>   Importer (default \'%s\')\n  --dummy        Build a dummy deck with no audio device\n\n\x00"
                as *const u8 as *const libc::c_char,
            b"/home/pi/libexec/xwax-import\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fd,
            b"ALSA device options:\n  -a <device>    Build a deck connected to ALSA audio device\n  -r <hz>        Sample rate (default %dHz)\n  -m <ms>        Buffer time (default %dms)\n\n\x00"
                as *const u8 as *const libc::c_char, 44100 as libc::c_int,
            8 as libc::c_int);
    fprintf(fd,
            b"MIDI control:\n  --dicer <dev>   Novation Dicer\n\n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(fd,
            b"The ordering of options is important. Options apply to subsequent\nmusic libraries or decks, which can be given multiple times. See the\nmanual for details.\n\nAvailable timecodes (for use with -t):\n  serato_2a (default), serato_2b, serato_cd,\n  traktor_a, traktor_b, mixvibes_v2, mixvibes_7inch\n\nSee the xwax(1) man page for full information and examples.\n\x00"
                as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn start_deck(mut desc: *const libc::c_char)
 -> *mut device {
    fprintf(stderr,
            b"Initialising deck %zd (%s)...\n\x00" as *const u8 as
                *const libc::c_char, ndeck, desc);
    if ndeck ==
           (::std::mem::size_of::<[deck; 3]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<deck>() as
                                                libc::c_ulong) {
        fprintf(stderr,
                b"Too many decks.\n\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut device
    }
    return &mut (*deck.as_mut_ptr().offset(ndeck as isize)).device;
}
unsafe extern "C" fn commit_deck() -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut d: *mut deck = 0 as *mut deck;
    let mut n: size_t = 0;
    /* Fallback to a default timecode. Don't initialise this at the
     * front of the program to avoid buildling unnecessary LUTs */
    if timecode.is_null() {
        timecode =
            timecoder_find_definition(b"serato_2a\x00" as *const u8 as
                                          *const libc::c_char);
        if !timecode.is_null() {
        } else {
            __assert_fail(b"timecode != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"xwax.c\x00" as *const u8 as *const libc::c_char,
                          165 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 22],
                                                    &[libc::c_char; 22]>(b"int commit_deck(void)\x00")).as_ptr());
        }
    }
    d = &mut *deck.as_mut_ptr().offset(ndeck as isize) as *mut deck;
    r = deck_init(d, &mut rt, timecode, importer, speed, phono, protect);
    if r == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    /* Connect this deck to available controllers */
    n = 0 as libc::c_int as size_t;
    while n < nctl {
        controller_add_deck(&mut *ctl.as_mut_ptr().offset(n as isize), d);
        n = n.wrapping_add(1)
    }
    ndeck = ndeck.wrapping_add(1);
    return 0 as libc::c_int;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut n: libc::c_int = 0;
    let mut priority: libc::c_int = 0;
    let mut scanner: *const libc::c_char = 0 as *const libc::c_char;
    let mut geo: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut use_mlock: bool = false;
    let mut decor: bool = false;
    let mut library: library =
        library{storage:
                    listing{by_artist:
                                index{record: 0 as *mut *mut record,
                                      size: 0,
                                      entries: 0,},
                            by_bpm:
                                index{record: 0 as *mut *mut record,
                                      size: 0,
                                      entries: 0,},
                            by_order:
                                index{record: 0 as *mut *mut record,
                                      size: 0,
                                      entries: 0,},
                            addition:
                                event{observers:
                                          list{prev: 0 as *mut list,
                                               next: 0 as *mut list,},},},
                all:
                    crate_0{is_fixed: false,
                            is_busy: false,
                            name: 0 as *mut libc::c_char,
                            listing: 0 as *mut listing,
                            on_addition:
                                observer{event:
                                             list{prev: 0 as *mut list,
                                                  next: 0 as *mut list,},
                                         func: None,},
                            on_completion:
                                observer{event:
                                             list{prev: 0 as *mut list,
                                                  next: 0 as *mut list,},
                                         func: None,},
                            activity:
                                event{observers:
                                          list{prev: 0 as *mut list,
                                               next: 0 as *mut list,},},
                            refresh:
                                event{observers:
                                          list{prev: 0 as *mut list,
                                               next: 0 as *mut list,},},
                            addition:
                                event{observers:
                                          list{prev: 0 as *mut list,
                                               next: 0 as *mut list,},},
                            scan: 0 as *const libc::c_char,
                            path: 0 as *const libc::c_char,
                            excrate: 0 as *mut excrate,},
                crate_0: 0 as *mut *mut crate_0,
                crates: 0,};
    let mut rate: libc::c_int = 0;
    let mut alsa_buffer: libc::c_int = 0;
    fprintf(stderr,
            b"%s\n\nThis software is supplied WITHOUT ANY WARRANTY; without even the implied\nwarranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. This is\nfree software, and you are welcome to redistribute it under certain\nconditions; see the file COPYING for details.\n\n\x00"
                as *const u8 as *const libc::c_char, banner);
    if setlocale(6 as libc::c_int,
                 b"\x00" as *const u8 as *const libc::c_char).is_null() {
        fprintf(stderr,
                b"Could not honour the local encoding\n\x00" as *const u8 as
                    *const libc::c_char);
        return -(1 as libc::c_int)
    }
    /* Explicit formatting for numbers; parsing and printing.  Match
     * the user's expectations, and the documentation */
    if setlocale(1 as libc::c_int,
                 b"POSIX\x00" as *const u8 as *const libc::c_char).is_null() {
        fprintf(stderr,
                b"Could not set numeric encoding\n\x00" as *const u8 as
                    *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if thread_global_init() == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    if library_global_init() == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    if rig_init() == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    rt_init(&mut rt);
    library_init(&mut library);
    ndeck = 0 as libc::c_int as size_t;
    geo = b"\x00" as *const u8 as *const libc::c_char;
    decor = 1 as libc::c_int != 0;
    nctl = 0 as libc::c_int as size_t;
    priority = 80 as libc::c_int;
    importer =
        b"/home/pi/libexec/xwax-import\x00" as *const u8 as
            *const libc::c_char;
    scanner =
        b"/home/pi/libexec/xwax-scan\x00" as *const u8 as *const libc::c_char;
    timecode = 0 as *mut timecode_def;
    speed = 1.0f64;
    protect = 0 as libc::c_int != 0;
    phono = 0 as libc::c_int != 0;
    use_mlock = 0 as libc::c_int != 0;
    rate = 44100 as libc::c_int;
    alsa_buffer = 8 as libc::c_int;
    /* Skip over command name */
    argv = argv.offset(1);
    argc -= 1;
    while argc > 0 as libc::c_int {
        if strcmp(*argv.offset(0 as libc::c_int as isize),
                  b"-h\x00" as *const u8 as *const libc::c_char) == 0 {
            usage(stdout);
            return 0 as libc::c_int
        } else {
            if strcmp(*argv.offset(0 as libc::c_int as isize),
                      b"-r\x00" as *const u8 as *const libc::c_char) == 0 {
                /* Set sample rate for subsequence devices */
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-r requires an integer argument.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                rate =
                    strtol(*argv.offset(1 as libc::c_int as isize),
                           &mut endptr, 10 as libc::c_int) as libc::c_int;
                if *endptr as libc::c_int != '\u{0}' as i32 {
                    fprintf(stderr,
                            b"-r requires an integer argument.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-m\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                /* Set size of ALSA buffer for subsequence devices */
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-m requires an integer argument.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                alsa_buffer =
                    strtol(*argv.offset(1 as libc::c_int as isize),
                           &mut endptr, 10 as libc::c_int) as libc::c_int;
                if *endptr as libc::c_int != '\u{0}' as i32 {
                    fprintf(stderr,
                            b"-m requires an integer argument.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-d\x00" as *const u8 as *const libc::c_char) ==
                          0 ||
                          strcmp(*argv.offset(0 as libc::c_int as isize),
                                 b"-a\x00" as *const u8 as
                                     *const libc::c_char) == 0 ||
                          strcmp(*argv.offset(0 as libc::c_int as isize),
                                 b"-j\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                let mut r: libc::c_int = 0;
                let mut device: *mut device = 0 as *mut device;
                /* Create a deck */
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-%c requires a device name as an argument.\n\x00"
                                as *const u8 as *const libc::c_char,
                            *(*argv.offset(0 as libc::c_int as
                                               isize)).offset(1 as libc::c_int
                                                                  as isize) as
                                libc::c_int);
                    return -(1 as libc::c_int)
                }
                device = start_deck(*argv.offset(1 as libc::c_int as isize));
                if device.is_null() { return -(1 as libc::c_int) }
                /* Work out which device type we are using, and initialise
             * an appropriate device. */
                match *(*argv.offset(0 as libc::c_int as
                                         isize)).offset(1 as libc::c_int as
                                                            isize) as
                          libc::c_int {
                    97 => {
                        r =
                            alsa_init(device,
                                      *argv.offset(1 as libc::c_int as isize),
                                      rate, alsa_buffer)
                    }
                    _ => {
                        fprintf(stderr,
                                b"Device type is not supported by this distribution of xwax.\n\x00"
                                    as *const u8 as *const libc::c_char);
                        return -(1 as libc::c_int)
                    }
                }
                if r == -(1 as libc::c_int) { return -(1 as libc::c_int) }
                commit_deck();
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"--dummy\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                let mut v: *mut device = 0 as *mut device;
                v =
                    start_deck(b"dummy\x00" as *const u8 as
                                   *const libc::c_char);
                if v.is_null() { return -(1 as libc::c_int) }
                dummy_init(v);
                commit_deck();
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-t\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                /* Set the timecode definition to use */
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-t requires a name as an argument.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                timecode =
                    timecoder_find_definition(*argv.offset(1 as libc::c_int as
                                                               isize));
                if timecode.is_null() {
                    fprintf(stderr,
                            b"Timecode \'%s\' is not known.\n\x00" as
                                *const u8 as *const libc::c_char,
                            *argv.offset(1 as libc::c_int as isize));
                    return -(1 as libc::c_int)
                }
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-33\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                speed = 1.0f64;
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-45\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                speed = 1.35f64;
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-c\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                protect = 1 as libc::c_int != 0;
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-u\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                protect = 0 as libc::c_int != 0;
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"--line\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                phono = 0 as libc::c_int != 0;
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"--phono\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                phono = 1 as libc::c_int != 0;
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-k\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                use_mlock = 1 as libc::c_int != 0;
                track_use_mlock();
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-q\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-q requires an integer argument.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                priority =
                    strtol(*argv.offset(1 as libc::c_int as isize),
                           &mut endptr, 10 as libc::c_int) as libc::c_int;
                if *endptr as libc::c_int != '\u{0}' as i32 {
                    fprintf(stderr,
                            b"-q requires an integer argument.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                if priority < 0 as libc::c_int {
                    fprintf(stderr,
                            b"Priority (%d) must be zero or positive.\n\x00"
                                as *const u8 as *const libc::c_char,
                            priority);
                    return -(1 as libc::c_int)
                }
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-g\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-g requires an argument.\n\x00" as *const u8 as
                                *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                geo = *argv.offset(1 as libc::c_int as isize);
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"--no-decor\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                decor = 0 as libc::c_int != 0;
                argv = argv.offset(1);
                argc -= 1
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-i\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                /* Importer script for subsequent decks */
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-i requires an executable path as an argument.\n\x00"
                                as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                importer = *argv.offset(1 as libc::c_int as isize);
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-s\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                /* Scan script for subsequent libraries */
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-s requires an executable path as an argument.\n\x00"
                                as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                scanner = *argv.offset(1 as libc::c_int as isize);
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"-l\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                /* Load in a music library */
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"-l requires a pathname as an argument.\n\x00" as
                                *const u8 as
                                *const libc::c_char); /* until clean exit */
                    return -(1 as libc::c_int)
                }
                if library_import(&mut library, scanner,
                                  *argv.offset(1 as libc::c_int as isize)) ==
                       -(1 as libc::c_int) {
                    return -(1 as libc::c_int)
                }
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else if strcmp(*argv.offset(0 as libc::c_int as isize),
                             b"--dicer\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                let mut c: *mut controller = 0 as *mut controller;
                if nctl ==
                       ::std::mem::size_of::<[controller; 2]>() as
                           libc::c_ulong {
                    fprintf(stderr,
                            b"Too many controllers; aborting.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                c =
                    &mut *ctl.as_mut_ptr().offset(nctl as isize) as
                        *mut controller;
                if argc < 2 as libc::c_int {
                    fprintf(stderr,
                            b"Dicer requires an ALSA device name.\n\x00" as
                                *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
                if dicer_init(c, &mut rt,
                              *argv.offset(1 as libc::c_int as isize)) ==
                       -(1 as libc::c_int) {
                    return -(1 as libc::c_int)
                }
                nctl = nctl.wrapping_add(1);
                argv = argv.offset(2 as libc::c_int as isize);
                argc -= 2 as libc::c_int
            } else {
                fprintf(stderr,
                        b"\'%s\' argument is unknown; try -h.\n\x00" as
                            *const u8 as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize));
                return -(1 as libc::c_int)
            }
        }
    }
    alsa_clear_config_cache();
    if ndeck == 0 as libc::c_int as libc::c_ulong {
        fprintf(stderr,
                b"You need to give at least one audio device to use as a deck; try -h.\n\x00"
                    as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    rc = 1 as libc::c_int;
    /* Order is important: launch realtime thread first, then mlock.
     * Don't mlock the interface, use sparingly for audio threads */
    if rt_start(&mut rt, priority) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    if use_mlock as libc::c_int != 0 &&
           mlockall(1 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"mlockall\x00" as *const u8 as *const libc::c_char);
    } else if !(interface_start(&mut library, geo, decor) ==
                    -(1 as libc::c_int)) {
        if !(rig_main() == -(1 as libc::c_int)) {
            rc = 0 as libc::c_int;
            fprintf(stderr,
                    b"Exiting cleanly...\n\x00" as *const u8 as
                        *const libc::c_char);
        }
        interface_stop();
    }
    rt_stop(&mut rt);
    n = 0 as libc::c_int;
    while (n as libc::c_ulong) < ndeck {
        deck_clear(&mut *deck.as_mut_ptr().offset(n as isize));
        n += 1
    }
    n = 0 as libc::c_int;
    while (n as libc::c_ulong) < nctl {
        controller_clear(&mut *ctl.as_mut_ptr().offset(n as isize));
        n += 1
    }
    timecoder_free_lookup();
    library_clear(&mut library);
    rt_clear(&mut rt);
    rig_clear();
    library_global_clear();
    thread_global_clear();
    if rc == 0 as libc::c_int {
        fprintf(stderr, b"Done.\n\x00" as *const u8 as *const libc::c_char);
    }
    return rc;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
