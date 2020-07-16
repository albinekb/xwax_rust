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
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn mlock(__addr: *const libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn fork_pipe_nb(fd: *mut libc::c_int, path: *const libc::c_char,
                    arg: *mut libc::c_char, _: ...) -> pid_t;
    #[no_mangle]
    fn rt_not_allowed();
    #[no_mangle]
    fn rig_post_track(t: *mut track);
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
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
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
#[inline]
unsafe extern "C" fn list_del(mut entry: *mut list) {
    let mut next: *mut list = 0 as *mut list;
    let mut prev: *mut list = 0 as *mut list;
    next = (*entry).next;
    prev = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
}
static mut tracks: list =
    unsafe {
        {
            let mut init =
                list{prev: &tracks as *const list as *mut list,
                     next: &tracks as *const list as *mut list,};
            init
        }
    };
static mut use_mlock: bool = 0 as libc::c_int != 0;
/*
 * An empty track is used rarely, and is easier than
 * continuous checks for NULL throughout the code
 */
static mut empty: track =
    {
        let mut init =
            track{tracks:
                      list{prev: 0 as *const list as *mut list,
                           next: 0 as *const list as *mut list,},
                  refcount: 1 as libc::c_int as libc::c_uint,
                  rate: 44100 as libc::c_int,
                  importer: 0 as *const libc::c_char,
                  path: 0 as *const libc::c_char,
                  bytes: 0 as libc::c_int as size_t,
                  length: 0 as libc::c_int as libc::c_uint,
                  blocks: 0 as libc::c_int as libc::c_uint,
                  block: [0 as *const track_block as *mut track_block; 64],
                  rig:
                      list{prev: 0 as *const list as *mut list,
                           next: 0 as *const list as *mut list,},
                  pid: 0 as libc::c_int,
                  fd: 0,
                  pe: 0 as *const pollfd as *mut pollfd,
                  terminated: false,
                  ppm: 0,
                  overview: 0,};
        init
    };
/*
 * Request that memory for tracks is locked into RAM as it is
 * allocated
 */
#[no_mangle]
pub unsafe extern "C" fn track_use_mlock() {
    use_mlock = 1 as libc::c_int != 0;
}
/*
 * Allocate more memory
 *
 * Return: -1 if memory could not be allocated, otherwize 0
 */
unsafe extern "C" fn more_space(mut tr: *mut track) -> libc::c_int {
    let mut block: *mut track_block = 0 as *mut track_block;
    rt_not_allowed();
    if (*tr).blocks >= 64 as libc::c_int as libc::c_uint {
        fprintf(stderr,
                b"Maximum track length reached.\n\x00" as *const u8 as
                    *const libc::c_char);
        return -(1 as libc::c_int)
    }
    block =
        malloc(::std::mem::size_of::<track_block>() as libc::c_ulong) as
            *mut track_block;
    if block.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if use_mlock as libc::c_int != 0 &&
           mlock(block as *const libc::c_void,
                 ::std::mem::size_of::<track_block>() as libc::c_ulong) ==
               -(1 as libc::c_int) {
        perror(b"mlock\x00" as *const u8 as *const libc::c_char);
        free(block as *mut libc::c_void);
        return -(1 as libc::c_int)
    }
    /* No memory barrier is needed here, because nobody else tries to
     * access these blocks until tr->length is actually incremented */
    let fresh0 = (*tr).blocks;
    (*tr).blocks = (*tr).blocks.wrapping_add(1);
    (*tr).block[fresh0 as usize] = block;
    return 0 as libc::c_int;
}
/*
 * Get access to the PCM buffer for incoming audio
 *
 * Return: pointer to buffer
 * Post: len contains the length of the buffer, in bytes
 */
unsafe extern "C" fn access_pcm(mut tr: *mut track, mut len: *mut size_t)
 -> *mut libc::c_void {
    let mut block: libc::c_uint = 0;
    let mut fill: size_t = 0;
    block =
        (*tr).bytes.wrapping_div(((2048 as libc::c_int * 1024 as libc::c_int)
                                      as
                                      libc::c_ulong).wrapping_mul((::std::mem::size_of::<libc::c_short>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(2
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
            as libc::c_uint;
    if block == (*tr).blocks {
        if more_space(tr) == -(1 as libc::c_int) {
            return 0 as *mut libc::c_void
        }
    }
    fill =
        (*tr).bytes.wrapping_rem(((2048 as libc::c_int * 1024 as libc::c_int)
                                      as
                                      libc::c_ulong).wrapping_mul((::std::mem::size_of::<libc::c_short>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(2
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)));
    *len =
        ((2048 as libc::c_int * 1024 as libc::c_int) as
             libc::c_ulong).wrapping_mul((::std::mem::size_of::<libc::c_short>()
                                              as
                                              libc::c_ulong).wrapping_mul(2 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)).wrapping_sub(fill);
    return ((*(*tr).block[block as usize]).pcm.as_mut_ptr() as
                *mut libc::c_void).offset(fill as isize);
}
/*
 * Notify that audio has been placed in the buffer
 *
 * The parameter is the number of stereo samples which have been
 * placed in the buffer.
 */
unsafe extern "C" fn commit_pcm_samples(mut tr: *mut track,
                                        mut samples: libc::c_uint) {
    let mut fill: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut pcm: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut block: *mut track_block = 0 as *mut track_block;
    block =
        (*tr).block[(*tr).length.wrapping_div((2048 as libc::c_int *
                                                   1024 as libc::c_int) as
                                                  libc::c_uint) as usize];
    fill =
        (*tr).length.wrapping_rem((2048 as libc::c_int * 1024 as libc::c_int)
                                      as libc::c_uint);
    pcm =
        (*block).pcm.as_mut_ptr().offset((2 as libc::c_int as
                                              libc::c_uint).wrapping_mul(fill)
                                             as isize);
    if samples <=
           ((2048 as libc::c_int * 1024 as libc::c_int) as
                libc::c_uint).wrapping_sub(fill) {
    } else {
        __assert_fail(b"samples <= TRACK_BLOCK_SAMPLES - fill\x00" as
                          *const u8 as *const libc::c_char,
                      b"track.c\x00" as *const u8 as *const libc::c_char,
                      154 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"void commit_pcm_samples(struct track *, unsigned int)\x00")).as_ptr());
    }
    /* Meter the new audio */
    n = samples;
    while n > 0 as libc::c_int as libc::c_uint {
        let mut v: libc::c_ushort = 0;
        let mut w: libc::c_uint = 0;
        v =
            (abs(*pcm.offset(0 as libc::c_int as isize) as libc::c_int) +
                 abs(*pcm.offset(1 as libc::c_int as isize) as libc::c_int))
                as libc::c_ushort;
        /* PPM-style fast meter approximation */
        if v as libc::c_int > (*tr).ppm as libc::c_int {
            (*tr).ppm =
                ((*tr).ppm as libc::c_int +
                     (v as libc::c_int - (*tr).ppm as libc::c_int >>
                          3 as libc::c_int)) as libc::c_ushort
        } else {
            (*tr).ppm =
                ((*tr).ppm as libc::c_int -
                     ((*tr).ppm as libc::c_int - v as libc::c_int >>
                          9 as libc::c_int)) as libc::c_ushort
        }
        (*block).ppm[fill.wrapping_div(64 as libc::c_int as libc::c_uint) as
                         usize] =
            ((*tr).ppm as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        /* Update the slow-metering overview. Fixed point arithmetic
         * going on here */
        w = ((v as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        if w > (*tr).overview {
            (*tr).overview =
                (*tr).overview.wrapping_add(w.wrapping_sub((*tr).overview) >>
                                                8 as libc::c_int)
        } else {
            (*tr).overview =
                (*tr).overview.wrapping_sub((*tr).overview.wrapping_sub(w) >>
                                                17 as libc::c_int)
        }
        (*block).overview[fill.wrapping_div(2048 as libc::c_int as
                                                libc::c_uint) as usize] =
            ((*tr).overview >> 24 as libc::c_int) as libc::c_uchar;
        fill = fill.wrapping_add(1);
        pcm = pcm.offset(2 as libc::c_int as isize);
        n = n.wrapping_sub(1)
    }
    /* Increment the track length. A memory barrier ensures the
     * realtime or UI thread does not access garbage audio */
    ::std::intrinsics::atomic_xadd(&mut (*tr).length, samples);
}
/*
 * Notify that data has been placed in the buffer
 *
 * This function passes any whole samples to commit_pcm_samples()
 * and leaves the residual in the buffer ready for next time.
 */
unsafe extern "C" fn commit(mut tr: *mut track, mut len: size_t) {
    (*tr).bytes =
        ((*tr).bytes as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    commit_pcm_samples(tr,
                       (*tr).bytes.wrapping_div((::std::mem::size_of::<libc::c_short>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)).wrapping_sub((*tr).length
                                                                                                                      as
                                                                                                                      libc::c_ulong)
                           as libc::c_uint);
}
/*
 * Initialise object which will hold PCM audio data, and start
 * importing the data
 *
 * Post: track is initialised
 * Post: track is importing
 */
unsafe extern "C" fn track_init(mut t: *mut track,
                                mut importer: *const libc::c_char,
                                mut path: *const libc::c_char)
 -> libc::c_int {
    let mut pid: pid_t = 0;
    fprintf(stderr,
            b"Importing \'%s\'...\n\x00" as *const u8 as *const libc::c_char,
            path);
    pid =
        fork_pipe_nb(&mut (*t).fd as *mut libc::c_int, importer,
                     b"import\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, path,
                     b"44100\x00" as *const u8 as *const libc::c_char,
                     0 as *mut libc::c_void);
    if pid == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    (*t).pid = pid;
    (*t).pe = 0 as *mut pollfd;
    (*t).terminated = 0 as libc::c_int != 0;
    (*t).refcount = 0 as libc::c_int as libc::c_uint;
    (*t).blocks = 0 as libc::c_int as libc::c_uint;
    (*t).rate = 44100 as libc::c_int;
    (*t).bytes = 0 as libc::c_int as size_t;
    (*t).length = 0 as libc::c_int as libc::c_uint;
    (*t).ppm = 0 as libc::c_int as libc::c_ushort;
    (*t).overview = 0 as libc::c_int as libc::c_uint;
    (*t).importer = importer;
    (*t).path = path;
    list_add(&mut (*t).tracks, &mut tracks);
    rig_post_track(t);
    return 0 as libc::c_int;
}
/*
 * Destroy this track from memory
 *
 * Terminates any import processes and frees any memory allocated by
 * this object.
 *
 * Pre: track is not importing
 * Pre: track is initialised
 */
unsafe extern "C" fn track_clear(mut tr: *mut track) {
    let mut n: libc::c_int = 0;
    if (*tr).pid == 0 as libc::c_int {
    } else {
        __assert_fail(b"tr->pid == 0\x00" as *const u8 as *const libc::c_char,
                      b"track.c\x00" as *const u8 as *const libc::c_char,
                      263 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void track_clear(struct track *)\x00")).as_ptr());
    }
    n = 0 as libc::c_int;
    while (n as libc::c_uint) < (*tr).blocks {
        free((*tr).block[n as usize] as *mut libc::c_void);
        n += 1
    }
    list_del(&mut (*tr).tracks);
}
/*
 * Get a pointer to a track object already in memory
 *
 * Return: pointer, or NULL if no such track exists
 */
unsafe extern "C" fn track_get_again(mut importer: *const libc::c_char,
                                     mut path: *const libc::c_char)
 -> *mut track {
    let mut t: *mut track = 0 as *mut track;
    t =
        (tracks.next as
             *mut libc::c_char).offset(-(0 as libc::c_ulong as isize)) as
            *mut track;
    while &mut (*t).tracks as *mut list != &mut tracks as *mut list {
        if (*t).importer == importer && (*t).path == path {
            track_acquire(t);
            return t
        }
        t =
            ((*t).tracks.next as
                 *mut libc::c_char).offset(-(0 as libc::c_ulong as isize)) as
                *mut track
    }
    return 0 as *mut track;
}
/*
 * Get a pointer to a track object for the given importer and path
 *
 * Return: pointer, or NULL if not enough resources
 */
#[no_mangle]
pub unsafe extern "C" fn track_acquire_by_import(mut importer:
                                                     *const libc::c_char,
                                                 mut path:
                                                     *const libc::c_char)
 -> *mut track {
    let mut t: *mut track = 0 as *mut track;
    t = track_get_again(importer, path);
    if !t.is_null() { return t }
    t = malloc(::std::mem::size_of::<track>() as libc::c_ulong) as *mut track;
    if t.is_null() {
        perror(b"malloc\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut track
    }
    if track_init(t, importer, path) == -(1 as libc::c_int) {
        free(t as *mut libc::c_void);
        return 0 as *mut track
    }
    track_acquire(t);
    return t;
}
/*
 * Get a pointer to a static track containing no audio
 *
 * Return: pointer, not NULL
 */
#[no_mangle]
pub unsafe extern "C" fn track_acquire_empty() -> *mut track {
    empty.refcount = empty.refcount.wrapping_add(1);
    return &mut empty;
}
#[no_mangle]
pub unsafe extern "C" fn track_acquire(mut t: *mut track) {
    (*t).refcount = (*t).refcount.wrapping_add(1);
}
/*
 * Request premature termination of an import operation
 */
unsafe extern "C" fn terminate(mut t: *mut track) {
    if (*t).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"t->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"track.c\x00" as *const u8 as *const libc::c_char,
                      344 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void terminate(struct track *)\x00")).as_ptr());
    }
    if kill((*t).pid, 15 as libc::c_int) == -(1 as libc::c_int) { abort(); }
    (*t).terminated = 1 as libc::c_int != 0;
}
/*
 * Finish use of a track object
 */
#[no_mangle]
pub unsafe extern "C" fn track_release(mut t: *mut track) {
    (*t).refcount = (*t).refcount.wrapping_sub(1);
    /* When importing, a reference is held. If it's the
     * only one remaining terminate it to save resources */
    if (*t).refcount == 1 as libc::c_int as libc::c_uint &&
           (*t).pid != 0 as libc::c_int {
        terminate(t);
        return
    }
    if (*t).refcount == 0 as libc::c_int as libc::c_uint {
        if t != &mut empty as *mut track {
        } else {
            __assert_fail(b"t != &empty\x00" as *const u8 as
                              *const libc::c_char,
                          b"track.c\x00" as *const u8 as *const libc::c_char,
                          369 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void track_release(struct track *)\x00")).as_ptr());
        }
        track_clear(t);
        free(t as *mut libc::c_void);
    };
}
/*
 * Get entry for use by poll()
 *
 * Pre: track is importing
 * Post: *pe contains poll entry
 */
#[no_mangle]
pub unsafe extern "C" fn track_pollfd(mut t: *mut track,
                                      mut pe: *mut pollfd) {
    if (*t).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"t->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"track.c\x00" as *const u8 as *const libc::c_char,
                      384 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void track_pollfd(struct track *, struct pollfd *)\x00")).as_ptr());
    }
    (*pe).fd = (*t).fd;
    (*pe).events = 0x1 as libc::c_int as libc::c_short;
    (*t).pe = pe;
}
/*
 * Read the next block of data from the file handle into the track's
 * PCM data
 *
 * Return: -1 on completion, otherwise zero
 */
unsafe extern "C" fn read_from_pipe(mut tr: *mut track) -> libc::c_int {
    loop  {
        let mut pcm: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut len: size_t = 0;
        let mut z: ssize_t = 0;
        pcm = access_pcm(tr, &mut len);
        if pcm.is_null() { return -(1 as libc::c_int) }
        z = read((*tr).fd, pcm, len);
        if z == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() == 11 as libc::c_int {
                return 0 as libc::c_int
            } else {
                perror(b"read\x00" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int)
            }
        }
        if z == 0 as libc::c_int as libc::c_long { break ; }
        commit(tr, z as size_t);
    }
    return -(1 as libc::c_int);
    /* completion without error */
}
/*
 * Synchronise with the import process and complete it
 *
 * Pre: track is importing
 * Post: track is not importing
 */
unsafe extern "C" fn stop_import(mut t: *mut track) {
    let mut status: libc::c_int = 0;
    if (*t).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"t->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"track.c\x00" as *const u8 as *const libc::c_char,
                      440 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void stop_import(struct track *)\x00")).as_ptr());
    }
    if close((*t).fd) == -(1 as libc::c_int) { abort(); }
    if waitpid((*t).pid, &mut status, 0 as libc::c_int) == -(1 as libc::c_int)
       {
        abort();
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int &&
           (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
               0 as libc::c_int {
        fprintf(stderr,
                b"Track import completed\n\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(stderr,
                b"Track import completed with status %d\n\x00" as *const u8 as
                    *const libc::c_char, status);
        if !(*t).terminated {
            status_printf(3 as libc::c_int,
                          b"Error importing %s\x00" as *const u8 as
                              *const libc::c_char, (*t).path);
        }
    }
    (*t).pid = 0 as libc::c_int;
}
/*
 * Handle any file descriptor activity on this track
 *
 * Return: true if import has completed, otherwise false
 */
#[no_mangle]
pub unsafe extern "C" fn track_handle(mut tr: *mut track) {
    if (*tr).pid != 0 as libc::c_int {
    } else {
        __assert_fail(b"tr->pid != 0\x00" as *const u8 as *const libc::c_char,
                      b"track.c\x00" as *const u8 as *const libc::c_char,
                      467 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void track_handle(struct track *)\x00")).as_ptr());
    }
    /* A track may be added while poll() was waiting,
     * in which case it has no return data from poll */
    if (*tr).pe.is_null() { return }
    if (*(*tr).pe).revents as libc::c_int == 0 as libc::c_int { return }
    if read_from_pipe(tr) != -(1 as libc::c_int) { return }
    stop_import(tr);
    list_del(&mut (*tr).rig);
    track_release(tr);
    /* may delete the track */
}
