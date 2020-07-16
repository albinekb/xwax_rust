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
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn pthread_spin_init(__lock: *mut pthread_spinlock_t,
                         __pshared: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_spin_destroy(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    #[no_mangle]
    fn rt_not_allowed();
    #[no_mangle]
    fn track_acquire(t: *mut track);
    #[no_mangle]
    fn track_release(t: *mut track);
    #[no_mangle]
    fn timecoder_get_position(tc: *mut timecoder, when: *mut libc::c_double)
     -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
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
pub type pid_t = __pid_t;
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
/* next slot with the same hash */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lut {
    pub slot: *mut slot,
    pub table: *mut slot_no_t,
    pub avail: slot_no_t,
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
pub type slot_no_t = libc::c_uint;
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
pub type spin = pthread_spinlock_t;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_PROCESS_SHARED: C2RustUnnamed = 1;
pub const PTHREAD_PROCESS_PRIVATE: C2RustUnnamed = 0;
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
#[inline]
unsafe extern "C" fn spin_init(mut s: *mut spin) {
    if pthread_spin_init(s, PTHREAD_PROCESS_PRIVATE as libc::c_int) !=
           0 as libc::c_int {
        abort();
    };
}
#[inline]
unsafe extern "C" fn spin_clear(mut s: *mut spin) {
    if pthread_spin_destroy(s) != 0 as libc::c_int { abort(); };
}
/*
 * Take a spinlock
 *
 * Pre: lock is initialised
 * Pre: lock is not held by the current thread
 * Pre: current thread is not realtime
 * Post: lock is held by the current thread
 */
#[inline]
unsafe extern "C" fn spin_lock(mut s: *mut spin) {
    rt_not_allowed();
    if pthread_spin_lock(s) != 0 as libc::c_int { abort(); };
}
/*
 * Try and take a spinlock
 *
 * Pre: lock is initialised
 * Pre: lock is not held by the current thread
 * Post: if true is returned, lock is held by the current thread
 */
#[inline]
unsafe extern "C" fn spin_try_lock(mut s: *mut spin) -> bool {
    let mut r: libc::c_int = 0;
    r = pthread_spin_trylock(s);
    match r {
        0 => { return 1 as libc::c_int != 0 }
        16 => { return 0 as libc::c_int != 0 }
        _ => { abort(); }
    };
}
/*
 * Release a spinlock
 *
 * Pre: lock is held by this thread
 * Post: lock is not held
 */
#[inline]
unsafe extern "C" fn spin_unlock(mut s: *mut spin) {
    if pthread_spin_unlock(s) != 0 as libc::c_int { abort(); };
}
/* Return a pointer to (not value of) the sample data for each channel */
#[inline]
unsafe extern "C" fn track_get_sample(mut tr: *mut track, mut s: libc::c_int)
 -> *mut libc::c_short {
    let mut b: *mut track_block = 0 as *mut track_block;
    b =
        (*tr).block[(s / (2048 as libc::c_int * 1024 as libc::c_int)) as
                        usize];
    return &mut *(*b).pcm.as_mut_ptr().offset((s %
                                                   (2048 as libc::c_int *
                                                        1024 as libc::c_int) *
                                                   2 as libc::c_int) as isize)
               as *mut libc::c_short;
}
/*
 * Return the pitch relative to reference playback speed
 */
#[inline]
unsafe extern "C" fn timecoder_get_pitch(mut tc: *mut timecoder)
 -> libc::c_double {
    return pitch_current(&mut (*tc).pitch) / (*tc).speed;
}
/*
 * The last 'safe' timecode value on the record. Beyond this value, we
 * probably want to ignore the timecode values, as we will hit the
 * label of the record.
 */
#[inline]
unsafe extern "C" fn timecoder_get_safe(mut tc: *mut timecoder)
 -> libc::c_uint {
    return (*(*tc).def).safe;
}
/*
 * The resolution of the timecode. This is the number of bits per
 * second at reference playback speed
 */
#[inline]
unsafe extern "C" fn timecoder_get_resolution(mut tc: *mut timecoder)
 -> libc::c_double {
    return (*(*tc).def).resolution as libc::c_double * (*tc).speed;
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
/* Values for the filter concluded experimentally */
/* State of the pitch calculation filter */
/* Prepare the filter for observations every dt seconds */
/* Input an observation to the filter; in the last dt seconds the
 * position has moved by dx.
 *
 * Because the vinyl uses timestamps, the values for dx are discrete
 * rather than smooth. */
/* relative to previous */
/* Get the pitch after filtering */
#[inline]
unsafe extern "C" fn pitch_current(mut p: *mut pitch) -> libc::c_double {
    return (*p).v;
}
/*
 * Return: the cubic interpolation of the sample at position 2 + mu
 */
#[inline]
unsafe extern "C" fn cubic_interpolate(mut y: *mut libc::c_short,
                                       mut mu: libc::c_double)
 -> libc::c_double {
    let mut a0: libc::c_long = 0;
    let mut a1: libc::c_long = 0;
    let mut a2: libc::c_long = 0;
    let mut a3: libc::c_long = 0;
    let mut mu2: libc::c_double = 0.;
    mu2 = mu * mu;
    a0 =
        (*y.offset(3 as libc::c_int as isize) as libc::c_int -
             *y.offset(2 as libc::c_int as isize) as libc::c_int -
             *y.offset(0 as libc::c_int as isize) as libc::c_int +
             *y.offset(1 as libc::c_int as isize) as libc::c_int) as
            libc::c_long;
    a1 =
        (*y.offset(0 as libc::c_int as isize) as libc::c_int -
             *y.offset(1 as libc::c_int as isize) as libc::c_int) as
            libc::c_long - a0;
    a2 =
        (*y.offset(2 as libc::c_int as isize) as libc::c_int -
             *y.offset(0 as libc::c_int as isize) as libc::c_int) as
            libc::c_long;
    a3 = *y.offset(1 as libc::c_int as isize) as libc::c_long;
    return mu * mu2 * a0 as libc::c_double + mu2 * a1 as libc::c_double +
               mu * a2 as libc::c_double + a3 as libc::c_double;
}
/*
 * Return: Random dither, between -0.5 and 0.5
 */
unsafe extern "C" fn dither() -> libc::c_double {
    let mut bit: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;
    static mut x: libc::c_uint = 0xbeefface as libc::c_uint;
    /* Maximum length LFSR sequence with 32-bit state */
    bit =
        (x ^ x >> 1 as libc::c_int ^ x >> 21 as libc::c_int ^
             x >> 31 as libc::c_int) & 1 as libc::c_int as libc::c_uint;
    x = x << 1 as libc::c_int | bit;
    /* We can adjust the balance between randomness and performance
     * by our chosen bit permutation; here we use a 12 bit subset
     * of the state */
    v =
        x & 0xf as libc::c_int as libc::c_uint |
            (x & 0xf0000 as libc::c_int as libc::c_uint) >> 12 as libc::c_int
            |
            (x & 0xf000000 as libc::c_int as libc::c_uint) >>
                16 as libc::c_int;
    return v as libc::c_double / 4096 as libc::c_int as libc::c_double -
               0.5f64;
    /* not quite whole range */
}
/*
 * Build a block of PCM audio, resampled from the track
 *
 * This is just a basic resampler which has a small amount of aliasing
 * where pitch > 1.0.
 *
 * Return: number of seconds advanced in the source audio track
 * Post: buffer at pcm is filled with the given number of samples
 */
unsafe extern "C" fn build_pcm(mut pcm: *mut libc::c_short,
                               mut samples: libc::c_uint,
                               mut sample_dt: libc::c_double,
                               mut tr: *mut track,
                               mut position: libc::c_double,
                               mut pitch: libc::c_double,
                               mut start_vol: libc::c_double,
                               mut end_vol: libc::c_double)
 -> libc::c_double {
    let mut s: libc::c_int = 0;
    let mut sample: libc::c_double = 0.;
    let mut step: libc::c_double = 0.;
    let mut vol: libc::c_double = 0.;
    let mut gradient: libc::c_double = 0.;
    sample = position * (*tr).rate as libc::c_double;
    step = sample_dt * pitch * (*tr).rate as libc::c_double;
    vol = start_vol;
    gradient = (end_vol - start_vol) / samples as libc::c_double;
    s = 0 as libc::c_int;
    while (s as libc::c_uint) < samples {
        let mut c: libc::c_int = 0;
        let mut sa: libc::c_int = 0;
        let mut q: libc::c_int = 0;
        let mut f: libc::c_double = 0.;
        let mut i: [[libc::c_short; 4]; 2] = [[0; 4]; 2];
        /* 4-sample window for interpolation */
        sa = sample as libc::c_int;
        if sample < 0.0f64 { sa -= 1 }
        f = sample - sa as libc::c_double;
        sa -= 1;
        q = 0 as libc::c_int;
        while q < 4 as libc::c_int {
            if sa < 0 as libc::c_int || sa as libc::c_uint >= (*tr).length {
                c = 0 as libc::c_int;
                while c < 2 as libc::c_int {
                    i[c as usize][q as usize] =
                        0 as libc::c_int as libc::c_short;
                    c += 1
                }
            } else {
                let mut ts: *mut libc::c_short = 0 as *mut libc::c_short;
                let mut c_0: libc::c_int = 0;
                ts = track_get_sample(tr, sa);
                c_0 = 0 as libc::c_int;
                while c_0 < 2 as libc::c_int {
                    i[c_0 as usize][q as usize] = *ts.offset(c_0 as isize);
                    c_0 += 1
                }
            }
            q += 1;
            sa += 1
        }
        c = 0 as libc::c_int;
        while c < 2 as libc::c_int {
            let mut v: libc::c_double = 0.;
            v =
                vol * cubic_interpolate(i[c as usize].as_mut_ptr(), f) +
                    dither();
            if v > 32767 as libc::c_int as libc::c_double {
                let fresh0 = pcm;
                pcm = pcm.offset(1);
                *fresh0 = 32767 as libc::c_int as libc::c_short
            } else if v <
                          (-(32767 as libc::c_int) - 1 as libc::c_int) as
                              libc::c_double {
                let fresh1 = pcm;
                pcm = pcm.offset(1);
                *fresh1 =
                    (-(32767 as libc::c_int) - 1 as libc::c_int) as
                        libc::c_short
            } else {
                let fresh2 = pcm;
                pcm = pcm.offset(1);
                *fresh2 = v as libc::c_short
            }
            c += 1
        }
        sample += step;
        vol += gradient;
        s += 1
    }
    return sample_dt * pitch * samples as libc::c_double;
}
/*
 * Equivalent to build_pcm, but for use when the track is
 * not available
 *
 * Return: number of seconds advanced in the audio track
 * Post: buffer at pcm is filled with silence
 */
unsafe extern "C" fn build_silence(mut pcm: *mut libc::c_short,
                                   mut samples: libc::c_uint,
                                   mut sample_dt: libc::c_double,
                                   mut pitch: libc::c_double)
 -> libc::c_double {
    memset(pcm as *mut libc::c_void, '\u{0}' as i32,
           (::std::mem::size_of::<libc::c_short>() as
                libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(samples
                                                                                as
                                                                                libc::c_ulong));
    return sample_dt * pitch * samples as libc::c_double;
}
/*
 * Change the timecoder used by this playback
 */
#[no_mangle]
pub unsafe extern "C" fn player_set_timecoder(mut pl: *mut player,
                                              mut tc: *mut timecoder) {
    if !tc.is_null() {
    } else {
        __assert_fail(b"tc != NULL\x00" as *const u8 as *const libc::c_char,
                      b"player.c\x00" as *const u8 as *const libc::c_char,
                      189 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 63],
                                                &[libc::c_char; 63]>(b"void player_set_timecoder(struct player *, struct timecoder *)\x00")).as_ptr());
    }
    (*pl).timecoder = tc;
    (*pl).recalibrate = 1 as libc::c_int != 0;
    (*pl).timecode_control = 1 as libc::c_int != 0;
}
/*
 * Post: player is initialised
 */
#[no_mangle]
pub unsafe extern "C" fn player_init(mut pl: *mut player,
                                     mut sample_rate: libc::c_uint,
                                     mut track: *mut track,
                                     mut tc: *mut timecoder) {
    if !track.is_null() {
    } else {
        __assert_fail(b"track != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"player.c\x00" as *const u8 as *const libc::c_char,
                      202 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"void player_init(struct player *, unsigned int, struct track *, struct timecoder *)\x00")).as_ptr());
    }
    if sample_rate != 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"sample_rate != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"player.c\x00" as *const u8 as *const libc::c_char,
                      203 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"void player_init(struct player *, unsigned int, struct track *, struct timecoder *)\x00")).as_ptr());
    }
    spin_init(&mut (*pl).lock);
    (*pl).sample_dt = 1.0f64 / sample_rate as libc::c_double;
    (*pl).track = track;
    player_set_timecoder(pl, tc);
    (*pl).position = 0.0f64;
    (*pl).offset = 0.0f64;
    (*pl).target_position = ::std::f32::INFINITY as libc::c_double;
    (*pl).last_difference = 0.0f64;
    (*pl).pitch = 0.0f64;
    (*pl).sync_pitch = 1.0f64;
    (*pl).volume = 0.0f64;
}
/*
 * Pre: player is initialised
 * Post: no resources are allocated by the player
 */
