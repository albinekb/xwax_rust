use ::libc;
extern "C" {
    pub type _snd_rawmidi;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn controller_init(c: *mut controller, t: *mut controller_ops,
                       local: *mut libc::c_void, rt: *mut rt) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
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
    #[no_mangle]
    fn deck_punch_out(d: *mut deck);
    #[no_mangle]
    fn deck_punch_in(d: *mut deck, label: libc::c_uint);
    #[no_mangle]
    fn deck_cue(deck: *mut deck, label: libc::c_uint);
    #[no_mangle]
    fn deck_unset_cue(deck: *mut deck, label: libc::c_uint);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
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
 * State information for an open, non-blocking MIDI device
 */
    #[no_mangle]
    fn midi_write(m: *mut midi, buf: *const libc::c_void, len: size_t)
     -> ssize_t;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn midi_read(m: *mut midi, buf: *mut libc::c_void, len: size_t)
     -> ssize_t;
    #[no_mangle]
    fn midi_pollfds(m: *mut midi, pe: *mut pollfd, len: size_t) -> ssize_t;
    #[no_mangle]
    fn midi_open(m: *mut midi, name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn midi_close(m: *mut midi);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dicer {
    pub midi: midi,
    pub left: *mut deck,
    pub right: *mut deck,
    pub left_led: [led_t; 5],
    pub right_led: [led_t; 5],
    pub obuf: [libc::c_char; 180],
    pub ofill: size_t,
}
/* LED states */
pub type led_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct midi {
    pub in_0: *mut snd_rawmidi_t,
    pub out: *mut snd_rawmidi_t,
}
pub type snd_rawmidi_t = _snd_rawmidi;
/*
 * Add a deck to the dicer or pair of dicer
 *
 * Return: -1 if the deck could not be added, otherwise zero
 */
unsafe extern "C" fn add_deck(mut c: *mut controller, mut k: *mut deck)
 -> libc::c_int {
    let mut d: *mut dicer = (*c).local as *mut dicer;
    if !(*d).left.is_null() && !(*d).right.is_null() {
        return -(1 as libc::c_int)
    }
    if (*d).left.is_null() { (*d).left = k } else { (*d).right = k }
    return 0 as libc::c_int;
}
/*
 * Write a MIDI command sequence which would bring the given LED
 * up-to-date
 *
 * Return: n, or -1 if not enough buffer space
 * Post: if buffer space is available, n bytes are written to buf
 */
unsafe extern "C" fn led_cmd(mut led: led_t, mut buf: *mut libc::c_char,
                             mut len: size_t, mut right: bool,
                             mut action: libc::c_uchar, mut shift: bool,
                             mut button: libc::c_uchar) -> ssize_t {
    if len < 3 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as ssize_t
    }
    if action as libc::c_int <= 2 as libc::c_int {
    } else {
        __assert_fail(b"action <= ROLL\x00" as *const u8 as
                          *const libc::c_char,
                      b"dicer.c\x00" as *const u8 as *const libc::c_char,
                      111 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"ssize_t led_cmd(led_t, char *, size_t, _Bool, unsigned char, _Bool, unsigned char)\x00")).as_ptr());
    }
    *buf.offset(0 as libc::c_int as isize) =
        ((if right as libc::c_int != 0 {
              0x9d as libc::c_int
          } else { 0x9a as libc::c_int }) + action as libc::c_int) as
            libc::c_char;
    if (button as libc::c_int) < 5 as libc::c_int {
    } else {
        __assert_fail(b"button < NBUTTONS\x00" as *const u8 as
                          *const libc::c_char,
                      b"dicer.c\x00" as *const u8 as *const libc::c_char,
                      114 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"ssize_t led_cmd(led_t, char *, size_t, _Bool, unsigned char, _Bool, unsigned char)\x00")).as_ptr());
    }
    *buf.offset(1 as libc::c_int as isize) =
        ((if shift as libc::c_int != 0 {
              0x41 as libc::c_int
          } else { 0x3c as libc::c_int }) + button as libc::c_int) as
            libc::c_char;
    /* The Dicer allows us to use any colour in any mode. For
     * simplicity, we tie the colour to the mode at this layer */
    match action as libc::c_int {
        0 => {
            *buf.offset(2 as libc::c_int as isize) =
                0 as libc::c_int as libc::c_char
        }
        1 => {
            *buf.offset(2 as libc::c_int as isize) =
                0x70 as libc::c_int as libc::c_char
        }
        2 => {
            *buf.offset(2 as libc::c_int as isize) =
                0x40 as libc::c_int as libc::c_char
        }
        _ => { abort(); }
    }
    if led as libc::c_int & 0x1 as libc::c_int != 0 {
        let ref mut fresh0 = *buf.offset(2 as libc::c_int as isize);
        *fresh0 =
            (*fresh0 as libc::c_int + 0xa as libc::c_int) as libc::c_char
    }
    if led as libc::c_int & 0x2 as libc::c_int != 0 {
        let ref mut fresh1 = *buf.offset(2 as libc::c_int as isize);
        *fresh1 =
            (*fresh1 as libc::c_int + 0x5 as libc::c_int) as libc::c_char
    }
    return 3 as libc::c_int as ssize_t;
}
/*
 * Push control code for a particular output LED
 *
 * Return: n, or -1 if not enough buffer space
 * Post: if buf is large enough, LED is synced and n bytes are written
 */
