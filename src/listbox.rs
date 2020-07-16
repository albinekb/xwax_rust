use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
 * Generic UI listbox widget internals
 */
/* Managed context of a scrolling window, of a number of fixed-height
 * lines, backed by a list of a known number of entries */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listbox {
    pub entries: libc::c_int,
    pub lines: libc::c_int,
    pub offset: libc::c_int,
    pub selected: libc::c_int,
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
pub unsafe extern "C" fn listbox_init(mut s: *mut listbox) {
    (*s).lines = 0 as libc::c_int;
    (*s).entries = 0 as libc::c_int;
    (*s).offset = 0 as libc::c_int;
    (*s).selected = -(1 as libc::c_int);
}
/*
 * Set the number of lines displayed on screen
 *
 * Zero is valid, to mean that the display is hidden. In all other
 * cases, the current selection is moved to within range.
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_set_lines(mut s: *mut listbox,
                                           mut lines: libc::c_uint) {
    (*s).lines = lines as libc::c_int;
    if (*s).selected >= (*s).offset + (*s).lines {
        (*s).selected = (*s).offset + (*s).lines - 1 as libc::c_int
    }
    if (*s).offset + (*s).lines > (*s).entries {
        (*s).offset = (*s).entries - (*s).lines;
        if (*s).offset < 0 as libc::c_int { (*s).offset = 0 as libc::c_int }
    };
}
/*
 * Set the number of entries in the list which backs the scrolling
 * display. Bring the current selection within the bounds given.
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_set_entries(mut s: *mut listbox,
                                             mut entries: libc::c_uint) {
    (*s).entries = entries as libc::c_int;
    if (*s).selected >= (*s).entries {
        (*s).selected = (*s).entries - 1 as libc::c_int
    }
    if (*s).offset + (*s).lines > (*s).entries {
        (*s).offset = (*s).entries - (*s).lines;
        if (*s).offset < 0 as libc::c_int { (*s).offset = 0 as libc::c_int }
    }
    /* If we went previously had zero entries, reset the selection */
    if (*s).selected < 0 as libc::c_int { (*s).selected = 0 as libc::c_int };
}
/* Scroll functions */
/*
 * Scroll the selection up by n lines. Move the window offset if
 * needed
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_up(mut s: *mut listbox,
                                    mut n: libc::c_uint) {
    (*s).selected =
        ((*s).selected as libc::c_uint).wrapping_sub(n) as libc::c_int as
            libc::c_int;
    if (*s).selected < 0 as libc::c_int { (*s).selected = 0 as libc::c_int }
    /* Move the viewing offset up, if necessary */
    if (*s).selected < (*s).offset {
        (*s).offset =
            (*s).selected + (*s).lines / 2 as libc::c_int - (*s).lines +
                1 as libc::c_int;
        if (*s).offset < 0 as libc::c_int { (*s).offset = 0 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn listbox_down(mut s: *mut listbox,
                                      mut n: libc::c_uint) {
    (*s).selected =
        ((*s).selected as libc::c_uint).wrapping_add(n) as libc::c_int as
            libc::c_int;
    if (*s).selected >= (*s).entries {
        (*s).selected = (*s).entries - 1 as libc::c_int
    }
    /* Move the viewing offset down, if necessary */
    if (*s).selected >= (*s).offset + (*s).lines {
        (*s).offset = (*s).selected - (*s).lines / 2 as libc::c_int;
        if (*s).offset + (*s).lines > (*s).entries {
            (*s).offset = (*s).entries - (*s).lines
        }
    };
}
/*
 * Scroll to the first entry on the list
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_first(mut s: *mut listbox) {
    (*s).selected = 0 as libc::c_int;
    (*s).offset = 0 as libc::c_int;
}
/*
 * Scroll to the final entry on the list
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_last(mut s: *mut listbox) {
    (*s).selected = (*s).entries - 1 as libc::c_int;
    (*s).offset = (*s).selected - (*s).lines + 1 as libc::c_int;
    if (*s).offset < 0 as libc::c_int { (*s).offset = 0 as libc::c_int };
}
/*
 * Scroll to an entry by index
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_to(mut s: *mut listbox,
                                    mut n: libc::c_uint) {
    let mut p: libc::c_int = 0;
    if (*s).selected != -(1 as libc::c_int) {
    } else {
        __assert_fail(b"s->selected != -1\x00" as *const u8 as
                          *const libc::c_char,
                      b"listbox.c\x00" as *const u8 as *const libc::c_char,
                      145 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void listbox_to(struct listbox *, unsigned int)\x00")).as_ptr());
    }
    if n < (*s).entries as libc::c_uint {
    } else {
        __assert_fail(b"n < s->entries\x00" as *const u8 as
                          *const libc::c_char,
                      b"listbox.c\x00" as *const u8 as *const libc::c_char,
                      146 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void listbox_to(struct listbox *, unsigned int)\x00")).as_ptr());
    }
    /* Retain the on-screen position of the current selection */
    p = (*s).selected - (*s).offset;
    (*s).selected = n as libc::c_int;
    (*s).offset = (*s).selected - p;
    if (*s).offset < 0 as libc::c_int { (*s).offset = 0 as libc::c_int };
}
/*
 * Return the index of the current selected list entry, or -1 if
 * no current selection
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_current(mut s: *const listbox)
 -> libc::c_int {
    if (*s).entries == 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else { return (*s).selected };
}
/*
 * Translate the given row on-screen (starting with row zero)
 * into a position in the list
 *
 * Return: -1 if the row is out of range, otherwise entry number
 */
#[no_mangle]
pub unsafe extern "C" fn listbox_map(mut s: *const listbox,
                                     mut row: libc::c_int) -> libc::c_int {
    let mut e: libc::c_int = 0;
    if row >= 0 as libc::c_int {
    } else {
        __assert_fail(b"row >= 0\x00" as *const u8 as *const libc::c_char,
                      b"listbox.c\x00" as *const u8 as *const libc::c_char,
                      182 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"int listbox_map(const struct listbox *, int)\x00")).as_ptr());
    }
    if row >= (*s).lines { return -(1 as libc::c_int) }
    e = (*s).offset + row;
    if e >= (*s).entries { return -(1 as libc::c_int) }
    return e;
}