#[no_mangle]
pub unsafe extern "C" fn player_clear(mut pl: *mut player) {
    spin_clear(&mut (*pl).lock);
    track_release((*pl).track);
}
/*
 * Enable or disable timecode control
 */
#[no_mangle]
pub unsafe extern "C" fn player_set_timecode_control(mut pl: *mut player,
                                                     mut on: bool) {
    if on as libc::c_int != 0 && !(*pl).timecode_control {
        (*pl).recalibrate = 1 as libc::c_int != 0
    }
    (*pl).timecode_control = on;
}
/*
 * Toggle timecode control
 *
 * Return: the new state of timecode control
 */
#[no_mangle]
pub unsafe extern "C" fn player_toggle_timecode_control(mut pl: *mut player)
 -> bool {
    (*pl).timecode_control = !(*pl).timecode_control;
    if (*pl).timecode_control { (*pl).recalibrate = 1 as libc::c_int != 0 }
    return (*pl).timecode_control;
}
#[no_mangle]
pub unsafe extern "C" fn player_set_internal_playback(mut pl: *mut player) {
    (*pl).timecode_control = 0 as libc::c_int != 0;
    (*pl).pitch = 1.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn player_get_position(mut pl: *mut player)
 -> libc::c_double {
    return (*pl).position;
}
#[no_mangle]
pub unsafe extern "C" fn player_get_elapsed(mut pl: *mut player)
 -> libc::c_double {
    return (*pl).position - (*pl).offset;
}
#[no_mangle]
pub unsafe extern "C" fn player_get_remain(mut pl: *mut player)
 -> libc::c_double {
    return (*(*pl).track).length as libc::c_double /
               (*(*pl).track).rate as libc::c_double + (*pl).offset -
               (*pl).position;
}
#[no_mangle]
pub unsafe extern "C" fn player_is_active(mut pl: *const player) -> bool {
    return fabs((*pl).pitch) > 0.01f64;
}
/*
 * Cue to the zero position of the track
 */
#[no_mangle]
pub unsafe extern "C" fn player_recue(mut pl: *mut player) {
    (*pl).offset = (*pl).position;
}
/*
 * Set the track used for the playback
 *
 * Pre: caller holds reference on track
 * Post: caller does not hold reference on track
 */
#[no_mangle]
pub unsafe extern "C" fn player_set_track(mut pl: *mut player,
                                          mut track: *mut track) {
    let mut x: *mut track =
        0 as *mut track; /* Synchronise with the playback thread */
    if !track.is_null() {
    } else {
        __assert_fail(b"track != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"player.c\x00" as *const u8 as *const libc::c_char,
                      304 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void player_set_track(struct player *, struct track *)\x00")).as_ptr());
    }
    if (*track).refcount > 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"track->refcount > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"player.c\x00" as *const u8 as *const libc::c_char,
                      305 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void player_set_track(struct player *, struct track *)\x00")).as_ptr());
    }
    spin_lock(&mut (*pl).lock);
    x = (*pl).track;
    (*pl).track = track;
    spin_unlock(&mut (*pl).lock);
    track_release(x);
    /* discard the old track */
}
/*
 * Set the playback of one player to match another, used
 * for "instant doubles" and beat juggling
 */
#[no_mangle]
pub unsafe extern "C" fn player_clone(mut pl: *mut player,
                                      mut from: *const player) {
    let mut elapsed: libc::c_double = 0.;
    let mut x: *mut track = 0 as *mut track;
    let mut t: *mut track = 0 as *mut track;
    elapsed = (*from).position - (*from).offset;
    (*pl).offset = (*pl).position - elapsed;
    t = (*from).track;
    track_acquire(t);
    spin_lock(&mut (*pl).lock);
    x = (*pl).track;
    (*pl).track = t;
    spin_unlock(&mut (*pl).lock);
    track_release(x);
}
/*
 * Synchronise to the position and speed given by the timecoder
 *
 * Return: 0 on success or -1 if the timecoder is not currently valid
 */
unsafe extern "C" fn sync_to_timecode(mut pl: *mut player) -> libc::c_int {
    let mut when: libc::c_double = 0.;
    let mut tcpos: libc::c_double = 0.;
    let mut timecode: libc::c_int = 0;
    timecode = timecoder_get_position((*pl).timecoder, &mut when);
    /* Instruct the caller to disconnect the timecoder if the needle
     * is outside the 'safe' zone of the record */
    if timecode != -(1 as libc::c_int) &&
           timecode as libc::c_uint > timecoder_get_safe((*pl).timecoder) {
        return -(1 as libc::c_int)
    }
    /* If the timecoder is alive, use the pitch from the sine wave */
    (*pl).pitch = timecoder_get_pitch((*pl).timecoder);
    /* If we can read an absolute time from the timecode, then use it */
    if timecode == -(1 as libc::c_int) {
        (*pl).target_position = ::std::f32::INFINITY as libc::c_double
    } else {
        tcpos =
            timecode as libc::c_double /
                timecoder_get_resolution((*pl).timecoder);
        (*pl).target_position = tcpos + (*pl).pitch * when
    }
    return 0 as libc::c_int;
}
/*
 * Synchronise to the position given by the timecoder without
 * affecting the audio playback position
 */
unsafe extern "C" fn calibrate_to_timecode_position(mut pl: *mut player) {
    if (*pl).target_position != ::std::f32::INFINITY as libc::c_double {
    } else {
        __assert_fail(b"pl->target_position != TARGET_UNKNOWN\x00" as
                          *const u8 as *const libc::c_char,
                      b"player.c\x00" as *const u8 as *const libc::c_char,
                      381 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"void calibrate_to_timecode_position(struct player *)\x00")).as_ptr());
    }
    (*pl).offset += (*pl).target_position - (*pl).position;
    (*pl).position = (*pl).target_position;
}
#[no_mangle]
pub unsafe extern "C" fn retarget(mut pl: *mut player) {
    let mut diff: libc::c_double = 0.;
    if (*pl).recalibrate {
        calibrate_to_timecode_position(pl);
        (*pl).recalibrate = 0 as libc::c_int != 0
    }
    /* Calculate the pitch compensation required to get us back on
     * track with the absolute timecode position */
    diff =
        (*pl).position -
            (*pl).target_position; /* to print in user interface */
    (*pl).last_difference = diff;
    if fabs(diff) > 1.0f64 / 8 as libc::c_int as libc::c_double {
        /* Jump the track to the time */
        (*pl).position = (*pl).target_position;
        fprintf(stderr,
                b"Seek to new position %.2lfs.\n\x00" as *const u8 as
                    *const libc::c_char, (*pl).position);
    } else if fabs((*pl).pitch) > 0.05f64 {
        /* Re-calculate the drift between the timecoder pitch from
         * the sine wave and the timecode values */
        (*pl).sync_pitch =
            (*pl).pitch /
                (diff / (1.0f64 / 2 as libc::c_int as libc::c_double) +
                     (*pl).pitch)
    };
}
/*
 * Seek to the given position
 */
