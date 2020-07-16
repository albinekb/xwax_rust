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
    fn iconv_open(__tocode: *const libc::c_char,
                  __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn iconv(__cd: iconv_t, __inbuf: *mut *mut libc::c_char,
             __inbytesleft: *mut size_t, __outbuf: *mut *mut libc::c_char,
             __outbytesleft: *mut size_t) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __finitel(_: f64) -> libc::c_int;
    #[no_mangle]
    fn __finitef(_: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn __finite(_: libc::c_double) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn excrate_acquire_by_scan(script: *const libc::c_char,
                               search: *const libc::c_char,
                               storage: *mut listing) -> *mut excrate;
    /* NULL-terminated array */
    #[no_mangle]
    fn index_init(ls: *mut index);
    #[no_mangle]
    fn index_clear(ls: *mut index);
    #[no_mangle]
    fn index_add(li: *mut index, lr: *mut record);
    #[no_mangle]
    fn index_insert(ls: *mut index, item: *mut record, sort: libc::c_int)
     -> *mut record;
    #[no_mangle]
    fn index_reserve(i: *mut index, n: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn excrate_release(e: *mut excrate);
}
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
pub struct crate_0 {
    pub is_fixed: bool,
    pub is_busy: bool,
    pub name: *mut libc::c_char,
    pub listing: *mut listing,
    pub on_addition: observer,
    pub on_completion: observer,
    pub activity: event,
    pub refresh: event,
    pub addition: event,
    pub scan: *const libc::c_char,
    pub path: *const libc::c_char,
    pub excrate: *mut excrate,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct library {
    pub storage: listing,
    pub all: crate_0,
    pub crate_0: *mut *mut crate_0,
    pub crates: size_t,
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
/*
 * Insert a new list entry after the given head
 */
#[inline]
unsafe extern "C" fn list_add(mut new: *mut list, mut head: *mut list) {
    __list_add(new, head, (*head).next);
}
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
unsafe extern "C" fn list_empty(mut head: *const list) -> bool {
    return (*head).next == head as *mut list;
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
 * Pre: observer is not watching anything
 * Post: observer is watching the given event
 */
#[inline]
unsafe extern "C" fn watch(mut observer: *mut observer, mut sig: *mut event,
                           mut func:
                               Option<unsafe extern "C" fn(_: *mut observer,
                                                           _:
                                                               *mut libc::c_void)
                                          -> ()>) {
    list_add(&mut (*observer).event, &mut (*sig).observers);
    (*observer).func = func;
}
#[inline]
unsafe extern "C" fn ignore(mut observer: *mut observer) {
    list_del(&mut (*observer).event);
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
/* The locale used for searches */
static mut ascii: iconv_t = unsafe { -(1 as libc::c_int) as iconv_t };
#[no_mangle]
pub unsafe extern "C" fn library_global_init() -> libc::c_int {
    if ascii == -(1 as libc::c_int) as iconv_t {
    } else {
        __assert_fail(b"ascii == (iconv_t)-1\x00" as *const u8 as
                          *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      44 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"int library_global_init(void)\x00")).as_ptr());
    }
    ascii =
        iconv_open(b"ASCII//TRANSLIT\x00" as *const u8 as *const libc::c_char,
                   b"\x00" as *const u8 as *const libc::c_char);
    if ascii == -(1 as libc::c_int) as iconv_t {
        perror(b"iconv_open\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn library_global_clear() {
    if ascii != -(1 as libc::c_int) as iconv_t {
    } else {
        __assert_fail(b"ascii != (iconv_t)-1\x00" as *const u8 as
                          *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      57 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void library_global_clear(void)\x00")).as_ptr());
    }
    if iconv_close(ascii) == -(1 as libc::c_int) { abort(); };
}
#[no_mangle]
pub unsafe extern "C" fn listing_init(mut l: *mut listing) {
    index_init(&mut (*l).by_artist);
    index_init(&mut (*l).by_bpm);
    index_init(&mut (*l).by_order);
    event_init(&mut (*l).addition);
}
#[no_mangle]
pub unsafe extern "C" fn listing_clear(mut l: *mut listing) {
    index_clear(&mut (*l).by_artist);
    index_clear(&mut (*l).by_bpm);
    index_clear(&mut (*l).by_order);
    event_clear(&mut (*l).addition);
}
/*
 * Base initialiser for a crate, shared by the other init functions
 *
 * Note the deep copy of the crate name
 *
 * Return: 0 on success or -1 on memory allocation failure
 */
unsafe extern "C" fn crate_init(mut c: *mut crate_0,
                                mut name: *const libc::c_char)
 -> libc::c_int {
    (*c).name = strdup(name);
    if (*c).name.is_null() {
        perror(b"strdup\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    (*c).is_busy = 0 as libc::c_int != 0;
    event_init(&mut (*c).activity);
    event_init(&mut (*c).refresh);
    event_init(&mut (*c).addition);
    return 0 as libc::c_int;
}
/*
 * Propagate an addition event on the listing upwards -- as an
 * addition event on this crate
 */
unsafe extern "C" fn propagate_addition(mut o: *mut observer,
                                        mut x: *mut libc::c_void) {
    let mut c: *mut crate_0 =
        (o as *mut libc::c_char).offset(-(24 as libc::c_ulong as isize)) as
            *mut crate_0;
    fire(&mut (*c).addition, x);
}
/*
 * Propagate notification that the scan has finished
 */
unsafe extern "C" fn propagate_completion(mut o: *mut observer,
                                          mut x: *mut libc::c_void) {
    let mut c: *mut crate_0 =
        (o as *mut libc::c_char).offset(-(48 as libc::c_ulong as isize)) as
            *mut crate_0;
    (*c).is_busy = 0 as libc::c_int != 0;
    fire(&mut (*c).activity, 0 as *mut libc::c_void);
}
/*
 * Initialise the crate which shows the entire library content
 *
 * Return: 0 on success, -1 on memory allocation failure
 */
unsafe extern "C" fn crate_init_all(mut l: *mut library, mut c: *mut crate_0,
                                    mut name: *const libc::c_char)
 -> libc::c_int {
    if crate_init(c, name) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    (*c).is_fixed = 1 as libc::c_int != 0;
    (*c).listing = &mut (*l).storage;
    watch(&mut (*c).on_addition, &mut (*(*c).listing).addition,
          Some(propagate_addition as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
    (*c).excrate = 0 as *mut excrate;
    return 0 as libc::c_int;
}
/*
 * Wire in the excrate to this crate, including events
 */
unsafe extern "C" fn hook_up_excrate(mut c: *mut crate_0,
                                     mut e: *mut excrate) {
    if !(*c).is_busy {
        (*c).is_busy = 1 as libc::c_int != 0;
        fire(&mut (*c).activity, 0 as *mut libc::c_void);
    }
    (*c).excrate = e;
    (*c).listing = &mut (*e).listing;
    fire(&mut (*c).refresh, 0 as *mut libc::c_void);
    watch(&mut (*c).on_addition, &mut (*(*c).listing).addition,
          Some(propagate_addition as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
    watch(&mut (*c).on_completion, &mut (*e).completion,
          Some(propagate_completion as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
}
/*
 * Initialise a crate which has a fixed scan as its source
 *
 * Not all crates have a source (eg. 'all' crate.) This is also
 * convenient as in future there may be other sources such as virtual
 * crates or external searches.
 *
 * Return: 0 on success or -1 on error
 */
unsafe extern "C" fn crate_init_scan(mut l: *mut library, mut c: *mut crate_0,
                                     mut name: *const libc::c_char,
                                     mut scan: *const libc::c_char,
                                     mut path: *const libc::c_char)
 -> libc::c_int {
    let mut e: *mut excrate = 0 as *mut excrate;
    if crate_init(c, name) == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    (*c).is_fixed = 0 as libc::c_int != 0;
    (*c).scan = scan;
    (*c).path = path;
    e = excrate_acquire_by_scan(scan, path, &mut (*l).storage);
    if e.is_null() { return -(1 as libc::c_int) }
    hook_up_excrate(c, e);
    return 0 as libc::c_int;
}
/*
 * Re-run a crate which has a scan as its source
 *
 * Return: 0 on success, -1 on error
 */
unsafe extern "C" fn crate_rescan(mut c: *mut crate_0, mut l: *mut library)
 -> libc::c_int {
    let mut e: *mut excrate = 0 as *mut excrate;
    if !(*c).excrate.is_null() {
    } else {
        __assert_fail(b"c->excrate != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      205 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"int crate_rescan(struct crate *, struct library *)\x00")).as_ptr());
    }
    /* Replace the excrate in-place. Care needed to re-wire
     * everything back up again as before */
    e = excrate_acquire_by_scan((*c).scan, (*c).path, &mut (*l).storage);
    if e.is_null() { return -(1 as libc::c_int) }
    ignore(&mut (*c).on_completion);
    ignore(&mut (*c).on_addition);
    excrate_release((*c).excrate);
    hook_up_excrate(c, e);
    return 0 as libc::c_int;
}
/*
 * Deallocate resources associated with this crate
 *
 * The crate does not 'own' the memory allocated by the records
 * in it, so we don't free them here.
 */
unsafe extern "C" fn crate_clear(mut c: *mut crate_0) {
    ignore(&mut (*c).on_addition);
    if !(*c).excrate.is_null() {
        ignore(&mut (*c).on_completion);
        excrate_release((*c).excrate);
    }
    event_clear(&mut (*c).activity);
    event_clear(&mut (*c).refresh);
    event_clear(&mut (*c).addition);
    free((*c).name as *mut libc::c_void);
}
/*
 * Comparison function for two crates
 */
unsafe extern "C" fn crate_cmp(mut a: *const crate_0, mut b: *const crate_0)
 -> libc::c_int {
    if (*a).is_fixed as libc::c_int != 0 && !(*b).is_fixed {
        return -(1 as libc::c_int)
    }
    if !(*a).is_fixed && (*b).is_fixed as libc::c_int != 0 {
        return 1 as libc::c_int
    }
    return strcmp((*a).name, (*b).name);
}
/*
 * Add a record into a crate and its various indexes
 *
 * Return: Pointer to existing entry, NULL if out of memory
 * Post: Record added to the crate
 */
#[no_mangle]
pub unsafe extern "C" fn listing_add(mut l: *mut listing, mut r: *mut record)
 -> *mut record {
    let mut x: *mut record = 0 as *mut record;
    if !r.is_null() {
    } else {
        __assert_fail(b"r != NULL\x00" as *const u8 as *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      269 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"struct record *listing_add(struct listing *, struct record *)\x00")).as_ptr());
    }
    /* Do all the memory reservation up-front as we can't
     * un-wind if it errors later */
    if index_reserve(&mut (*l).by_artist, 1 as libc::c_int as libc::c_uint) ==
           -(1 as libc::c_int) {
        return 0 as *mut record
    }
    if index_reserve(&mut (*l).by_bpm, 1 as libc::c_int as libc::c_uint) ==
           -(1 as libc::c_int) {
        return 0 as *mut record
    }
    if index_reserve(&mut (*l).by_order, 1 as libc::c_int as libc::c_uint) ==
           -(1 as libc::c_int) {
        return 0 as *mut record
    }
    x = index_insert(&mut (*l).by_artist, r, 0 as libc::c_int);
    if !x.is_null() {
    } else {
        __assert_fail(b"x != NULL\x00" as *const u8 as *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      282 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"struct record *listing_add(struct listing *, struct record *)\x00")).as_ptr());
    }
    if x != r { return x }
    x = index_insert(&mut (*l).by_bpm, r, 1 as libc::c_int);
    if x == r {
    } else {
        __assert_fail(b"x == r\x00" as *const u8 as *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      287 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"struct record *listing_add(struct listing *, struct record *)\x00")).as_ptr());
    }
    index_add(&mut (*l).by_order, r);
    fire(&mut (*l).addition, r as *mut libc::c_void);
    return r;
}
/*
 * Comparison function, see qsort(3)
 */
unsafe extern "C" fn qcompar(mut a: *const libc::c_void,
                             mut b: *const libc::c_void) -> libc::c_int {
    return crate_cmp(*(a as *mut *mut crate_0), *(b as *mut *mut crate_0));
}
/*
 * Sort all crates into a defined order
 */
unsafe extern "C" fn sort_crates(mut lib: *mut library) {
    qsort((*lib).crate_0 as *mut libc::c_void, (*lib).crates,
          ::std::mem::size_of::<*mut crate_0>() as libc::c_ulong,
          Some(qcompar as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
}
/*
 * Add a crate to the list of all crates
 *
 * Return: 0 on success or -1 on memory allocation failure
 */
unsafe extern "C" fn add_crate(mut lib: *mut library, mut c: *mut crate_0)
 -> libc::c_int {
    let mut cn: *mut *mut crate_0 = 0 as *mut *mut crate_0;
    cn =
        realloc((*lib).crate_0 as *mut libc::c_void,
                (::std::mem::size_of::<*mut crate_0>() as
                     libc::c_ulong).wrapping_mul((*lib).crates.wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)))
            as *mut *mut crate_0;
    if cn.is_null() {
        perror(b"realloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    (*lib).crate_0 = cn;
    let fresh0 = (*lib).crates;
    (*lib).crates = (*lib).crates.wrapping_add(1);
    let ref mut fresh1 = *(*lib).crate_0.offset(fresh0 as isize);
    *fresh1 = c;
    sort_crates(lib);
    return 0 as libc::c_int;
}
/*
 * Get a crate by the given name
 *
 * Beware: The match could match the fixed crates if the name is the
 * same.
 *
 * Return: pointer to crate, or NULL if no crate has the given name
 */
#[no_mangle]
pub unsafe extern "C" fn get_crate(mut lib: *mut library,
                                   mut name: *const libc::c_char)
 -> *mut crate_0 {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while (n as libc::c_ulong) < (*lib).crates {
        if strcmp((**(*lib).crate_0.offset(n as isize)).name, name) ==
               0 as libc::c_int {
            return *(*lib).crate_0.offset(n as isize)
        }
        n += 1
    }
    return 0 as *mut crate_0;
}
/*
 * Initialise the record library
 *
 * Return: 0 on success or -1 on memory allocation failure
 */
#[no_mangle]
pub unsafe extern "C" fn library_init(mut li: *mut library) -> libc::c_int {
    (*li).crate_0 = 0 as *mut *mut crate_0;
    (*li).crates = 0 as libc::c_int as size_t;
    listing_init(&mut (*li).storage);
    if crate_init_all(li, &mut (*li).all,
                      b"All records\x00" as *const u8 as *const libc::c_char)
           == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    if add_crate(li, &mut (*li).all) == -(1 as libc::c_int) {
        crate_clear(&mut (*li).all);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Free resources associated with a record
 */
unsafe extern "C" fn record_clear(mut re: *mut record) {
    free((*re).pathname as *mut libc::c_void);
    free((*re).match_0 as *mut libc::c_void);
    /* may be NULL */
}
/*
 * Free resources associated with the music library
 */
#[no_mangle]
pub unsafe extern "C" fn library_clear(mut li: *mut library) {
    let mut n: libc::c_int = 0;
    /* This object is responsible for all the record pointers */
    n = 0 as libc::c_int;
    while (n as libc::c_ulong) < (*li).storage.by_artist.entries {
        let mut re: *mut record = 0 as *mut record;
        re = *(*li).storage.by_artist.record.offset(n as isize);
        record_clear(re);
        free(re as *mut libc::c_void);
        n += 1
    }
    /* Clear crates */
    n = 1 as libc::c_int;
    while (n as libc::c_ulong) < (*li).crates {
        /* skip the 'all' crate */
        let mut crate_0: *mut crate_0 = 0 as *mut crate_0;
        crate_0 = *(*li).crate_0.offset(n as isize);
        crate_clear(crate_0);
        free(crate_0 as *mut libc::c_void);
        n += 1
    }
    free((*li).crate_0 as *mut libc::c_void);
    crate_clear(&mut (*li).all);
    listing_clear(&mut (*li).storage);
}
/*
 * Translate a string from the scan to our internal BPM value
 *
 * Return: internal BPM value, or INFINITY if string is malformed
 */
unsafe extern "C" fn parse_bpm(mut s: *const libc::c_char) -> libc::c_double {
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bpm: libc::c_double = 0.;
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
        /* empty string, valid for 'unknown BPM' */
        return 0.0f64
    }
    *__errno_location() = 0 as libc::c_int;
    bpm = strtod(s, &mut endptr);
    if *__errno_location() == 34 as libc::c_int ||
           *endptr as libc::c_int != '\u{0}' as i32 ||
           (if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                   ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
                __finitef(bpm as libc::c_float)
            } else {
                (if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                        ==
                        ::std::mem::size_of::<libc::c_double>() as
                            libc::c_ulong {
                     __finite(bpm)
                 } else { __finitel(bpm as f64) })
            }) == 0 || bpm <= 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    }
    return bpm;
}
/*
 * Split string into array of fields (destructive)
 *
 * Return: number of fields found
 * Post: array x is filled with n values
 */
unsafe extern "C" fn split(mut s: *mut libc::c_char,
                           mut x: *mut *mut libc::c_char, mut len: size_t)
 -> size_t {
    let mut n: size_t = 0;
    n = 0 as libc::c_int as size_t;
    while n < len {
        let mut y: *mut libc::c_char = 0 as *mut libc::c_char;
        y = strchr(s, '\t' as i32);
        if y.is_null() {
            let ref mut fresh2 = *x.offset(n as isize);
            *fresh2 = s;
            return n.wrapping_add(1 as libc::c_int as libc::c_ulong)
        }
        *y = '\u{0}' as i32 as libc::c_char;
        let ref mut fresh3 = *x.offset(n as isize);
        *fresh3 = s;
        s = y.offset(1 as libc::c_int as isize);
        n = n.wrapping_add(1)
    }
    return n;
}
/*
 * Construct a string for matching against
 *
 * Only construct the string if the given "artist" and "title" contain
 * characters which require converting to the ASCII locale which is
 * used for searches.
 *
 * Return: string with responsibility, or NULL if not required
 */
unsafe extern "C" fn matchable(mut artist: *const libc::c_char,
                               mut title: *const libc::c_char)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut fill: size_t = 0;
    let mut nonrev: size_t = 0;
    /*
     * Append all the strings of interest into a single buffer
     */
    len =
        strlen(artist).wrapping_add(strlen(title)).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong); /* include \0 terminator */
    let mut fresh4 =
        ::std::vec::from_elem(0,
                              len.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong) as usize);
    buf = fresh4.as_mut_ptr() as *mut libc::c_char;
    if !buf.is_null() {
    } else {
        __assert_fail(b"buf\x00" as *const u8 as *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      496 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"char *matchable(const char *, const char *)\x00")).as_ptr());
    }
    sprintf(buf, b"%s %s\x00" as *const u8 as *const libc::c_char, artist,
            title);
    /*
     * Perform iconv
     *
     * We know ASCII is the shorter encoding, so we can use the input
     * buffer as also the output buffer
     */
    out = buf; /* does not include \0 */
    fill = len;
    if ascii != -(1 as libc::c_int) as iconv_t {
    } else {
        __assert_fail(b"ascii != (iconv_t)-1\x00" as *const u8 as
                          *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      510 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"char *matchable(const char *, const char *)\x00")).as_ptr());
    }
    if iconv(ascii, 0 as *mut *mut libc::c_char, 0 as *mut size_t, &mut out,
             &mut fill) == -(1 as libc::c_int) as libc::c_ulong {
        abort();
    }
    in_0 = buf;
    nonrev = iconv(ascii, &mut in_0, &mut len, &mut out, &mut fill);
    *out = '\u{0}' as i32 as libc::c_char;
    if nonrev == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char
    }
    return strdup(buf);
}
/*
 * Convert a line from the scan script to a record structure in memory
 *
 * Return: pointer to alloc'd record, or NULL on error
 * Post: if successful, responsibility for pointer line is taken
 */
#[no_mangle]
pub unsafe extern "C" fn get_record(mut line: *mut libc::c_char)
 -> *mut record {
    let mut n: libc::c_int = 0;
    let mut x: *mut record = 0 as *mut record;
    let mut field: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    x =
        malloc(::std::mem::size_of::<record>() as libc::c_ulong) as
            *mut record;
    if x.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut record
    }
    (*x).bpm = 0.0f64;
    n =
        split(line, field.as_mut_ptr(),
              (::std::mem::size_of::<[*mut libc::c_char; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                   as libc::c_ulong)) as
            libc::c_int;
    match n {
        4 => {
            (*x).bpm = parse_bpm(field[3 as libc::c_int as usize]);
            if if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                      ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                  {
                   __finitef((*x).bpm as libc::c_float)
               } else if ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong ==
                             ::std::mem::size_of::<libc::c_double>() as
                                 libc::c_ulong {
                   __finite((*x).bpm)
               } else { __finitel((*x).bpm as f64) } == 0 {
                fprintf(stderr,
                        b"%s: Ignoring malformed BPM \'%s\'\n\x00" as
                            *const u8 as *const libc::c_char,
                        field[0 as libc::c_int as usize],
                        field[3 as libc::c_int as usize]);
                (*x).bpm = 0.0f64
            }
        }
        3 => { }
        2 | 1 | _ => {
            fprintf(stderr,
                    b"Malformed record \'%s\'\n\x00" as *const u8 as
                        *const libc::c_char, line);
            free(x as *mut libc::c_void);
            return 0 as *mut record
        }
    }
    /* fall-through */
    (*x).pathname = field[0 as libc::c_int as usize];
    (*x).artist = field[1 as libc::c_int as usize];
    (*x).title = field[2 as libc::c_int as usize];
    /* Decide if this record needs a character-equivalent in the
     * locale used for searching */
    (*x).match_0 = matchable((*x).artist, (*x).title);
    return x;
}
/*
 * Scan a record library
 *
 * Launch the given scan script and pass it the path argument.
 * Parse the results into the crates.
 *
 * Return: 0 on success, -1 on fatal error (may leak)
 */
#[no_mangle]
pub unsafe extern "C" fn library_import(mut li: *mut library,
                                        mut scan: *const libc::c_char,
                                        mut path: *const libc::c_char)
 -> libc::c_int {
    let mut cratename: *mut libc::c_char =
        0 as *mut libc::c_char; /* POSIX version, see basename(3) */
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut crate_0: *mut crate_0 = 0 as *mut crate_0;
    pathname =
        ({
             let mut __old: *const libc::c_char = path;
             let mut __len: size_t =
                 strlen(__old).wrapping_add(1 as libc::c_int as
                                                libc::c_ulong);
             let mut fresh5 = ::std::vec::from_elem(0, __len as usize);
             let mut __new: *mut libc::c_char =
                 fresh5.as_mut_ptr() as *mut libc::c_char;
             memcpy(__new as *mut libc::c_void, __old as *const libc::c_void,
                    __len) as *mut libc::c_char
         });
    cratename = __xpg_basename(pathname);
    if !cratename.is_null() {
    } else {
        __assert_fail(b"cratename != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"library.c\x00" as *const u8 as *const libc::c_char,
                      598 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 65],
                                                &[libc::c_char; 65]>(b"int library_import(struct library *, const char *, const char *)\x00")).as_ptr());
    }
    crate_0 =
        malloc(::std::mem::size_of::<crate_0>() as libc::c_ulong) as
            *mut crate_0;
    if crate_0.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if !(crate_init_scan(li, crate_0, cratename, scan, path) ==
             -(1 as libc::c_int)) {
        if add_crate(li, crate_0) == -(1 as libc::c_int) {
            crate_clear(crate_0);
        } else { return 0 as libc::c_int }
    }
    free(crate_0 as *mut libc::c_void);
    return -(1 as libc::c_int);
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
/* A single crate of records */
/* at the crate level, not the listing */
/* Optionally, the corresponding source */
/* The complete music library, which consists of multiple crates */
/* owns the record pointers */
/*
 * Request a rescan on the given crate
 *
 * Only crates with an external source can be rescanned, others result
 * in a no-op.
 *
 * Return: -1 if scan is not possible, otherwise 0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn library_rescan(mut l: *mut library,
                                        mut c: *mut crate_0) -> libc::c_int {
    if (*c).excrate.is_null() {
        return -(1 as libc::c_int)
    } else { return crate_rescan(c, l) };
}
