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
    fn __errno_location() -> *mut libc::c_int;
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
    fn abort() -> !;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fork_pipe_nb(fd: *mut libc::c_int, path: *const libc::c_char,
                    arg: *mut libc::c_char, _: ...) -> pid_t;
    #[no_mangle]
    fn rb_reset(rb: *mut rb);
    #[no_mangle]
    fn get_line(fd: libc::c_int, rb: *mut rb, string: *mut *mut libc::c_char)
     -> ssize_t;
    #[no_mangle]
    fn listing_init(l: *mut listing);
    #[no_mangle]
    fn listing_clear(l: *mut listing);
    #[no_mangle]
    fn listing_add(l: *mut listing, r: *mut record) -> *mut record;
    #[no_mangle]
    fn get_record(line: *mut libc::c_char) -> *mut record;
    #[no_mangle]
    fn rig_post_excrate(e: *mut excrate);
    #[no_mangle]
    fn status_printf(level: libc::c_int, s: *const libc::c_char, _: ...);
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
/*
 * Utility functions for launching external processes
 */
/*
 * A handy read buffer; an equivalent of fread() but for
 * non-blocking file descriptors
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb {
    pub buf: [libc::c_char; 4096],
    pub len: size_t,
}
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
/* or 0.0 if not known */
/* Index points to records, but does not manage those pointers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index {
    pub record: *mut *mut record,
    pub size: size_t,
    pub entries: size_t,
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
 * Implementation of "observer pattern"
 *
 * There are several cases in the code where we need to notify
 * when something changes (eg. to update a UI.)
 *
 * The use of simple function calls is problematic because it creates
 * cyclical dependencies in header files, and is not sufficiently
 * modular to allow the code to be re-used in a self-contained test.
 *
 * So, reluctantly introduce a slots and signals concept; xwax is
 * getting to be quite a lot of code and structure now.
 */
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
/* A set of records, with several optimised indexes */
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
/*
 * Insert a new list entry after the given head
 */
/*
 * Insert a new list entry before the given head (ie. end of list)
 */
#[inline]
unsafe extern "C" fn list_del(mut entry: *mut list) {
    let mut next: *mut list = 0 as *mut list;
    let mut prev: *mut list = 0 as *mut list;
    next = (*entry).next;
    prev = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
}
#[inline]
unsafe extern "C" fn list_init(mut list: *mut list) {
    (*list).next = list;
    (*list).prev = list;
}
#[inline]
unsafe extern "C" fn __list_add(mut new: *mut list, mut prev: *mut list,
                                mut next: *mut list) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}