#[no_mangle]
pub unsafe extern "C" fn player_seek_to(mut pl: *mut player,
                                        mut seconds: libc::c_double) {
    (*pl).offset = (*pl).position - seconds;
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
/* Current playback parameters */
/* seconds */
/* seconds, or TARGET_UNKNOWN */
/* track start point in timecode */
/* last known position minus target_position */
/* from timecoder */
/* pitch required to sync to timecode signal */
/* Timecode control */
/* re-sync offset at next opportunity */
/*
 * Get a block of PCM audio data to send to the soundcard
 *
 * This is the main function which retrieves audio for playback.  The
 * clock of playback is decoupled from the clock of the timecode
 * signal.
 *
 * Post: buffer at pcm is filled with the given number of samples
 */
#[no_mangle]
pub unsafe extern "C" fn player_collect(mut pl: *mut player,
                                        mut pcm: *mut libc::c_short,
                                        mut samples: libc::c_uint) {
    let mut r: libc::c_double = 0.;
    let mut pitch: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut target_volume: libc::c_double = 0.;
    dt = (*pl).sample_dt * samples as libc::c_double;
    if (*pl).timecode_control {
        if sync_to_timecode(pl) == -(1 as libc::c_int) {
            (*pl).timecode_control = 0 as libc::c_int != 0
        }
    }
    if (*pl).target_position != ::std::f32::INFINITY as libc::c_double {
        /* Bias the pitch towards a known target, and acknowledge that
         * we did so */
        retarget(pl);
        (*pl).target_position = ::std::f32::INFINITY as libc::c_double
    } else {
        /* Without a known target, tend sync_pitch towards 1.0, to
         * avoid using outlier values from scratching for too long */
        (*pl).sync_pitch += dt / (0.05f64 + dt) * (1.0f64 - (*pl).sync_pitch)
    }
    target_volume =
        fabs((*pl).pitch) * (7.0f64 / 8 as libc::c_int as libc::c_double);
    if target_volume > 1.0f64 { target_volume = 1.0f64 }
    /* Sync pitch is applied post-filtering */
    pitch = (*pl).pitch * (*pl).sync_pitch;
    /* We must return audio immediately to stay realtime. A spin
     * lock protects us from changes to the audio source */
    if !spin_try_lock(&mut (*pl).lock) {
        r = build_silence(pcm, samples, (*pl).sample_dt, pitch)
    } else {
        r =
            build_pcm(pcm, samples, (*pl).sample_dt, (*pl).track,
                      (*pl).position - (*pl).offset, pitch, (*pl).volume,
                      target_volume);
        spin_unlock(&mut (*pl).lock);
    }
    (*pl).position += r;
    (*pl).volume = target_volume;
}
