use ::libc;
extern "C" {
    pub type excrate;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn index_add(li: *mut index, lr: *mut record);
    #[no_mangle]
    fn record_match(re: *mut record, h: *const match_0) -> bool;
    #[no_mangle]
    fn index_copy(src: *const index, dest: *mut index) -> libc::c_int;
    #[no_mangle]
    fn match_compile(h: *mut match_0, d: *const libc::c_char);
    #[no_mangle]
    fn index_insert(ls: *mut index, item: *mut record, sort: libc::c_int)
     -> *mut record;
    #[no_mangle]
    fn index_reserve(i: *mut index, n: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn index_find(ls: *mut index, item: *mut record, sort: libc::c_int)
     -> size_t;
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
    /* Scroll functions */
    #[no_mangle]
    fn listbox_to(s: *mut listbox, n: libc::c_uint);
    #[no_mangle]
    fn listbox_last(s: *mut listbox);
    #[no_mangle]
    fn library_rescan(l: *mut library, c: *mut crate_0) -> libc::c_int;
    #[no_mangle]
    fn listbox_init(s: *mut listbox);
    #[no_mangle]
    fn listbox_set_lines(s: *mut listbox, lines: libc::c_uint);
    #[no_mangle]
    fn listbox_set_entries(s: *mut listbox, entries: libc::c_uint);
    #[no_mangle]
    fn listbox_up(s: *mut listbox, n: libc::c_uint);
    #[no_mangle]
    fn listbox_down(s: *mut listbox, n: libc::c_uint);
    #[no_mangle]
    fn index_init(ls: *mut index);
    #[no_mangle]
    fn listbox_current(s: *const listbox) -> libc::c_int;
    #[no_mangle]
    fn listbox_first(s: *mut listbox);
    #[no_mangle]
    fn index_match(src: *mut index, dest: *mut index, match_0: *const match_0)
     -> libc::c_int;
    #[no_mangle]
    fn index_clear(ls: *mut index);
}
pub type size_t = libc::c_ulong;
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
pub struct index {
    pub record: *mut *mut record,
    pub size: size_t,
    pub entries: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_0 {
    pub buf: [libc::c_char; 512],
    pub words: [*mut libc::c_char; 32],
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
/* A single crate of records */
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
/* The complete music library, which consists of multiple crates */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct library {
    pub storage: listing,
    pub all: crate_0,
    pub crate_0: *mut *mut crate_0,
    pub crates: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listbox {
    pub entries: libc::c_int,
    pub lines: libc::c_int,
    pub offset: libc::c_int,
    pub selected: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct selector {
    pub library: *mut library,
    pub view_index: *mut index,
    pub swap_index: *mut index,
    pub index_a: index,
    pub index_b: index,
    pub records: listbox,
    pub crates: listbox,
    pub toggled: bool,
    pub toggle_back: libc::c_int,
    pub sort: libc::c_int,
    pub target: *mut record,
    pub on_activity: observer,
    pub on_refresh: observer,
    pub on_addition: observer,
    pub search_len: size_t,
    pub search: [libc::c_char; 256],
    pub match_0: match_0,
    pub changed: event,
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
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>,
 *                    Yves Adler <yves.adler@googlemail.com>
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
 * Scroll to our target entry if it can be found, otherwise leave our
 * position unchanged
 */
unsafe extern "C" fn retain_target(mut sel: *mut selector) {
    let mut n: size_t = 0;
    let mut l: *mut index = 0 as *mut index;
    if (*sel).target.is_null() { return }
    l = (*sel).view_index;
    match (*sel).sort {
        0 | 1 => { n = index_find(l, (*sel).target, (*sel).sort) }
        2 => {
            /* Linear search */
            n = 0 as libc::c_int as size_t;
            while n < (*l).entries {
                if *(*l).record.offset(n as isize) == (*sel).target {
                    break ;
                }
                n = n.wrapping_add(1)
            }
        }
        _ => { abort(); }
    }
    if n < (*l).entries {
        listbox_to(&mut (*sel).records, n as libc::c_uint);
    };
}
/*
 * Optimised version of retain_target where our position may
 * only have moved due to insertion of a single record
 */
unsafe extern "C" fn hunt_target(mut s: *mut selector) {
    let mut l: *mut index = 0 as *mut index;
    let mut n: size_t = 0;
    if (*s).target.is_null() { return }
    l = (*s).view_index;
    n = listbox_current(&mut (*s).records) as size_t;
    if n < (*l).entries &&
           *(*l).record.offset(n.wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong) as isize) ==
               (*s).target {
        let mut x: *mut listbox = 0 as *mut listbox;
        /* Retain selection in the same position on screen
         * FIXME: listbox should provide this functionality */
        x = &mut (*s).records;
        (*x).selected += 1;
        (*x).offset += 1
    };
}
/*
 * Return: the currently selected crate
 */
unsafe extern "C" fn current_crate(mut sel: *mut selector) -> *mut crate_0 {
    let mut n: libc::c_int = 0;
    n = listbox_current(&mut (*sel).crates);
    if n != -(1 as libc::c_int) {
    } else {
        __assert_fail(b"n != -1\x00" as *const u8 as *const libc::c_char,
                      b"selector.c\x00" as *const u8 as *const libc::c_char,
                      99 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"struct crate *current_crate(struct selector *)\x00")).as_ptr());
    }
    return *(*(*sel).library).crate_0.offset(n as isize);
}
/*
 * Return the index which acts as the starting point before
 * string matching, based on the current crate
 */
unsafe extern "C" fn initial(mut sel: *mut selector) -> *mut index {
    let mut c: *mut crate_0 = 0 as *mut crate_0;
    c = current_crate(sel);
    if !c.is_null() {
    } else {
        __assert_fail(b"c != NULL\x00" as *const u8 as *const libc::c_char,
                      b"selector.c\x00" as *const u8 as *const libc::c_char,
                      114 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"struct index *initial(struct selector *)\x00")).as_ptr());
    }
    match (*sel).sort {
        0 => { return &mut (*(*c).listing).by_artist }
        1 => { return &mut (*(*c).listing).by_bpm }
        2 => { return &mut (*(*c).listing).by_order }
        _ => { abort(); }
    };
}
unsafe extern "C" fn notify(mut s: *mut selector) {
    fire(&mut (*s).changed, 0 as *mut libc::c_void);
}
/*
 * When the crate has changed, update the current index to reflect
 * the crate and the search criteria
 */
unsafe extern "C" fn do_content_change(mut sel: *mut selector) {
    index_match(initial(sel), (*sel).view_index, &mut (*sel).match_0);
    listbox_set_entries(&mut (*sel).records,
                        (*(*sel).view_index).entries as libc::c_uint);
    retain_target(sel);
    notify(sel);
}
/*
 * Callback notification that the crate has changed at the top
 * level (eg. it's gone from busy to no longer busy)
 */
unsafe extern "C" fn handle_activity(mut o: *mut observer,
                                     mut x: *mut libc::c_void) {
    let mut s: *mut selector =
        (o as *mut libc::c_char).offset(-(128 as libc::c_ulong as isize)) as
            *mut selector;
    notify(s);
}
/*
 * Callback notification that the crate has changed, including
 * removal of items
 */
unsafe extern "C" fn handle_refresh(mut o: *mut observer,
                                    mut x: *mut libc::c_void) {
    let mut s: *mut selector =
        (o as *mut libc::c_char).offset(-(152 as libc::c_ulong as isize)) as
            *mut selector;
    if x.is_null() {
    } else {
        __assert_fail(b"x == NULL\x00" as *const u8 as *const libc::c_char,
                      b"selector.c\x00" as *const u8 as *const libc::c_char,
                      166 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void handle_refresh(struct observer *, void *)\x00")).as_ptr());
    }
    do_content_change(s);
    notify(s);
}
/*
 * A new record has been added to the currently selected crate. Merge
 * this new addition into the current view, if applicable.
 */
unsafe extern "C" fn merge_addition(mut o: *mut observer,
                                    mut x: *mut libc::c_void) {
    let mut s: *mut selector =
        (o as *mut libc::c_char).offset(-(176 as libc::c_ulong as isize)) as
            *mut selector;
    let mut r: *mut record = x as *mut record;
    if !r.is_null() {
    } else {
        __assert_fail(b"r != NULL\x00" as *const u8 as *const libc::c_char,
                      b"selector.c\x00" as *const u8 as *const libc::c_char,
                      182 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void merge_addition(struct observer *, void *)\x00")).as_ptr());
    }
    if !record_match(r, &mut (*s).match_0) { return }
    /* If we're out of memory then silently drop it */
    if index_reserve((*s).view_index, 1 as libc::c_int as libc::c_uint) ==
           -(1 as libc::c_int) {
        return
    }
    if (*s).sort == 2 as libc::c_int {
        index_add((*s).view_index, r);
    } else { index_insert((*s).view_index, r, (*s).sort); }
    listbox_set_entries(&mut (*s).records,
                        (*(*s).view_index).entries as libc::c_uint);
    /* If this addition is what we've been looking for, send the
     * cursor to it (not optimal, in some cases we know the position
     * from the insert above.) Otherwise track the target one step */
    if r == (*s).target { retain_target(s); } else { hunt_target(s); }
    notify(s);
}
/*
 * Attach callbacks to the relevant crate
 *
 * So that we are notified when the crate content changes and
 * can update the GUI accordingly.
 */
unsafe extern "C" fn watch_crate(mut s: *mut selector, mut c: *mut crate_0) {
    watch(&mut (*s).on_activity, &mut (*c).activity,
          Some(handle_activity as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
    watch(&mut (*s).on_refresh, &mut (*c).refresh,
          Some(handle_refresh as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
    watch(&mut (*s).on_addition, &mut (*c).addition,
          Some(merge_addition as
                   unsafe extern "C" fn(_: *mut observer,
                                        _: *mut libc::c_void) -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn selector_init(mut sel: *mut selector,
                                       mut lib: *mut library) {
    let mut c: *mut crate_0 = 0 as *mut crate_0;
    (*sel).library = lib;
    listbox_init(&mut (*sel).records);
    listbox_init(&mut (*sel).crates);
    if (*lib).crates > 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"lib->crates > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"selector.c\x00" as *const u8 as *const libc::c_char,
                      234 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"void selector_init(struct selector *, struct library *)\x00")).as_ptr());
    }
    listbox_set_entries(&mut (*sel).crates, (*lib).crates as libc::c_uint);
    (*sel).toggled = 0 as libc::c_int != 0;
    (*sel).sort = 0 as libc::c_int;
    (*sel).search[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*sel).search_len = 0 as libc::c_int as size_t;
    (*sel).target = 0 as *mut record;
    index_init(&mut (*sel).index_a);
    index_init(&mut (*sel).index_b);
    (*sel).view_index = &mut (*sel).index_a;
    (*sel).swap_index = &mut (*sel).index_b;
    c = current_crate(sel);
    watch_crate(sel, c);
    index_copy(initial(sel), (*sel).view_index);
    listbox_set_entries(&mut (*sel).records,
                        (*(*sel).view_index).entries as libc::c_uint);
    event_init(&mut (*sel).changed);
}
#[no_mangle]
pub unsafe extern "C" fn selector_clear(mut sel: *mut selector) {
    event_clear(&mut (*sel).changed);
    ignore(&mut (*sel).on_activity);
    ignore(&mut (*sel).on_refresh);
    ignore(&mut (*sel).on_addition);
    index_clear(&mut (*sel).index_a);
    index_clear(&mut (*sel).index_b);
}
/*
 * Set the number of display lines in use
 *
 * If the selector is invisible, it must continue to exist with 1 or
 * more lines to provide a current selected crate and/or record.
 *
 * Pre: lines is greater than zero
 */
#[no_mangle]
pub unsafe extern "C" fn selector_set_lines(mut sel: *mut selector,
                                            mut lines: libc::c_uint) {
    if lines > 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"lines > 0\x00" as *const u8 as *const libc::c_char,
                      b"selector.c\x00" as *const u8 as *const libc::c_char,
                      278 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"void selector_set_lines(struct selector *, unsigned int)\x00")).as_ptr());
    }
    listbox_set_lines(&mut (*sel).crates, lines);
    listbox_set_lines(&mut (*sel).records, lines);
}
/*
 * Return: the currently selected record, or NULL if none selected
 */
#[no_mangle]
pub unsafe extern "C" fn selector_current(mut sel: *mut selector)
 -> *mut record {
    let mut i: libc::c_int = 0;
    i = listbox_current(&mut (*sel).records);
    if i == -(1 as libc::c_int) {
        return 0 as *mut record
    } else { return *(*(*sel).view_index).record.offset(i as isize) };
}
/*
 * Make a note of the current selected record, and make it the
 * position we will try and retain if the crate is changed etc.
 */
unsafe extern "C" fn set_target(mut sel: *mut selector) {
    let mut x: *mut record = 0 as *mut record;
    x = selector_current(sel);
    if !x.is_null() { (*sel).target = x };
}
#[no_mangle]
pub unsafe extern "C" fn selector_up(mut sel: *mut selector) {
    listbox_up(&mut (*sel).records, 1 as libc::c_int as libc::c_uint);
    set_target(sel);
    notify(sel);
}
#[no_mangle]
pub unsafe extern "C" fn selector_down(mut sel: *mut selector) {
    listbox_down(&mut (*sel).records, 1 as libc::c_int as libc::c_uint);
    set_target(sel);
    notify(sel);
}
#[no_mangle]
pub unsafe extern "C" fn selector_page_up(mut sel: *mut selector) {
    listbox_up(&mut (*sel).records, (*sel).records.lines as libc::c_uint);
    set_target(sel);
    notify(sel);
}
#[no_mangle]
pub unsafe extern "C" fn selector_page_down(mut sel: *mut selector) {
    listbox_down(&mut (*sel).records, (*sel).records.lines as libc::c_uint);
    set_target(sel);
    notify(sel);
}
#[no_mangle]
pub unsafe extern "C" fn selector_top(mut sel: *mut selector) {
    listbox_first(&mut (*sel).records);
    set_target(sel);
    notify(sel);
}
#[no_mangle]
pub unsafe extern "C" fn selector_bottom(mut sel: *mut selector) {
    listbox_last(&mut (*sel).records);
    set_target(sel);
    notify(sel);
}
/*
 * Helper function when we have switched crate
 */
unsafe extern "C" fn do_crate_change(mut sel: *mut selector) {
    let mut c: *mut crate_0 = 0 as *mut crate_0;
    c = current_crate(sel);
    ignore(&mut (*sel).on_activity);
    ignore(&mut (*sel).on_refresh);
    ignore(&mut (*sel).on_addition);
    watch_crate(sel, c);
    do_content_change(sel);
}
#[no_mangle]
pub unsafe extern "C" fn selector_prev(mut sel: *mut selector) {
    listbox_up(&mut (*sel).crates, 1 as libc::c_int as libc::c_uint);
    (*sel).toggled = 0 as libc::c_int != 0;
    do_crate_change(sel);
}
#[no_mangle]
pub unsafe extern "C" fn selector_next(mut sel: *mut selector) {
    listbox_down(&mut (*sel).crates, 1 as libc::c_int as libc::c_uint);
    (*sel).toggled = 0 as libc::c_int != 0;
    do_crate_change(sel);
}
/*
 * Toggle between the current crate and the 'all' crate
 */
#[no_mangle]
pub unsafe extern "C" fn selector_toggle(mut sel: *mut selector) {
    if !(*sel).toggled {
        (*sel).toggle_back = listbox_current(&mut (*sel).crates);
        listbox_first(&mut (*sel).crates);
        (*sel).toggled = 1 as libc::c_int != 0
    } else {
        listbox_to(&mut (*sel).crates, (*sel).toggle_back as libc::c_uint);
        (*sel).toggled = 0 as libc::c_int != 0
    }
    do_crate_change(sel);
}
/*
 * Toggle between sort order
 */
#[no_mangle]
pub unsafe extern "C" fn selector_toggle_order(mut sel: *mut selector) {
    set_target(sel);
    (*sel).sort = ((*sel).sort + 1 as libc::c_int) % 3 as libc::c_int;
    do_content_change(sel);
}
/*
 * Request a re-scan on the currently selected crate
 */
#[no_mangle]
pub unsafe extern "C" fn selector_rescan(mut sel: *mut selector) {
    /* Ignore any errors at this point. A rescan must not leak
     * resources or cause a crash */
    library_rescan((*sel).library, current_crate(sel));
}
/*
 * Expand the search. Do not disrupt the running process on memory
 * allocation failure, leave the view index incomplete
 */
#[no_mangle]
pub unsafe extern "C" fn selector_search_expand(mut sel: *mut selector) {
    if (*sel).search_len == 0 as libc::c_int as libc::c_ulong { return }
    (*sel).search_len = (*sel).search_len.wrapping_sub(1);
    (*sel).search[(*sel).search_len as usize] =
        '\u{0}' as i32 as libc::c_char;
    match_compile(&mut (*sel).match_0, (*sel).search.as_mut_ptr());
    do_content_change(sel);
}
/*
 * Copyright (C) 2018 Mark Hills <mark@xwax.org>,
 *                    Yves Adler <yves.adler@googlemail.com>
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
/* base_index + search filter applied */
/* used to swap between a and b indexes */
/* the compiled search, kept in-sync */
/*
 * Refine the search. Do not distrupt the running process on memory
 * allocation failure, leave the view index incomplete
 */
#[no_mangle]
pub unsafe extern "C" fn selector_search_refine(mut sel: *mut selector,
                                                mut key: libc::c_char) {
    let mut tmp: *mut index = 0 as *mut index;
    if (*sel).search_len >=
           (::std::mem::size_of::<[libc::c_char; 256]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        /* would overflow */
        return
    }
    (*sel).search[(*sel).search_len as usize] = key;
    (*sel).search_len = (*sel).search_len.wrapping_add(1);
    (*sel).search[(*sel).search_len as usize] =
        '\u{0}' as i32 as libc::c_char;
    match_compile(&mut (*sel).match_0, (*sel).search.as_mut_ptr());
    index_match((*sel).view_index, (*sel).swap_index, &mut (*sel).match_0);
    tmp = (*sel).view_index;
    (*sel).view_index = (*sel).swap_index;
    (*sel).swap_index = tmp;
    listbox_set_entries(&mut (*sel).records,
                        (*(*sel).view_index).entries as libc::c_uint);
    set_target(sel);
    notify(sel);
}
