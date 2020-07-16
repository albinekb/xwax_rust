use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn pthread_setspecific(__key: pthread_key_t,
                           __pointer: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    #[no_mangle]
    fn pthread_key_delete(__key: pthread_key_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_key_create(__key: *mut pthread_key_t,
                          __destr_function:
                              Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void)
                                         -> ()>) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn abort() -> !;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type pthread_key_t = libc::c_uint;
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
static mut key: pthread_key_t = 0;
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
/*
 * Put in place checks for realtime and non-realtime threads
 *
 * Return: 0 on success, otherwise -1
 */
#[no_mangle]
pub unsafe extern "C" fn thread_global_init() -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = pthread_key_create(&mut key, None);
    if r != 0 as libc::c_int {
        *__errno_location() = r;
        perror(b"pthread_key_create\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if pthread_setspecific(key, 0 as *const libc::c_void) != 0 as libc::c_int
       {
        abort();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn thread_global_clear() {
    if pthread_key_delete(key) != 0 as libc::c_int { abort(); };
}
/*
 * Inform that this thread is a realtime thread, for assertions later
 */
#[no_mangle]
pub unsafe extern "C" fn thread_to_realtime() {
    if pthread_setspecific(key, 1 as libc::c_int as *mut libc::c_void) !=
           0 as libc::c_int {
        abort();
    };
}
/*
 * Check for programmer error
 *
 * Pre: the current thread is non realtime
 */
#[no_mangle]
pub unsafe extern "C" fn rt_not_allowed() {
    let mut rt: bool = false;
    rt = !pthread_getspecific(key).is_null();
    if rt {
        fprintf(stderr,
                b"Realtime thread called a blocking function\n\x00" as
                    *const u8 as *const libc::c_char);
        abort();
    };
}
