use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
pub type size_t = libc::c_ulong;
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
/*
 * A set of cue points
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
#[no_mangle]
pub unsafe extern "C" fn cues_reset(mut q: *mut cues) {
    let mut n: size_t = 0;
    n = 0 as libc::c_int as size_t;
    while n < 16 as libc::c_int as libc::c_ulong {
        (*q).position[n as usize] = ::std::f64::INFINITY;
        n = n.wrapping_add(1)
    };
}
/*
 * Unset the given cue point
 */
#[no_mangle]
pub unsafe extern "C" fn cues_unset(mut q: *mut cues,
                                    mut label: libc::c_uint) {
    (*q).position[label as usize] = ::std::f64::INFINITY;
}
#[no_mangle]
pub unsafe extern "C" fn cues_set(mut q: *mut cues, mut label: libc::c_uint,
                                  mut position: libc::c_double) {
    if label < 16 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"label < MAX_CUES\x00" as *const u8 as
                          *const libc::c_char,
                      b"cues.c\x00" as *const u8 as *const libc::c_char,
                      47 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void cues_set(struct cues *, unsigned int, double)\x00")).as_ptr());
    }
    (*q).position[label as usize] = position;
}
#[no_mangle]
pub unsafe extern "C" fn cues_get(mut q: *const cues, mut label: libc::c_uint)
 -> libc::c_double {
    if label < 16 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"label < MAX_CUES\x00" as *const u8 as
                          *const libc::c_char,
                      b"cues.c\x00" as *const u8 as *const libc::c_char,
                      53 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"double cues_get(const struct cues *, unsigned int)\x00")).as_ptr());
    }
    return (*q).position[label as usize];
}
/*
 * Return: the previous cue point before the current position, or CUE_UNSET
 */
#[no_mangle]
pub unsafe extern "C" fn cues_prev(mut q: *const cues,
                                   mut current: libc::c_double)
 -> libc::c_double {
    let mut n: size_t = 0;
    let mut r: libc::c_double = 0.;
    r = ::std::f64::INFINITY;
    n = 0 as libc::c_int as size_t;
    while n < 16 as libc::c_int as libc::c_ulong {
        let mut p: libc::c_double = 0.;
        p = (*q).position[n as usize];
        if !(p == ::std::f64::INFINITY) { if p > r && p < current { r = p } }
        n = n.wrapping_add(1)
    }
    return r;
}
/*
 * Return: the next cue point after the given position, or CUE_UNSET
 */
#[no_mangle]
pub unsafe extern "C" fn cues_next(mut q: *const cues,
                                   mut current: libc::c_double)
 -> libc::c_double {
    let mut n: size_t = 0;
    let mut r: libc::c_double = 0.;
    r = ::std::f64::INFINITY;
    n = 0 as libc::c_int as size_t;
    while n < 16 as libc::c_int as libc::c_ulong {
        let mut p: libc::c_double = 0.;
        p = (*q).position[n as usize];
        if !(p == ::std::f64::INFINITY) { if p < r && p > current { r = p } }
        n = n.wrapping_add(1)
    }
    return r;
}
