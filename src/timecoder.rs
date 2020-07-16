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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* next available slot */
    #[no_mangle]
    fn lut_init(lut: *mut lut, nslots: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lut_clear(lut: *mut lut);
    #[no_mangle]
    fn lut_push(lut: *mut lut, timecode: libc::c_uint);
    #[no_mangle]
    fn lut_lookup(lut: *mut lut, timecode: libc::c_uint) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
/* next slot with the same hash */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lut {
    pub slot: *mut slot,
    pub table: *mut slot_no_t,
    pub avail: slot_no_t,
}
/* State of the pitch calculation filter */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pitch {
    pub dt: libc::c_double,
    pub x: libc::c_double,
    pub v: libc::c_double,
}
pub type bits_t = libc::c_uint;
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
pub struct timecoder_channel {
    pub positive: bool,
    pub swapped: bool,
    pub zero: libc::c_int,
    pub crossing_ticker: libc::c_uint,
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
/* Prepare the filter for observations every dt seconds */
#[inline]
unsafe extern "C" fn pitch_init(mut p: *mut pitch, mut dt: libc::c_double) {
    (*p).dt = dt;
    (*p).x = 0.0f64;
    (*p).v = 0.0f64;
}
/* Input an observation to the filter; in the last dt seconds the
 * position has moved by dx.
 *
 * Because the vinyl uses timestamps, the values for dx are discrete
 * rather than smooth. */
#[inline]
unsafe extern "C" fn pitch_dt_observation(mut p: *mut pitch,
                                          mut dx: libc::c_double) {
    let mut predicted_x: libc::c_double = 0.;
    let mut predicted_v: libc::c_double = 0.;
    let mut residual_x: libc::c_double = 0.;
    predicted_x = (*p).x + (*p).v * (*p).dt;
    predicted_v = (*p).v;
    residual_x = dx - predicted_x;
    (*p).x =
        predicted_x +
            residual_x * (1.0f64 / 512 as libc::c_int as libc::c_double);
    (*p).v =
        predicted_v +
            residual_x *
                (1.0f64 / 512 as libc::c_int as libc::c_double /
                     256 as libc::c_int as libc::c_double) / (*p).dt;
    (*p).x -= dx;
    /* relative to previous */
}
/* read bit values in negative (not positive) */
static mut timecodes: [timecode_def; 7] =
    [{
         let mut init =
             timecode_def{name:
                              b"serato_2a\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          desc:
                              b"Serato 2nd Ed., side A\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          bits: 20 as libc::c_int,
                          resolution: 1000 as libc::c_int,
                          flags: 0,
                          seed: 0x59017 as libc::c_int as bits_t,
                          taps: 0x361e4 as libc::c_int as bits_t,
                          length: 712000 as libc::c_int as libc::c_uint,
                          safe: 707000 as libc::c_int as libc::c_uint,
                          lookup: false,
                          lut:
                              lut{slot: 0 as *const slot as *mut slot,
                                  table:
                                      0 as *const slot_no_t as *mut slot_no_t,
                                  avail: 0,},};
         init
     },
     {
         let mut init =
             timecode_def{name:
                              b"serato_2b\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          desc:
                              b"Serato 2nd Ed., side B\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          bits: 20 as libc::c_int,
                          resolution: 1000 as libc::c_int,
                          flags: 0,
                          seed: 0x8f3c6 as libc::c_int as bits_t,
                          taps: 0x4f0d8 as libc::c_int as bits_t,
                          length: 922000 as libc::c_int as libc::c_uint,
                          safe: 917000 as libc::c_int as libc::c_uint,
                          lookup: false,
                          lut:
                              lut{slot: 0 as *const slot as *mut slot,
                                  table:
                                      0 as *const slot_no_t as *mut slot_no_t,
                                  avail: 0,},};
         init
     },
     {
         let mut init =
             timecode_def{name:
                              b"serato_cd\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          desc:
                              b"Serato CD\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          bits: 20 as libc::c_int,
                          resolution: 1000 as libc::c_int,
                          flags: 0,
                          seed: 0xd8b40 as libc::c_int as bits_t,
                          taps: 0x34d54 as libc::c_int as bits_t,
                          length: 950000 as libc::c_int as libc::c_uint,
                          safe: 940000 as libc::c_int as libc::c_uint,
                          lookup: false,
                          lut:
                              lut{slot: 0 as *const slot as *mut slot,
                                  table:
                                      0 as *const slot_no_t as *mut slot_no_t,
                                  avail: 0,},};
         init
     },
     {
         let mut init =
             timecode_def{name:
                              b"traktor_a\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          desc:
                              b"Traktor Scratch, side A\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          bits: 23 as libc::c_int,
                          resolution: 2000 as libc::c_int,
                          flags:
                              0x2 as libc::c_int | 0x4 as libc::c_int |
                                  0x1 as libc::c_int,
                          seed: 0x134503 as libc::c_int as bits_t,
                          taps: 0x41040 as libc::c_int as bits_t,
                          length: 1500000 as libc::c_int as libc::c_uint,
                          safe: 1480000 as libc::c_int as libc::c_uint,
                          lookup: false,
                          lut:
                              lut{slot: 0 as *const slot as *mut slot,
                                  table:
                                      0 as *const slot_no_t as *mut slot_no_t,
                                  avail: 0,},};
         init
     },
     {
         let mut init =
             timecode_def{name:
                              b"traktor_b\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          desc:
                              b"Traktor Scratch, side B\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          bits: 23 as libc::c_int,
                          resolution: 2000 as libc::c_int,
                          flags:
                              0x2 as libc::c_int | 0x4 as libc::c_int |
                                  0x1 as libc::c_int,
                          seed: 0x32066c as libc::c_int as bits_t,
                          taps: 0x41040 as libc::c_int as bits_t,
                          length: 2110000 as libc::c_int as libc::c_uint,
                          safe: 2090000 as libc::c_int as libc::c_uint,
                          lookup: false,
                          lut:
                              lut{slot: 0 as *const slot as *mut slot,
                                  table:
                                      0 as *const slot_no_t as *mut slot_no_t,
                                  avail: 0,},};
         init
     },
     {
         let mut init =
             timecode_def{name:
                              b"mixvibes_v2\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          desc:
                              b"MixVibes V2\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          bits: 20 as libc::c_int,
                          resolution: 1300 as libc::c_int,
                          flags: 0x1 as libc::c_int,
                          seed: 0x22c90 as libc::c_int as bits_t,
                          taps: 0x8 as libc::c_int as bits_t,
                          length: 950000 as libc::c_int as libc::c_uint,
                          safe: 923000 as libc::c_int as libc::c_uint,
                          lookup: false,
                          lut:
                              lut{slot: 0 as *const slot as *mut slot,
                                  table:
                                      0 as *const slot_no_t as *mut slot_no_t,
                                  avail: 0,},};
         init
     },
     {
         let mut init =
             timecode_def{name:
                              b"mixvibes_7inch\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          desc:
                              b"MixVibes 7\"\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                          bits: 20 as libc::c_int,
                          resolution: 1300 as libc::c_int,
                          flags: 0x1 as libc::c_int,
                          seed: 0x22c90 as libc::c_int as bits_t,
                          taps: 0x8 as libc::c_int as bits_t,
                          length: 312000 as libc::c_int as libc::c_uint,
                          safe: 310000 as libc::c_int as libc::c_uint,
                          lookup: false,
                          lut:
                              lut{slot: 0 as *const slot as *mut slot,
                                  table:
                                      0 as *const slot_no_t as *mut slot_no_t,
                                  avail: 0,},};
         init
     }];
/*
 * Calculate LFSR bit
 */
#[inline]
unsafe extern "C" fn lfsr(mut code: bits_t, mut taps: bits_t) -> bits_t {
    let mut taken: bits_t = 0;
    let mut xrs: libc::c_int = 0;
    taken = code & taps;
    xrs = 0 as libc::c_int;
    while taken != 0 as libc::c_int as libc::c_uint {
        xrs =
            (xrs as
                 libc::c_uint).wrapping_add(taken &
                                                0x1 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int;
        taken >>= 1 as libc::c_int
    }
    return (xrs & 0x1 as libc::c_int) as bits_t;
}
/*
 * Linear Feedback Shift Register in the forward direction. New values
 * are generated at the least-significant bit.
 */
#[inline]
unsafe extern "C" fn fwd(mut current: bits_t, mut def: *mut timecode_def)
 -> bits_t {
    let mut l: bits_t = 0;
    /* New bits are added at the MSB; shift right by one */
    l = lfsr(current, (*def).taps | 0x1 as libc::c_int as libc::c_uint);
    return current >> 1 as libc::c_int | l << (*def).bits - 1 as libc::c_int;
}
/*
 * Linear Feedback Shift Register in the reverse direction
 */
#[inline]
unsafe extern "C" fn rev(mut current: bits_t, mut def: *mut timecode_def)
 -> bits_t {
    let mut l: bits_t = 0;
    let mut mask: bits_t = 0;
    /* New bits are added at the LSB; shift left one and mask */
    mask = (((1 as libc::c_int) << (*def).bits) - 1 as libc::c_int) as bits_t;
    l =
        lfsr(current,
             (*def).taps >> 1 as libc::c_int |
                 ((0x1 as libc::c_int) << (*def).bits - 1 as libc::c_int) as
                     libc::c_uint);
    return current << 1 as libc::c_int & mask | l;
}
/*
 * Where necessary, build the lookup table required for this timecode
 *
 * Return: -1 if not enough memory could be allocated, otherwise 0
 */
unsafe extern "C" fn build_lookup(mut def: *mut timecode_def) -> libc::c_int {
    let mut n: libc::c_uint = 0;
    let mut current: bits_t = 0;
    if (*def).lookup { return 0 as libc::c_int }
    fprintf(stderr,
            b"Building LUT for %d bit %dHz timecode (%s)\n\x00" as *const u8
                as *const libc::c_char, (*def).bits, (*def).resolution,
            (*def).desc);
    if lut_init(&mut (*def).lut, (*def).length as libc::c_int) ==
           -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    current = (*def).seed;
    n = 0 as libc::c_int as libc::c_uint;
    while n < (*def).length {
        let mut next: bits_t = 0;
        /* timecode must not wrap */
        lut_push(&mut (*def).lut, current);
        /* check symmetry of the lfsr functions */
        next = fwd(current, def);
        current = next;
        n = n.wrapping_add(1)
    }
    (*def).lookup = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
/*
 * Find a timecode definition by name
 *
 * Return: pointer to timecode definition, or NULL if not found
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_find_definition(mut name:
                                                       *const libc::c_char)
 -> *mut timecode_def {
    let mut def: *mut timecode_def = 0 as *mut timecode_def;
    let mut end: *mut timecode_def = 0 as *mut timecode_def;
    def =
        &mut *timecodes.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut timecode_def;
    end =
        def.offset((::std::mem::size_of::<[timecode_def; 7]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<timecode_def>()
                                                        as libc::c_ulong) as
                       isize);
    while !(strcmp((*def).name, name) == 0) {
        def = def.offset(1);
        if def == end { return 0 as *mut timecode_def }
    }
    if build_lookup(def) == -(1 as libc::c_int) {
        return 0 as *mut timecode_def
    }
    return def;
}
/*
 * Free the timecoder lookup tables when they are no longer needed
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_free_lookup() {
    let mut def: *mut timecode_def = 0 as *mut timecode_def;
    let mut end: *mut timecode_def = 0 as *mut timecode_def;
    def =
        &mut *timecodes.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut timecode_def;
    end =
        def.offset((::std::mem::size_of::<[timecode_def; 7]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<timecode_def>()
                                                        as libc::c_ulong) as
                       isize);
    while def < end {
        if (*def).lookup { lut_clear(&mut (*def).lut); }
        def = def.offset(1)
    };
}
/*
 * Initialise filter values for one channel
 */
unsafe extern "C" fn init_channel(mut ch: *mut timecoder_channel) {
    (*ch).positive = 0 as libc::c_int != 0;
    (*ch).zero = 0 as libc::c_int;
}
/*
 * Initialise a timecode decoder at the given reference speed
 *
 * Return: -1 if the timecoder could not be initialised, otherwise 0
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_init(mut tc: *mut timecoder,
                                        mut def: *mut timecode_def,
                                        mut speed: libc::c_double,
                                        mut sample_rate: libc::c_uint,
                                        mut phono: bool) {
    if !def.is_null() {
    } else {
        __assert_fail(b"def != NULL\x00" as *const u8 as *const libc::c_char,
                      b"timecoder.c\x00" as *const u8 as *const libc::c_char,
                      285 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[libc::c_char; 92]>(b"void timecoder_init(struct timecoder *, struct timecode_def *, double, unsigned int, _Bool)\x00")).as_ptr());
    }
    /* A definition contains a lookup table which can be shared
     * across multiple timecoders */
    if (*def).lookup {
    } else {
        __assert_fail(b"def->lookup\x00" as *const u8 as *const libc::c_char,
                      b"timecoder.c\x00" as *const u8 as *const libc::c_char,
                      290 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[libc::c_char; 92]>(b"void timecoder_init(struct timecoder *, struct timecode_def *, double, unsigned int, _Bool)\x00")).as_ptr()); /* approx -36dB */
    }
    (*tc).def = def;
    (*tc).speed = speed;
    (*tc).dt = 1.0f64 / sample_rate as libc::c_double;
    (*tc).zero_alpha = (*tc).dt / (0.001f64 + (*tc).dt);
    (*tc).threshold = (128 as libc::c_int) << 16 as libc::c_int;
    if phono { (*tc).threshold >>= 5 as libc::c_int }
    (*tc).forwards = 1 as libc::c_int != 0;
    init_channel(&mut (*tc).primary);
    init_channel(&mut (*tc).secondary);
    pitch_init(&mut (*tc).pitch, (*tc).dt);
    (*tc).ref_level = 2147483647 as libc::c_int;
    (*tc).bitstream = 0 as libc::c_int as bits_t;
    (*tc).timecode = 0 as libc::c_int as bits_t;
    (*tc).valid_counter = 0 as libc::c_int as libc::c_uint;
    (*tc).timecode_ticker = 0 as libc::c_int as libc::c_uint;
    (*tc).mon = 0 as *mut libc::c_uchar;
}
/*
 * Clear resources associated with a timecode decoder
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_clear(mut tc: *mut timecoder) {
    if (*tc).mon.is_null() {
    } else {
        __assert_fail(b"tc->mon == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"timecoder.c\x00" as *const u8 as *const libc::c_char,
                      320 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void timecoder_clear(struct timecoder *)\x00")).as_ptr());
    };
}
/*
 * Initialise a raster display of the incoming audio
 *
 * The monitor (otherwise known as 'scope' in the interface) is an x-y
 * display of the post-calibrated incoming audio.
 *
 * Return: -1 if not enough memory could be allocated, otherwise 0
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_monitor_init(mut tc: *mut timecoder,
                                                mut size: libc::c_int)
 -> libc::c_int {
    if (*tc).mon.is_null() {
    } else {
        __assert_fail(b"tc->mon == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"timecoder.c\x00" as *const u8 as *const libc::c_char,
                      334 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 52],
                                                &[libc::c_char; 52]>(b"int timecoder_monitor_init(struct timecoder *, int)\x00")).as_ptr());
    }
    (*tc).mon_size = size;
    (*tc).mon =
        malloc(((*tc).mon_size * (*tc).mon_size) as libc::c_ulong) as
            *mut libc::c_uchar;
    if (*tc).mon.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    memset((*tc).mon as *mut libc::c_void, 0 as libc::c_int,
           ((*tc).mon_size * (*tc).mon_size) as libc::c_ulong);
    (*tc).mon_counter = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/*
 * Clear the monitor on the given timecoder
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_monitor_clear(mut tc: *mut timecoder) {
    if !(*tc).mon.is_null() {
    } else {
        __assert_fail(b"tc->mon != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"timecoder.c\x00" as *const u8 as *const libc::c_char,
                      352 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void timecoder_monitor_clear(struct timecoder *)\x00")).as_ptr());
    }
    free((*tc).mon as *mut libc::c_void);
    (*tc).mon = 0 as *mut libc::c_uchar;
}
/*
 * Update channel information with axis-crossings
 */
unsafe extern "C" fn detect_zero_crossing(mut ch: *mut timecoder_channel,
                                          mut v: libc::c_int,
                                          mut alpha: libc::c_double,
                                          mut threshold: libc::c_int) {
    (*ch).crossing_ticker = (*ch).crossing_ticker.wrapping_add(1);
    (*ch).swapped = 0 as libc::c_int != 0;
    if v > (*ch).zero + threshold && !(*ch).positive {
        (*ch).swapped = 1 as libc::c_int != 0;
        (*ch).positive = 1 as libc::c_int != 0;
        (*ch).crossing_ticker = 0 as libc::c_int as libc::c_uint
    } else if v < (*ch).zero - threshold && (*ch).positive as libc::c_int != 0
     {
        (*ch).swapped = 1 as libc::c_int != 0;
        (*ch).positive = 0 as libc::c_int != 0;
        (*ch).crossing_ticker = 0 as libc::c_int as libc::c_uint
    }
    (*ch).zero =
        ((*ch).zero as libc::c_double +
             alpha * (v - (*ch).zero) as libc::c_double) as libc::c_int;
}
/*
 * Plot the given sample value in the x-y monitor
 */
unsafe extern "C" fn update_monitor(mut tc: *mut timecoder,
                                    mut x: libc::c_int, mut y: libc::c_int) {
    let mut px: libc::c_int = 0;
    let mut py: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ref_0: libc::c_int = 0;
    if (*tc).mon.is_null() { return }
    size = (*tc).mon_size;
    ref_0 = (*tc).ref_level;
    /* Decay the pixels already in the montior */
    (*tc).mon_counter += 1;
    if (*tc).mon_counter % 512 as libc::c_int == 0 as libc::c_int {
        let mut p: libc::c_int = 0;
        p = 0 as libc::c_int;
        while p < size * size {
            if *(*tc).mon.offset(p as isize) != 0 {
                *(*tc).mon.offset(p as isize) =
                    (*(*tc).mon.offset(p as isize) as libc::c_int *
                         7 as libc::c_int / 8 as libc::c_int) as libc::c_uchar
            }
            p += 1
        }
    }
    if ref_0 > 0 as libc::c_int {
    } else {
        __assert_fail(b"ref > 0\x00" as *const u8 as *const libc::c_char,
                      b"timecoder.c\x00" as *const u8 as *const libc::c_char,
                      406 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"void update_monitor(struct timecoder *, int, int)\x00")).as_ptr());
    }
    /* ref_level is half the prevision of signal level */
    px =
        ((size / 2 as libc::c_int) as libc::c_longlong +
             x as libc::c_longlong * size as libc::c_longlong /
                 ref_0 as libc::c_longlong /
                 8 as libc::c_int as libc::c_longlong) as libc::c_int;
    py =
        ((size / 2 as libc::c_int) as libc::c_longlong +
             y as libc::c_longlong * size as libc::c_longlong /
                 ref_0 as libc::c_longlong /
                 8 as libc::c_int as libc::c_longlong) as libc::c_int;
    if px < 0 as libc::c_int || px >= size || py < 0 as libc::c_int ||
           py >= size {
        return
    }
    *(*tc).mon.offset((py * size + px) as isize) =
        0xff as libc::c_int as libc::c_uchar;
    /* white */
}
/*
 * Extract the bitstream from the sample value
 */