unsafe extern "C" fn sync_one_led(mut led: *mut led_t,
                                  mut buf: *mut libc::c_char, mut len: size_t,
                                  mut right: bool, mut button: libc::c_uchar)
 -> ssize_t {
    let mut a: libc::c_uint = 0;
    let mut t: size_t = 0;
    if *led as libc::c_int & 0x4 as libc::c_int != 0 {
        return 0 as libc::c_int as ssize_t
    }
    /* For simplicify we light all LEDs in all modes the same:
     * (cue, loop, roll) x (shift, non-shift) */
    t = 0 as libc::c_int as size_t;
    a = 0 as libc::c_int as libc::c_uint;
    while a <= 2 as libc::c_int as libc::c_uint {
        let mut z: ssize_t = 0;
        z =
            led_cmd(*led, buf, len, right, a as libc::c_uchar,
                    0 as libc::c_int != 0, button);
        if z == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t
        }
        buf = buf.offset(z as isize);
        len =
            (len as libc::c_ulong).wrapping_sub(z as libc::c_ulong) as size_t
                as size_t;
        t =
            (t as libc::c_ulong).wrapping_add(z as libc::c_ulong) as size_t as
                size_t;
        z =
            led_cmd(*led, buf, len, right, a as libc::c_uchar,
                    1 as libc::c_int != 0, button);
        if z == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t
        }
        buf = buf.offset(z as isize);
        len =
            (len as libc::c_ulong).wrapping_sub(z as libc::c_ulong) as size_t
                as size_t;
        t =
            (t as libc::c_ulong).wrapping_add(z as libc::c_ulong) as size_t as
                size_t;
        a = a.wrapping_add(1)
    }
    *led = (*led as libc::c_int | 0x4 as libc::c_int) as led_t;
    return t as ssize_t;
}
/*
 * Return: number of bytes written to the buffer
 */
unsafe extern "C" fn sync_one_dicer(mut led: *mut led_t, mut right: bool,
                                    mut buf: *mut libc::c_char,
                                    mut len: size_t) -> size_t {
    let mut n: size_t = 0;
    let mut t: size_t = 0;
    t = 0 as libc::c_int as size_t;
    n = 0 as libc::c_int as size_t;
    while n < 5 as libc::c_int as libc::c_ulong {
        let mut z: ssize_t = 0;
        z =
            sync_one_led(&mut *led.offset(n as isize), buf, len, right,
                         n as libc::c_uchar);
        if z == -(1 as libc::c_int) as libc::c_long { break ; }
        buf = buf.offset(z as isize);
        len =
            (len as libc::c_ulong).wrapping_sub(z as libc::c_ulong) as size_t
                as size_t;
        t =
            (t as libc::c_ulong).wrapping_add(z as libc::c_ulong) as size_t as
                size_t;
        n = n.wrapping_add(1)
    }
    return t;
}
/*
 * Write a MIDI command sequence to sync all hardware LEDs with the
 * state held in memory.
 *
 * The Dicer first appears to only have five output LEDs on two
 * controllers. But there are three modes for each, and then shift
 * on/off modes: total (5 * 2 * 3 * 2) = 60
 */
unsafe extern "C" fn sync_all_leds(mut d: *mut dicer) {
    let mut w: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    buf = (*d).obuf.as_mut_ptr().offset((*d).ofill as isize);
    len =
        (::std::mem::size_of::<[libc::c_char; 180]>() as
             libc::c_ulong).wrapping_sub((*d).ofill);
    /* Top-up the buffer, even if not empty */
    w =
        sync_one_dicer((*d).left_led.as_mut_ptr(), 0 as libc::c_int != 0, buf,
                       len);
    buf = buf.offset(w as isize);
    len = (len as libc::c_ulong).wrapping_sub(w) as size_t as size_t;
    (*d).ofill =
        ((*d).ofill as libc::c_ulong).wrapping_add(w) as size_t as size_t;
    w =
        sync_one_dicer((*d).right_led.as_mut_ptr(), 1 as libc::c_int != 0,
                       buf, len);
    buf = buf.offset(w as isize);
    len = (len as libc::c_ulong).wrapping_sub(w) as size_t as size_t;
    (*d).ofill =
        ((*d).ofill as libc::c_ulong).wrapping_add(w) as size_t as size_t;
    if (*d).ofill > 0 as libc::c_int as libc::c_ulong {
        let mut z: ssize_t = 0;
        z =
            midi_write(&mut (*d).midi,
                       (*d).obuf.as_mut_ptr() as *const libc::c_void,
                       (*d).ofill);
        if z == -(1 as libc::c_int) as libc::c_long { return }
        if (z as libc::c_ulong) < (*d).ofill {
            memmove((*d).obuf.as_mut_ptr() as *mut libc::c_void,
                    (*d).obuf.as_mut_ptr().offset(z as isize) as
                        *const libc::c_void, z as libc::c_ulong);
        }
        (*d).ofill =
            ((*d).ofill as libc::c_ulong).wrapping_sub(z as libc::c_ulong) as
                size_t as size_t
    };
}
/*
 * Modify state flags of an LED
 *
 * Post: *led is updated with the new flags
 */
