use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn cues_reset(q: *mut cues);
    #[no_mangle]
    fn cues_unset(q: *mut cues, label: libc::c_uint);
    #[no_mangle]
    fn cues_set(q: *mut cues, label: libc::c_uint, position: libc::c_double);
    #[no_mangle]
    fn cues_get(q: *const cues, label: libc::c_uint) -> libc::c_double;
    #[no_mangle]
    fn device_clear(dv: *mut device);
    #[no_mangle]
    fn device_connect_timecoder(dv: *mut device, tc: *mut timecoder);
    #[no_mangle]
    fn device_connect_player(dv: *mut device, pl: *mut player);
    #[no_mangle]
    fn device_sample_rate(dv: *mut device) -> libc::c_uint;
    #[no_mangle]
    fn rt_add_device(rt: *mut rt, dv: *mut device) -> libc::c_int;
    /* Tracks are dynamically allocated and reference counted */
    #[no_mangle]
    fn track_acquire_by_import(importer: *const libc::c_char,
                               path: *const libc::c_char) -> *mut track;
    #[no_mangle]
    fn track_acquire_empty() -> *mut track;
    /* re-sync offset at next opportunity */
    #[no_mangle]
    fn player_init(pl: *mut player, sample_rate: libc::c_uint,
                   track: *mut track, timecoder: *mut timecoder);
    #[no_mangle]
    fn player_clear(pl: *mut player);
    #[no_mangle]
    fn player_set_track(pl: *mut player, track: *mut track);
    #[no_mangle]
    fn player_clone(pl: *mut player, from: *const player);
    #[no_mangle]
    fn player_get_elapsed(pl: *mut player) -> libc::c_double;
    #[no_mangle]
    fn player_is_active(pl: *const player) -> bool;
    #[no_mangle]
    fn player_seek_to(pl: *mut player, seconds: libc::c_double);
    #[no_mangle]
    fn player_recue(pl: *mut player);
    #[no_mangle]
    fn timecoder_init(tc: *mut timecoder, def: *mut timecode_def,
                      speed: libc::c_double, sample_rate: libc::c_uint,
                      phono: bool);
    #[no_mangle]
    fn timecoder_clear(tc: *mut timecoder);
    #[no_mangle]
    fn status_printf(level: libc::c_int, s: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct controller {
    pub fault: bool,
    pub local: *mut libc::c_void,
    pub ops: *mut controller_ops,
}
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
 * A set of cue points
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cues {
    pub position: [libc::c_double; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecoder_channel {
    pub positive: bool,
    pub swapped: bool,
    pub zero: libc::c_int,
    pub crossing_ticker: libc::c_uint,
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
 * An empty record, is used briefly until a record is loaded
 * to a deck
 */
static mut no_record: record =
    {
        let mut init =
            record{pathname: 0 as *const libc::c_char as *mut libc::c_char,
                   artist:
                       b"\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   title:
                       b"\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   match_0: 0 as *const libc::c_char as *mut libc::c_char,
                   bpm: 0.,};
        init
    };
/*
 * Initialise a deck
 *
 * A deck is a logical grouping of the various components which
 * reflects the user's view on a deck in the system.
 *
 * Pre: deck->device is valid
 */
#[no_mangle]
pub unsafe extern "C" fn deck_init(mut d: *mut deck, mut rt: *mut rt,
                                   mut timecode: *mut timecode_def,
                                   mut importer: *const libc::c_char,
                                   mut speed: libc::c_double, mut phono: bool,
                                   mut protect: bool) -> libc::c_int {
    let mut rate: libc::c_uint = 0;
    if rt_add_device(rt, &mut (*d).device) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    (*d).ncontrol = 0 as libc::c_int as size_t;
    (*d).record = &no_record;
    (*d).punch = ::std::f64::INFINITY;
    (*d).protect = protect;
    if !importer.is_null() {
    } else {
        __assert_fail(b"importer != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"deck.c\x00" as *const u8 as *const libc::c_char,
                      60 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 101],
                                                &[libc::c_char; 101]>(b"int deck_init(struct deck *, struct rt *, struct timecode_def *, const char *, double, _Bool, _Bool)\x00")).as_ptr());
    }
    (*d).importer = importer;
    rate = device_sample_rate(&mut (*d).device);
    if !timecode.is_null() {
    } else {
        __assert_fail(b"timecode != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"deck.c\x00" as *const u8 as *const libc::c_char,
                      63 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 101],
                                                &[libc::c_char; 101]>(b"int deck_init(struct deck *, struct rt *, struct timecode_def *, const char *, double, _Bool, _Bool)\x00")).as_ptr());
    }
    timecoder_init(&mut (*d).timecoder, timecode, speed, rate, phono);
    player_init(&mut (*d).player, rate, track_acquire_empty(),
                &mut (*d).timecoder);
    cues_reset(&mut (*d).cues);
    /* The timecoder and player are driven by requests from
     * the audio device */
    device_connect_timecoder(&mut (*d).device, &mut (*d).timecoder);
    device_connect_player(&mut (*d).device, &mut (*d).player);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn deck_clear(mut d: *mut deck) {
    /* FIXME: remove from rig and rt */
    player_clear(&mut (*d).player);
    timecoder_clear(&mut (*d).timecoder);
    device_clear(&mut (*d).device);
}
#[no_mangle]
pub unsafe extern "C" fn deck_is_locked(mut d: *const deck) -> bool {
    return (*d).protect as libc::c_int != 0 &&
               player_is_active(&(*d).player) as libc::c_int != 0;
}
/*
 * Load a record from the library to a deck
 */
#[no_mangle]
pub unsafe extern "C" fn deck_load(mut d: *mut deck,
                                   mut record: *mut record) {
    let mut t: *mut track = 0 as *mut track;
    if deck_is_locked(d) {
        status_printf(2 as libc::c_int,
                      b"Stop deck to load a different track\x00" as *const u8
                          as *const libc::c_char);
        return
    }
    t = track_acquire_by_import((*d).importer, (*record).pathname);
    if t.is_null() { return }
    (*d).record = record;
    player_set_track(&mut (*d).player, t);
    /* passes reference */
}
#[no_mangle]
pub unsafe extern "C" fn deck_recue(mut d: *mut deck) {
    if deck_is_locked(d) {
        status_printf(2 as libc::c_int,
                      b"Stop deck to recue\x00" as *const u8 as
                          *const libc::c_char);
        return
    }
    player_recue(&mut (*d).player);
}
#[no_mangle]
pub unsafe extern "C" fn deck_clone(mut d: *mut deck, mut from: *const deck) {
    (*d).record = (*from).record;
    player_clone(&mut (*d).player, &(*from).player);
}
/*
 * Clear the cue point, ready to be set again
 */
#[no_mangle]
pub unsafe extern "C" fn deck_unset_cue(mut d: *mut deck,
                                        mut label: libc::c_uint) {
    cues_unset(&mut (*d).cues, label);
}
/*
 * Seek the current playback position to a cue point position,
 * or set the cue point if unset
 */
#[no_mangle]
pub unsafe extern "C" fn deck_cue(mut d: *mut deck, mut label: libc::c_uint) {
    let mut p: libc::c_double = 0.;
    p = cues_get(&mut (*d).cues, label);
    if p == ::std::f64::INFINITY {
        cues_set(&mut (*d).cues, label, player_get_elapsed(&mut (*d).player));
    } else { player_seek_to(&mut (*d).player, p); };
}
/*
 * Seek to a cue point ready to return from it later. Overrides an
 * existing punch operation.
 */
#[no_mangle]
pub unsafe extern "C" fn deck_punch_in(mut d: *mut deck,
                                       mut label: libc::c_uint) {
    let mut p: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    e = player_get_elapsed(&mut (*d).player);
    p = cues_get(&mut (*d).cues, label);
    if p == ::std::f64::INFINITY {
        cues_set(&mut (*d).cues, label, e);
        return
    }
    if (*d).punch != ::std::f64::INFINITY { e -= (*d).punch }
    player_seek_to(&mut (*d).player, p);
    (*d).punch = p - e;
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
/* Punch */
/* A controller adds itself here */
/*
 * Return from a cue point
 */
#[no_mangle]
pub unsafe extern "C" fn deck_punch_out(mut d: *mut deck) {
    let mut e: libc::c_double = 0.;
    if (*d).punch == ::std::f64::INFINITY { return }
    e = player_get_elapsed(&mut (*d).player);
    player_seek_to(&mut (*d).player, e - (*d).punch);
    (*d).punch = ::std::f64::INFINITY;
}
