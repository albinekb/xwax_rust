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
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
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
    #[no_mangle]
    fn rt_add_controller(rt: *mut rt, c: *mut controller) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct controller {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut controller_ops,
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
/* next slot with the same hash */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lut {
    pub slot: *mut slot,
    pub table: *mut slot_no_t,
    pub avail: slot_no_t,
}
pub type slot_no_t = libc::c_uint;
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
pub struct slot {
    pub timecode: libc::c_uint,
    pub next: slot_no_t,
}
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
pub type FILE = _IO_FILE;
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
pub unsafe extern "C" fn controller_init(mut c: *mut controller,
                                         mut ops: *mut controller_ops,
                                         mut local: *mut libc::c_void,
                                         mut rt: *mut rt) -> libc::c_int {
    (*c).fault = 0 as libc::c_int != 0;
    (*c).ops = ops;
    (*c).local = local;
    return rt_add_controller(rt, c);
}
#[no_mangle]
pub unsafe extern "C" fn controller_clear(mut c: *mut controller) {
    (*(*c).ops).clear.expect("non-null function pointer")(c);
}
/*
 * Add a deck to this controller, if possible
 */
#[no_mangle]
pub unsafe extern "C" fn controller_add_deck(mut c: *mut controller,
                                             mut d: *mut deck) {
    if (*(*c).ops).add_deck.expect("non-null function pointer")(c, d) ==
           0 as libc::c_int {
        if (*d).ncontrol <
               (::std::mem::size_of::<[*mut controller; 4]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut controller>()
                                                    as libc::c_ulong) {
        } else {
            __assert_fail(b"d->ncontrol < ARRAY_SIZE(d->control)\x00" as
                              *const u8 as *const libc::c_char,
                          b"controller.c\x00" as *const u8 as
                              *const libc::c_char,
                          57 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 61],
                                                    &[libc::c_char; 61]>(b"void controller_add_deck(struct controller *, struct deck *)\x00")).as_ptr()); /* FIXME: report error */
        }
        let fresh0 = (*d).ncontrol;
        (*d).ncontrol = (*d).ncontrol.wrapping_add(1);
        (*d).control[fresh0 as usize] = c
        /* for callbacks */
    };
}
/*
 * Get file descriptors which should be polled for this controller
 *
 * Important on systems where only callback-based audio devices
 * (eg. JACK) are used. We need to return some descriptors so
 * that the realtime thread runs.
 *
 * Return: the number of pollfd filled, or -1 on error
 */
#[no_mangle]
pub unsafe extern "C" fn controller_pollfds(mut c: *mut controller,
                                            mut pe: *mut pollfd,
                                            mut z: size_t) -> ssize_t {
    if (*(*c).ops).pollfds.is_some() {
        return (*(*c).ops).pollfds.expect("non-null function pointer")(c, pe,
                                                                       z)
    } else { return 0 as libc::c_int as ssize_t };
}
#[no_mangle]
pub unsafe extern "C" fn controller_handle(mut c: *mut controller) {
    if (*c).fault { return }
    if (*(*c).ops).realtime.expect("non-null function pointer")(c) !=
           0 as libc::c_int {
        (*c).fault = 1 as libc::c_int != 0;
        fputs(b"Error handling hardware controller; disabling it\n\x00" as
                  *const u8 as *const libc::c_char, stderr);
    };
}