#[inline]
unsafe extern "C" fn list_add(mut new: *mut list, mut head: *mut list) {
    __list_add(new, head, (*head).next);
}
#[inline]
unsafe extern "C" fn event_init(mut s: *mut event) {
    list_init(&mut (*s).observers);
}
#[inline]
unsafe extern "C" fn event_clear(mut s: *mut event) {
    if list_empty(&mut (*s).observers) {
    } else {
        __assert_fail(b"list_empty(&s->observers)\x00" as *const u8 as
                          *const libc::c_char,
                      b"./observer.h\x00" as *const u8 as *const libc::c_char,
                      61 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void event_clear(struct event *)\x00")).as_ptr());
    };
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
#[inline]
unsafe extern "C" fn list_empty(mut head: *const list) -> bool {
    return (*head).next == head as *mut list;
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
 * External record library ('excrate')
 *
 * Implement search in an external script. The results are streamed
 * back into a local listing.
 */
static mut excrates: list =
    unsafe {
        {
            let mut init =
                list{prev: &excrates as *const list as *mut list,
                     next: &excrates as *const list as *mut list,};
            init
        }
    };
unsafe extern "C" fn excrate_init(mut e: *mut excrate,
                                  mut script: *const libc::c_char,
                                  mut search: *const libc::c_char,
                                  mut storage: *mut listing) -> libc::c_int {
    let mut pid: pid_t = 0;
    fprintf(stderr,
            b"External scan \'%s\'...\n\x00" as *const u8 as
                *const libc::c_char, search);
    pid =
        fork_pipe_nb(&mut (*e).fd as *mut libc::c_int, script,
                     b"scan\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, search, 0 as *mut libc::c_void);
    if pid == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    (*e).pid = pid;
    (*e).pe = 0 as *mut pollfd;
    (*e).terminated = 0 as libc::c_int != 0;
    (*e).refcount = 0 as libc::c_int as libc::c_uint;
    rb_reset(&mut (*e).rb);
    listing_init(&mut (*e).listing);
    (*e).storage = storage;
    event_init(&mut (*e).completion);
    (*e).search = search;
    list_add(&mut (*e).excrates, &mut excrates);
    rig_post_excrate(e);
    return 0 as libc::c_int;
}
unsafe extern "C" fn excrate_clear(mut e: *mut excrate) {
    if (*e).pid == 0 as libc::c_int {
    } else {
        __assert_fail(b"e->pid == 0\x00" as *const u8 as *const libc::c_char,
                      b"excrate.c\x00" as *const u8 as *const libc::c_char,
                      69 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void excrate_clear(struct excrate *)\x00")).as_ptr());
    }
    list_del(&mut (*e).excrates);
    listing_clear(&mut (*e).listing);
    event_clear(&mut (*e).completion);
}
#[no_mangle]
pub unsafe extern "C" fn excrate_acquire_by_scan(mut script:
                                                     *const libc::c_char,
                                                 mut search:
                                                     *const libc::c_char,
                                                 mut storage: *mut listing)
 -> *mut excrate {
    let mut e: *mut excrate = 0 as *mut excrate;
    e =
        malloc(::std::mem::size_of::<excrate>() as libc::c_ulong) as
            *mut excrate;
    if e.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut excrate
    }
    if excrate_init(e, script, search, storage) == -(1 as libc::c_int) {
        free(e as *mut libc::c_void);
        return 0 as *mut excrate
    }
    excrate_acquire(e);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn excrate_acquire(mut e: *mut excrate) {
    (*e).refcount = (*e).refcount.wrapping_add(1);
}
/*
 * Request premature termination of the scan
 */
unsafe extern "C" fn terminate(mut e: *mut excrate) {
    if (*e).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"e->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"excrate.c\x00" as *const u8 as *const libc::c_char,
                      111 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void terminate(struct excrate *)\x00")).as_ptr());
    }
    if kill((*e).pid, 15 as libc::c_int) == -(1 as libc::c_int) { abort(); }
    (*e).terminated = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn excrate_release(mut e: *mut excrate) {
    (*e).refcount = (*e).refcount.wrapping_sub(1);
    /* Scan must terminate before this object goes away */
    if (*e).refcount == 1 as libc::c_int as libc::c_uint &&
           (*e).pid != 0 as libc::c_int {
        terminate(e);
        return
    }
    if (*e).refcount == 0 as libc::c_int as libc::c_uint {
        excrate_clear(e);
        free(e as *mut libc::c_void);
    };
}
/*
 * Get entry for use by poll()
 *
 * Pre: scan is running
 * Post: *pe contains poll entry
 */
#[no_mangle]
pub unsafe extern "C" fn excrate_pollfd(mut e: *mut excrate,
                                        mut pe: *mut pollfd) {
    if (*e).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"e->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"excrate.c\x00" as *const u8 as *const libc::c_char,
                      148 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void excrate_pollfd(struct excrate *, struct pollfd *)\x00")).as_ptr());
    }
    (*pe).fd = (*e).fd;
    (*pe).events = 0x1 as libc::c_int as libc::c_short;
    (*e).pe = pe;
}
unsafe extern "C" fn do_wait(mut e: *mut excrate) {
    let mut status: libc::c_int = 0;
    if (*e).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"e->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"excrate.c\x00" as *const u8 as *const libc::c_char,
                      160 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void do_wait(struct excrate *)\x00")).as_ptr());
    }
    if close((*e).fd) == -(1 as libc::c_int) { abort(); }
    if waitpid((*e).pid, &mut status, 0 as libc::c_int) == -(1 as libc::c_int)
       {
        abort();
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int &&
           (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
               0 as libc::c_int {
        fprintf(stderr,
                b"Scan completed\n\x00" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stderr,
                b"Scan completed with status %d\n\x00" as *const u8 as
                    *const libc::c_char, status);
        if !(*e).terminated {
            status_printf(3 as libc::c_int,
                          b"Error scanning %s\x00" as *const u8 as
                              *const libc::c_char, (*e).search);
        }
    }
    (*e).pid = 0 as libc::c_int;
}
/*
 * Return: -1 on completion, otherwise zero
 */
unsafe extern "C" fn read_from_pipe(mut e: *mut excrate) -> libc::c_int {
    loop  {
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut z: ssize_t = 0;
        let mut d: *mut record = 0 as *mut record;
        let mut x: *mut record = 0 as *mut record;
        z = get_line((*e).fd, &mut (*e).rb, &mut line);
        if z == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() == 11 as libc::c_int {
                return 0 as libc::c_int
            }
            perror(b"get_line\x00" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        if z == 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int)
        }
        d = get_record(line);
        if d.is_null() {
            free(line as *mut libc::c_void);
            /* ignore malformed entries */
        } else {
            x = listing_add((*e).storage, d);
            if x.is_null() { return -(1 as libc::c_int) }
            if x != d {
                /* our new record is a duplicate */
                free(d as *mut libc::c_void);
            }
            x = listing_add(&mut (*e).listing, x);
            if x.is_null() { return -(1 as libc::c_int) }
        }
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
/* State of the external scan process */
/* State of reader */
/* Used by the rig and main thread */
#[no_mangle]
pub unsafe extern "C" fn excrate_handle(mut e: *mut excrate) {
    if (*e).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"e->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"excrate.c\x00" as *const u8 as *const libc::c_char,
                      226 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"void excrate_handle(struct excrate *)\x00")).as_ptr());
    }
    if (*e).pe.is_null() { return }
    if (*(*e).pe).revents as libc::c_int == 0 as libc::c_int { return }
    if read_from_pipe(e) != -(1 as libc::c_int) { return }
    do_wait(e);
    fire(&mut (*e).completion, 0 as *mut libc::c_void);
    list_del(&mut (*e).rig);
    excrate_release(e);
    /* may invalidate e */
}