unsafe extern "C" fn process_bitstream(mut tc: *mut timecoder,
                                       mut m: libc::c_int) {
    let mut b: bits_t = 0;
    b = (m > (*tc).ref_level) as libc::c_int as bits_t;
    /* Add it to the bitstream, and work out what we were expecting
     * (timecode). */
    /* tc->bitstream is always in the order it is physically placed on
     * the vinyl, regardless of the direction. */
    if (*tc).forwards {
        (*tc).timecode = fwd((*tc).timecode, (*tc).def);
        (*tc).bitstream =
            ((*tc).bitstream >>
                 1 as
                     libc::c_int).wrapping_add(b <<
                                                   (*(*tc).def).bits -
                                                       1 as libc::c_int)
    } else {
        let mut mask: bits_t = 0;
        mask =
            (((1 as libc::c_int) << (*(*tc).def).bits) - 1 as libc::c_int) as
                bits_t;
        (*tc).timecode = rev((*tc).timecode, (*tc).def);
        (*tc).bitstream =
            ((*tc).bitstream << 1 as libc::c_int & mask).wrapping_add(b)
    }
    if (*tc).timecode == (*tc).bitstream {
        (*tc).valid_counter = (*tc).valid_counter.wrapping_add(1)
    } else {
        (*tc).timecode = (*tc).bitstream;
        (*tc).valid_counter = 0 as libc::c_int as libc::c_uint
    }
    /* Take note of the last time we read a valid timecode */
    (*tc).timecode_ticker = 0 as libc::c_int as libc::c_uint;
    /* Adjust the reference level based on this new peak */
    (*tc).ref_level -= (*tc).ref_level / 48 as libc::c_int;
    (*tc).ref_level += m / 48 as libc::c_int;
}
/*
 * Process a single sample from the incoming audio
 *
 * The two input signals (primary and secondary) are in the full range
 * of a signed int; ie. 32-bit signed.
 */