unsafe extern "C" fn set_led(mut led: *mut led_t, mut set: libc::c_uchar,
                             mut clear_0: libc::c_uchar) {
    let mut n: led_t = 0;
    n =
        (*led as libc::c_int & !(clear_0 as libc::c_int) | set as libc::c_int)
            as led_t;
    if n as libc::c_int != *led as libc::c_int {
        *led = (n as libc::c_int & !(0x4 as libc::c_int)) as led_t
    };
}
/*
 * Act on an event, and update the given LED status
 */
unsafe extern "C" fn event_decoded(mut d: *mut deck, mut led: *mut led_t,
                                   mut action: libc::c_uchar, mut shift: bool,
                                   mut button: libc::c_uchar, mut on: bool) {
    /* Always toggle the LED status */
    if on {
        set_led(&mut *led.offset(button as isize),
                0x2 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar);
    } else {
        set_led(&mut *led.offset(button as isize),
                0 as libc::c_int as libc::c_uchar,
                0x2 as libc::c_int as libc::c_uchar);
    }
    /* FIXME: We assume that we are the only operator of the cue
     * points; we should change the LEDs via a callback from deck */
    if shift as libc::c_int != 0 && on as libc::c_int != 0 {
        deck_unset_cue(d, button as libc::c_uint);
        set_led(&mut *led.offset(button as isize),
                0 as libc::c_int as libc::c_uchar,
                0x1 as libc::c_int as libc::c_uchar);
    }
    if shift { return }
    if action as libc::c_int == 0 as libc::c_int && on as libc::c_int != 0 {
        deck_cue(d, button as libc::c_uint);
        set_led(&mut *led.offset(button as isize),
                0x1 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar);
    }
    if action as libc::c_int == 1 as libc::c_int {
        if on {
            deck_punch_in(d, button as libc::c_uint);
            set_led(&mut *led.offset(button as isize),
                    0x1 as libc::c_int as libc::c_uchar,
                    0 as libc::c_int as libc::c_uchar);
        } else { deck_punch_out(d); }
    };
}
/*
 * Process an event from the device, given the MIDI control codes
 */
unsafe extern "C" fn event(mut d: *mut dicer, mut buf: *mut libc::c_uchar) {
    let mut deck: *mut deck = 0 as *mut deck;
    let mut led: *mut led_t = 0 as *mut led_t;
    let mut action: libc::c_uchar = 0;
    let mut button: libc::c_uchar = 0;
    let mut on: bool = false;
    let mut shift: bool = false;
    /* Ignore signal that the second controller is (un)plugged */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
           0xba as libc::c_int &&
           *buf.offset(1 as libc::c_int as isize) as libc::c_int ==
               0x11 as libc::c_int &&
           (*buf.offset(2 as libc::c_int as isize) as libc::c_int ==
                0 as libc::c_int ||
                *buf.offset(2 as libc::c_int as isize) as libc::c_int ==
                    0x8 as libc::c_int) {
        return
    }
    match *buf.offset(0 as libc::c_int as isize) as libc::c_int {
        154 | 155 | 156 => {
            deck = (*d).left;
            led = (*d).left_led.as_mut_ptr();
            action =
                (*buf.offset(0 as libc::c_int as isize) as libc::c_int -
                     0x9a as libc::c_int) as libc::c_uchar
        }
        157 | 158 | 159 => {
            deck = (*d).right;
            led = (*d).right_led.as_mut_ptr();
            action =
                (*buf.offset(0 as libc::c_int as isize) as libc::c_int -
                     0x9d as libc::c_int) as libc::c_uchar
        }
        _ => { abort(); }
    }
    if deck.is_null() {
        /* no deck assigned to this unit */
        return
    }
    match *buf.offset(1 as libc::c_int as isize) as libc::c_int {
        60 | 61 | 62 | 63 | 64 => {
            button =
                (*buf.offset(1 as libc::c_int as isize) as libc::c_int -
                     0x3c as libc::c_int) as libc::c_uchar;
            shift = 0 as libc::c_int != 0
        }
        65 | 66 | 67 | 68 | 69 => {
            button =
                (*buf.offset(1 as libc::c_int as isize) as libc::c_int -
                     0x41 as libc::c_int) as libc::c_uchar;
            shift = 1 as libc::c_int != 0
        }
        _ => { abort(); }
    }
    match *buf.offset(2 as libc::c_int as isize) as libc::c_int {
        0 => { on = 0 as libc::c_int != 0 }
        127 => { on = 1 as libc::c_int != 0 }
        _ => { abort(); }
    }
    event_decoded(deck, led, action, shift, button, on);
}
unsafe extern "C" fn pollfds(mut c: *mut controller, mut pe: *mut pollfd,
                             mut z: size_t) -> ssize_t {
    let mut d: *mut dicer = (*c).local as *mut dicer;
    return midi_pollfds(&mut (*d).midi, pe, z);
}
/*
 * Handler in the realtime thread, which polls on both input
 * and output
 */
