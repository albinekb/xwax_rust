use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn rt_not_allowed();
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
    /* pointers to external data */
    /* loaded in */
    /* track length in samples */
    /* number of blocks allocated */
    /* State of audio import */
    /* Current value of audio meters when loading */
    /* Tracks are dynamically allocated and reference counted */
    /* Functions used by the rig and main thread */
    #[no_mangle]
    fn track_handle(tr: *mut track);
    #[no_mangle]
    fn excrate_acquire(e: *mut excrate);
    /* Used by the rig and main thread */
    #[no_mangle]
    fn excrate_pollfd(tr: *mut excrate, pe: *mut pollfd);
    #[no_mangle]
    fn excrate_handle(tr: *mut excrate);
    #[no_mangle]
    fn track_acquire(t: *mut track);
    #[no_mangle]
    fn track_pollfd(tr: *mut track, pe: *mut pollfd);
}
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_int,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_long,
}
pub type nfds_t = libc::c_ulong;
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
 * Double-linked lists
 */
/* offsetof() */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub prev: *mut list,
    pub next: *mut list,
}
pub type mutex = pthread_mutex_t;
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
pub struct track_block {
    pub pcm: [libc::c_short; 4194304],
    pub ppm: [libc::c_uchar; 32768],
    pub overview: [libc::c_uchar; 1024],
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
 * Mutex locking for syncronisation between low priority threads
 */
/*
 * Pre: lock is not held
 */
/*
 * Take a mutex lock
 *
 * Pre: lock is initialised
 * Pre: lock is not held by this thread
 * Post: lock is held by this thread
 */
/*
 * Release a mutex lock
 *
 * Pre: lock is held by this thread
 * Post: lock is not held
 */
#[inline]
unsafe extern "C" fn mutex_unlock(mut m: *mut mutex) {
    if pthread_mutex_unlock(m) != 0 as libc::c_int { abort(); };
}
#[inline]
unsafe extern "C" fn mutex_lock(mut m: *mut mutex) {
    rt_not_allowed();
    if pthread_mutex_lock(m) != 0 as libc::c_int { abort(); };
}
#[inline]
unsafe extern "C" fn mutex_clear(mut m: *mut mutex) {
    let mut r: libc::c_int = 0;
    r = pthread_mutex_destroy(m);
    if r != 0 as libc::c_int {
        *__errno_location() = r;
        perror(b"pthread_mutex_destroy\x00" as *const u8 as
                   *const libc::c_char);
        abort();
    };
}
#[inline]
unsafe extern "C" fn mutex_init(mut m: *mut mutex) {
    if pthread_mutex_init(m, 0 as *const pthread_mutexattr_t) !=
           0 as libc::c_int {
        abort();
    };
}
static mut event: [libc::c_int; 2] = [0; 2];
/* pipe to wake up service thread */
static mut tracks: list =
    unsafe {
        {
            let mut init =
                list{prev: &tracks as *const list as *mut list,
                     next: &tracks as *const list as *mut list,};
            init
        }
    };
static mut excrates: list =
    unsafe {
        {
            let mut init =
                list{prev: &excrates as *const list as *mut list,
                     next: &excrates as *const list as *mut list,};
            init
        }
    };
#[no_mangle]
pub static mut lock: mutex =
    pthread_mutex_t{__data:
                        __pthread_mutex_s{__lock: 0,
                                          __count: 0,
                                          __owner: 0,
                                          __nusers: 0,
                                          __kind: 0,
                                          __spins: 0,
                                          __list:
                                              __pthread_list_t{__prev:
                                                                   0 as
                                                                       *const __pthread_internal_list
                                                                       as
                                                                       *mut __pthread_internal_list,
                                                               __next:
                                                                   0 as
                                                                       *const __pthread_internal_list
                                                                       as
                                                                       *mut __pthread_internal_list,},},};
#[no_mangle]
pub unsafe extern "C" fn rig_init() -> libc::c_int {
    /* Create a pipe which will be used to wake us from other threads */
    if pipe(event.as_mut_ptr()) == -(1 as libc::c_int) {
        perror(b"pipe\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if fcntl(event[0 as libc::c_int as usize], 4 as libc::c_int,
             0o4000 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"fcntl\x00" as *const u8 as *const libc::c_char);
        if close(event[1 as libc::c_int as usize]) == -(1 as libc::c_int) {
            abort();
        }
        if close(event[0 as libc::c_int as usize]) == -(1 as libc::c_int) {
            abort();
        }
        return -(1 as libc::c_int)
    }
    mutex_init(&mut lock);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rig_clear() {
    mutex_clear(&mut lock);
    if close(event[0 as libc::c_int as usize]) == -(1 as libc::c_int) {
        abort();
    }
    if close(event[1 as libc::c_int as usize]) == -(1 as libc::c_int) {
        abort();
    };
}
/*
 * Main thread which handles input and output
 *
 * The rig is the main thread of execution. It is responsible for all
 * non-priority event driven operations (eg. everything but audio).
 *
 * The SDL interface requires its own event loop, and so whilst the
 * rig is technically responsible for managing it, it does very little
 * on its behalf. In future if there are other interfaces or
 * controllers (which expected to use more traditional file-descriptor
 * I/O), the rig will also be responsible for them.
 */
#[no_mangle]
pub unsafe extern "C" fn rig_main() -> libc::c_int {
    let mut pt: [pollfd; 4] = [pollfd{fd: 0, events: 0, revents: 0,}; 4];
    let mut px: *const pollfd =
        pt.as_mut_ptr().offset((::std::mem::size_of::<[pollfd; 4]>() as
                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<pollfd>()
                                                                    as
                                                                    libc::c_ulong)
                                   as isize);
    /* Monitor event pipe from external threads */
    pt[0 as libc::c_int as usize].fd = event[0 as libc::c_int as usize];
    pt[0 as libc::c_int as usize].revents = 0 as libc::c_int as libc::c_short;
    pt[0 as libc::c_int as usize].events =
        0x1 as libc::c_int as libc::c_short;
    mutex_lock(&mut lock);
    's_29:
        loop 
             /* exit via EVENT_QUIT */
             {
            let mut r: libc::c_int = 0;
            let mut pe: *mut pollfd = 0 as *mut pollfd;
            let mut track: *mut track = 0 as *mut track;
            let mut xtrack: *mut track = 0 as *mut track;
            let mut excrate: *mut excrate = 0 as *mut excrate;
            let mut xexcrate: *mut excrate = 0 as *mut excrate;
            pe =
                &mut *pt.as_mut_ptr().offset(1 as libc::c_int as isize) as
                    *mut pollfd;
            /* Do our best if we run out of poll entries */
            track =
                (tracks.next as
                     *mut libc::c_char).offset(-(568 as libc::c_ulong as
                                                     isize)) as *mut track;
            while &mut (*track).rig as *mut list != &mut tracks as *mut list {
                if pe == px as *mut pollfd { break ; }
                track_pollfd(track, pe);
                pe = pe.offset(1);
                track =
                    ((*track).rig.next as
                         *mut libc::c_char).offset(-(568 as libc::c_ulong as
                                                         isize)) as *mut track
            }
            excrate =
                (excrates.next as
                     *mut libc::c_char).offset(-(144 as libc::c_ulong as
                                                     isize)) as *mut excrate;
            while &mut (*excrate).rig as *mut list !=
                      &mut excrates as *mut list {
                if pe == px as *mut pollfd { break ; }
                excrate_pollfd(excrate, pe);
                pe = pe.offset(1);
                excrate =
                    ((*excrate).rig.next as
                         *mut libc::c_char).offset(-(144 as libc::c_ulong as
                                                         isize)) as
                        *mut excrate
            }
            mutex_unlock(&mut lock);
            r =
                poll(pt.as_mut_ptr(),
                     pe.wrapping_offset_from(pt.as_mut_ptr()) as libc::c_long
                         as nfds_t, -(1 as libc::c_int));
            if r == -(1 as libc::c_int) {
                if *__errno_location() == 4 as libc::c_int {
                    mutex_lock(&mut lock);
                } else {
                    perror(b"poll\x00" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int)
                }
            } else {
                /* Process all events on the event pipe */
                if pt[0 as libc::c_int as usize].revents as libc::c_int !=
                       0 as libc::c_int {
                    loop  {
                        let mut e: libc::c_char = 0;
                        let mut z: size_t = 0;
                        z =
                            read(event[0 as libc::c_int as usize],
                                 &mut e as *mut libc::c_char as
                                     *mut libc::c_void,
                                 1 as libc::c_int as size_t) as size_t;
                        if z == -(1 as libc::c_int) as libc::c_ulong {
                            if *__errno_location() == 11 as libc::c_int {
                                break ;
                            }
                            perror(b"read\x00" as *const u8 as
                                       *const libc::c_char);
                            return -(1 as libc::c_int)
                        } else {
                            match e as libc::c_int {
                                0 => { continue ; }
                                1 => { break 's_29 ; }
                                _ => { }
                            }
                            abort();
                        }
                    }
                }
                mutex_lock(&mut lock);
                track =
                    (tracks.next as
                         *mut libc::c_char).offset(-(568 as libc::c_ulong as
                                                         isize)) as
                        *mut track;
                xtrack =
                    ((*track).rig.next as
                         *mut libc::c_char).offset(-(568 as libc::c_ulong as
                                                         isize)) as
                        *mut track;
                while &mut (*track).rig as *mut list !=
                          &mut tracks as *mut list {
                    track_handle(track);
                    track = xtrack;
                    xtrack =
                        ((*xtrack).rig.next as
                             *mut libc::c_char).offset(-(568 as libc::c_ulong
                                                             as isize)) as
                            *mut track
                }
                excrate =
                    (excrates.next as
                         *mut libc::c_char).offset(-(144 as libc::c_ulong as
                                                         isize)) as
                        *mut excrate;
                xexcrate =
                    ((*excrate).rig.next as
                         *mut libc::c_char).offset(-(144 as libc::c_ulong as
                                                         isize)) as
                        *mut excrate;
                while &mut (*excrate).rig as *mut list !=
                          &mut excrates as *mut list {
                    excrate_handle(excrate);
                    excrate = xexcrate;
                    xexcrate =
                        ((*xexcrate).rig.next as
                             *mut libc::c_char).offset(-(144 as libc::c_ulong
                                                             as isize)) as
                            *mut excrate
                }
            }
        }
    return 0 as libc::c_int;
}
/*
 * Post a simple event into the rig event loop
 */
unsafe extern "C" fn post_event(mut e: libc::c_char) -> libc::c_int {
    rt_not_allowed();
    if write(event[1 as libc::c_int as usize],
             &mut e as *mut libc::c_char as *const libc::c_void,
             1 as libc::c_int as size_t) ==
           -(1 as libc::c_int) as libc::c_long {
        perror(b"write\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Ask the rig to exit from another thread or signal handler
 */
#[no_mangle]
pub unsafe extern "C" fn rig_quit() -> libc::c_int {
    return post_event(1 as libc::c_int as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn rig_lock() { mutex_lock(&mut lock); }
#[no_mangle]
pub unsafe extern "C" fn rig_unlock() { mutex_unlock(&mut lock); }
/*
 * Add a track to be handled until import has completed
 */
#[no_mangle]
pub unsafe extern "C" fn rig_post_track(mut t: *mut track) {
    track_acquire(t);
    list_add(&mut (*t).rig, &mut tracks);
    post_event(0 as libc::c_int as libc::c_char);
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
pub unsafe extern "C" fn rig_post_excrate(mut e: *mut excrate) {
    excrate_acquire(e);
    list_add(&mut (*e).rig, &mut excrates);
    post_event(0 as libc::c_int as libc::c_char);
}