unsafe extern "C" fn process_sample(mut tc: *mut timecoder,
                                    mut primary: libc::c_int,
                                    mut secondary: libc::c_int) {
    detect_zero_crossing(&mut (*tc).primary, primary, (*tc).zero_alpha,
                         (*tc).threshold);
    detect_zero_crossing(&mut (*tc).secondary, secondary, (*tc).zero_alpha,
                         (*tc).threshold);
    /* If an axis has been crossed, use the direction of the crossing
     * to work out the direction of the vinyl */
    if (*tc).primary.swapped as libc::c_int != 0 ||
           (*tc).secondary.swapped as libc::c_int != 0 {
        let mut forwards: bool = false;
        if (*tc).primary.swapped {
            forwards =
                (*tc).primary.positive as libc::c_int !=
                    (*tc).secondary.positive as libc::c_int
        } else {
            forwards =
                (*tc).primary.positive as libc::c_int ==
                    (*tc).secondary.positive as libc::c_int
        }
        if (*(*tc).def).flags & 0x1 as libc::c_int != 0 {
            forwards = !forwards
        }
        if forwards as libc::c_int != (*tc).forwards as libc::c_int {
            /* direction has changed */
            (*tc).forwards = forwards;
            (*tc).valid_counter = 0 as libc::c_int as libc::c_uint
        }
    }
    /* If any axis has been crossed, register movement using the pitch
     * counters */
    if !(*tc).primary.swapped && !(*tc).secondary.swapped {
        pitch_dt_observation(&mut (*tc).pitch, 0.0f64);
    } else {
        let mut dx: libc::c_double = 0.;
        dx =
            1.0f64 / (*(*tc).def).resolution as libc::c_double /
                4 as libc::c_int as libc::c_double;
        if !(*tc).forwards { dx = -dx }
        pitch_dt_observation(&mut (*tc).pitch, dx);
    }
    /* If we have crossed the primary channel in the right polarity,
     * it's time to read off a timecode 0 or 1 value */
    if (*tc).secondary.swapped as libc::c_int != 0 &&
           (*tc).primary.positive as libc::c_int ==
               ((*(*tc).def).flags & 0x4 as libc::c_int == 0 as libc::c_int)
                   as libc::c_int {
        let mut m: libc::c_int = 0;
        /* scale to avoid clipping */
        m =
            abs(primary / 2 as libc::c_int -
                    (*tc).primary.zero / 2 as libc::c_int);
        process_bitstream(tc, m);
    }
    (*tc).timecode_ticker = (*tc).timecode_ticker.wrapping_add(1);
}
/*
 * Cycle to the next timecode definition which has a valid lookup
 *
 * Return: pointer to timecode definition
 */
