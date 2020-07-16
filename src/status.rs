use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
/*
 * Call the callback in all slots which are watching the given event
 */
#[inline]
unsafe extern "C" fn fire(mut s: *mut event, mut data: *mut libc::c_void) {
    let mut t: *mut observer = 0 as *mut observer;
    t =
        ((*s).observers.next as
             *mut libc::c_char).offset(-(0 as libc::c_ulong as isize)) as
            *mut observer;
    while &mut (*t).event as *mut list != &mut (*s).observers as *mut list {
        if (*t).func.is_some() {
        } else {
            __assert_fail(b"t->func != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"./observer.h\x00" as *const u8 as
                              *const libc::c_char,
                          90 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 34],
                                                    &[libc::c_char; 34]>(b"void fire(struct event *, void *)\x00")).as_ptr());
        }
        (*t).func.expect("non-null function pointer")(t, data);
        t =
            ((*t).event.next as
                 *mut libc::c_char).offset(-(0 as libc::c_ulong as isize)) as
                *mut observer
    };
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
// Initialized in run_static_initializers
#[no_mangle]
pub static mut status_changed: event =
    event{observers:
              list{prev: 0 as *const list as *mut list,
                   next: 0 as *const list as *mut list,},};
static mut message: *const libc::c_char =
    b"\x00" as *const u8 as *const libc::c_char;
static mut level: libc::c_int = 0 as libc::c_int;
/*
 * Return: current status string
 */
#[no_mangle]
pub unsafe extern "C" fn status() -> *const libc::c_char { return message; }
#[no_mangle]
pub unsafe extern "C" fn status_level() -> libc::c_int { return level; }
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
 * Implement a global one-line status console
 */
/*
 * Set status to reference a static string
 *
 * Post: reference on s is held
 */
#[no_mangle]
pub unsafe extern "C" fn status_set(mut l: libc::c_int,
                                    mut s: *const libc::c_char) {
    message = s;
    level = l;
    if l >= 1 as libc::c_int { fputs(s, stderr); fputc('\n' as i32, stderr); }
    fire(&mut status_changed, s as *mut libc::c_void);
}
/*
 * Set status to a formatted string
 */
#[no_mangle]
pub unsafe extern "C" fn status_printf(mut lvl: libc::c_int,
                                       mut t: *const libc::c_char,
                                       mut args: ...) {
    static mut buf: [libc::c_char; 256] = [0; 256];
    let mut l: ::std::ffi::VaListImpl;
    l = args.clone();
    vsnprintf(buf.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
              t, l.as_va_list());
    status_set(lvl, buf.as_mut_ptr());
}
unsafe extern "C" fn run_static_initializers() {
    status_changed =
        {
            let mut init =
                event{observers:
                          {
                              let mut init =
                                  list{prev: &mut status_changed.observers,
                                       next: &mut status_changed.observers,};
                              init
                          },};
            init
        }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