unsafe extern "C" fn realtime(mut c: *mut controller) -> libc::c_int {
    let mut d: *mut dicer = (*c).local as *mut dicer;
    loop  {
        let mut buf: [libc::c_uchar; 3] = [0; 3];
        let mut z: ssize_t = 0;
        z =
            midi_read(&mut (*d).midi, buf.as_mut_ptr() as *mut libc::c_void,
                      ::std::mem::size_of::<[libc::c_uchar; 3]>() as
                          libc::c_ulong);
        if z == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int)
        }
        if z == 0 as libc::c_int as libc::c_long { break ; }
        event(d, buf.as_mut_ptr());
    }
    sync_all_leds(d);
    return 0 as libc::c_int;
}
unsafe extern "C" fn clear(mut c: *mut controller) {
    let mut d: *mut dicer = (*c).local as *mut dicer;
    let mut n: size_t = 0;
    /* FIXME: Uses non-blocking functionality really intended
     * for realtime; no guarantee buffer is emptied */
    n = 0 as libc::c_int as size_t;
    while n < 5 as libc::c_int as libc::c_ulong {
        set_led(&mut *(*d).left_led.as_mut_ptr().offset(n as isize),
                0 as libc::c_int as libc::c_uchar,
                0x1 as libc::c_int as libc::c_uchar);
        set_led(&mut *(*d).right_led.as_mut_ptr().offset(n as isize),
                0 as libc::c_int as libc::c_uchar,
                0x1 as libc::c_int as libc::c_uchar);
        n = n.wrapping_add(1)
    }
    sync_all_leds(d);
    midi_close(&mut (*d).midi);
    free((*c).local);
}
static mut dicer_ops: controller_ops =
    unsafe {
        {
            let mut init =
                controller_ops{add_deck:
                                   Some(add_deck as
                                            unsafe extern "C" fn(_:
                                                                     *mut controller,
                                                                 _: *mut deck)
                                                -> libc::c_int),
                               pollfds:
                                   Some(pollfds as
                                            unsafe extern "C" fn(_:
                                                                     *mut controller,
                                                                 _:
                                                                     *mut pollfd,
                                                                 _: size_t)
                                                -> ssize_t),
                               realtime:
                                   Some(realtime as
                                            unsafe extern "C" fn(_:
                                                                     *mut controller)
                                                -> libc::c_int),
                               clear:
                                   Some(clear as
                                            unsafe extern "C" fn(_:
                                                                     *mut controller)
                                                -> ()),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn dicer_init(mut c: *mut controller, mut rt: *mut rt,
                                    mut hw: *const libc::c_char)
 -> libc::c_int {
    let mut n: size_t = 0;
    let mut d: *mut dicer = 0 as *mut dicer;
    d = malloc(::std::mem::size_of::<dicer>() as libc::c_ulong) as *mut dicer;
    if d.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if !(midi_open(&mut (*d).midi, hw) == -(1 as libc::c_int)) {
        (*d).left = 0 as *mut deck;
        (*d).right = 0 as *mut deck;
        (*d).ofill = 0 as libc::c_int as size_t;
        n = 0 as libc::c_int as size_t;
        while n < 5 as libc::c_int as libc::c_ulong {
            (*d).left_led[n as usize] = 0 as libc::c_int as led_t;
            (*d).right_led[n as usize] = 0 as libc::c_int as led_t;
            n = n.wrapping_add(1)
        }
        if controller_init(c, &mut dicer_ops, d as *mut libc::c_void, rt) ==
               -(1 as libc::c_int) {
            midi_close(&mut (*d).midi);
        } else { return 0 as libc::c_int }
    }
    free(d as *mut libc::c_void);
    return -(1 as libc::c_int);
}