unsafe extern "C" fn next_definition(mut def: *mut timecode_def)
 -> *mut timecode_def {
    if !def.is_null() {
    } else {
        __assert_fail(b"def != NULL\x00" as *const u8 as *const libc::c_char,
                      b"timecoder.c\x00" as *const u8 as *const libc::c_char,
                      542 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 60],
                                                &[libc::c_char; 60]>(b"struct timecode_def *next_definition(struct timecode_def *)\x00")).as_ptr());
    }
    loop  {
        def = def.offset(1);
        if def >
               timecodes.as_mut_ptr().offset((::std::mem::size_of::<[timecode_def; 7]>()
                                                  as
                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<timecode_def>()
                                                                                  as
                                                                                  libc::c_ulong)
                                                 as isize) {
            def = timecodes.as_mut_ptr()
        }
        if (*def).lookup { break ; }
    }
    return def;
}
/*
 * Change the timecode definition to the next available
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_cycle_definition(mut tc: *mut timecoder) {
    (*tc).def = next_definition((*tc).def);
    (*tc).valid_counter = 0 as libc::c_int as libc::c_uint;
    (*tc).timecode_ticker = 0 as libc::c_int as libc::c_uint;
}
/*
 * Submit and decode a block of PCM audio data to the timecode decoder
 *
 * PCM data is in the full range of signed short; ie. 16-bit signed.
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_submit(mut tc: *mut timecoder,
                                          mut pcm: *mut libc::c_short,
                                          mut npcm: size_t) {
    loop  {
        let fresh0 = npcm;
        npcm = npcm.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        let mut left: libc::c_int = 0;
        let mut right: libc::c_int = 0;
        let mut primary: libc::c_int = 0;
        let mut secondary: libc::c_int = 0;
        left =
            (*pcm.offset(0 as libc::c_int as isize) as libc::c_int) <<
                16 as libc::c_int;
        right =
            (*pcm.offset(1 as libc::c_int as isize) as libc::c_int) <<
                16 as libc::c_int;
        if (*(*tc).def).flags & 0x2 as libc::c_int != 0 {
            primary = left;
            secondary = right
        } else { primary = right; secondary = left }
        process_sample(tc, primary, secondary);
        update_monitor(tc, left, right);
        pcm = pcm.offset(2 as libc::c_int as isize)
    };
}
/*
 * Get the last-known position of the timecode
 *
 * If now data is available or if too few bits have been error
 * checked, then this counts as invalid. The last known position is
 * given along with the time elapsed since the position stamp was
 * read.
 *
 * Return: the known position of the timecode, or -1 if not known
 * Post: if when != NULL, *when is the elapsed time in seconds
 */
#[no_mangle]
pub unsafe extern "C" fn timecoder_get_position(mut tc: *mut timecoder,
                                                mut when: *mut libc::c_double)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*tc).valid_counter <= 24 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    r = lut_lookup(&mut (*(*tc).def).lut, (*tc).bitstream) as libc::c_int;
    if r == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if !when.is_null() {
        *when = (*tc).timecode_ticker as libc::c_double * (*tc).dt
    }
    return r;
}
