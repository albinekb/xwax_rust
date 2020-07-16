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
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasestr(__haystack: *const libc::c_char,
                  __needle: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
/* A 'compiled' search criteria, so we can repeat searches and
 * matches efficiently */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_0 {
    pub buf: [libc::c_char; 512],
    pub words: [*mut libc::c_char; 32],
}
/* NULL-terminated array */
/*
 * Initialise a record index
 */
#[no_mangle]
pub unsafe extern "C" fn index_init(mut ls: *mut index) {
    (*ls).record = 0 as *mut *mut record;
    (*ls).size = 0 as libc::c_int as size_t;
    (*ls).entries = 0 as libc::c_int as size_t;
}
/*
 * Deallocate resources associated with this index
 *
 * The index does not allocate records itself, so it is not
 * responsible for deallocating them.
 */
#[no_mangle]
pub unsafe extern "C" fn index_clear(mut ls: *mut index) {
    free((*ls).record as *mut libc::c_void);
    /* may be NULL */
}
/*
 * Blank the index so it contains no entries
 *
 * We don't de-allocate memory, but this gives us an advantage where
 * index re-use is of similar size.
 */
#[no_mangle]
pub unsafe extern "C" fn index_blank(mut ls: *mut index) {
    (*ls).entries = 0 as libc::c_int as size_t;
}
/*
 * Enlarge the storage space of the index to at least the target
 * size
 *
 * Return: 0 on success or -1 on memory allocation failure
 * Post: size of index is greater than or equal to target
 */
unsafe extern "C" fn enlarge(mut ls: *mut index, mut target: size_t)
 -> libc::c_int {
    let mut p: size_t = 0; /* pre-allocate additional entries */
    let mut ln: *mut *mut record = 0 as *mut *mut record;
    if target <= (*ls).size { return 0 as libc::c_int }
    p =
        target.wrapping_add(1024 as libc::c_int as
                                libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_ulong);
    ln =
        realloc((*ls).record as *mut libc::c_void,
                (::std::mem::size_of::<*mut record>() as
                     libc::c_ulong).wrapping_mul(p)) as *mut *mut record;
    if ln.is_null() {
        perror(b"realloc\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    (*ls).record = ln;
    (*ls).size = p;
    return 0 as libc::c_int;
}
/*
 * Return: false if the caller did not call index_reserve(), otherwise
 * true
 */
unsafe extern "C" fn has_space(mut i: *const index) -> bool {
    return (*i).entries < (*i).size;
}
/*
 * Add a record to the index
 *
 * Pre: at least one entry is reserved
 * Post: lr is the record at the end of the index
 */
#[no_mangle]
pub unsafe extern "C" fn index_add(mut ls: *mut index, mut lr: *mut record) {
    if !lr.is_null() {
    } else {
        __assert_fail(b"lr != NULL\x00" as *const u8 as *const libc::c_char,
                      b"index.c\x00" as *const u8 as *const libc::c_char,
                      119 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void index_add(struct index *, struct record *)\x00")).as_ptr());
    }
    if has_space(ls) {
    } else {
        __assert_fail(b"has_space(ls)\x00" as *const u8 as
                          *const libc::c_char,
                      b"index.c\x00" as *const u8 as *const libc::c_char,
                      120 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void index_add(struct index *, struct record *)\x00")).as_ptr());
    }
    let fresh0 = (*ls).entries;
    (*ls).entries = (*ls).entries.wrapping_add(1);
    let ref mut fresh1 = *(*ls).record.offset(fresh0 as isize);
    *fresh1 = lr;
}
/*
 * Standard comparison function between two records
 */
unsafe extern "C" fn record_cmp_artist(mut a: *const record,
                                       mut b: *const record) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = strcasecmp((*a).artist, (*b).artist);
    if r < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else { if r > 0 as libc::c_int { return 1 as libc::c_int } }
    r = strcasecmp((*a).title, (*b).title);
    if r < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else { if r > 0 as libc::c_int { return 1 as libc::c_int } }
    return strcmp((*a).pathname, (*b).pathname);
}
/*
 * Compare two records principally by BPM, fastest to slowest
 * followed by unknown
 */
unsafe extern "C" fn record_cmp_bpm(mut a: *const record,
                                    mut b: *const record) -> libc::c_int {
    if (*a).bpm < (*b).bpm { return 1 as libc::c_int }
    if (*a).bpm > (*b).bpm { return -(1 as libc::c_int) }
    return record_cmp_artist(a, b);
}
/*
 * Check if a record matches the given string. This function is the
 * definitive code which defines what constitutes a 'match'.
 *
 * Return: true if this is a match, otherwise false
 */
unsafe extern "C" fn record_match_word(mut re: *mut record,
                                       mut match_0: *const libc::c_char)
 -> bool {
    /* Some records provide a dedicated string for matching against,
     * in the same locale as "match" */
    if !(*re).match_0.is_null() {
        if !strcasestr((*re).match_0, match_0).is_null() {
            return 1 as libc::c_int != 0
        }
    } else {
        if !strcasestr((*re).artist, match_0).is_null() {
            return 1 as libc::c_int != 0
        }
        if !strcasestr((*re).title, match_0).is_null() {
            return 1 as libc::c_int != 0
        }
    }
    return 0 as libc::c_int != 0;
}
/*
 * Check for a match against the given search criteria
 *
 * Return: true if the given record matches, otherwise false
 */
#[no_mangle]
pub unsafe extern "C" fn record_match(mut re: *mut record,
                                      mut h: *const match_0) -> bool {
    let mut matches: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    matches = (*h).words.as_ptr();
    while !(*matches).is_null() {
        if !record_match_word(re, *matches) { return 0 as libc::c_int != 0 }
        matches = matches.offset(1)
    }
    return 1 as libc::c_int != 0;
}
/*
 * Copy the source index
 *
 * Return: 0 on success or -1 on memory allocation failure
 * Post: on failure, dest is valid but incomplete
 */
#[no_mangle]
pub unsafe extern "C" fn index_copy(mut src: *const index,
                                    mut dest: *mut index) -> libc::c_int {
    let mut n: libc::c_int = 0;
    index_blank(dest);
    if index_reserve(dest, (*src).entries as libc::c_uint) ==
           -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    n = 0 as libc::c_int;
    while (n as libc::c_ulong) < (*src).entries {
        index_add(dest, *(*src).record.offset(n as isize));
        n += 1
    }
    return 0 as libc::c_int;
}
/*
 * Compile a search object from a given string
 *
 * Pre: search string is within length
 */
#[no_mangle]
pub unsafe extern "C" fn match_compile(mut h: *mut match_0,
                                       mut d: *const libc::c_char) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    if strlen(d) <
           ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong {
    } else {
        __assert_fail(b"strlen(d) < sizeof h->buf\x00" as *const u8 as
                          *const libc::c_char,
                      b"index.c\x00" as *const u8 as *const libc::c_char,
                      242 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void match_compile(struct match *, const char *)\x00")).as_ptr());
    }
    strcpy((*h).buf.as_mut_ptr(), d);
    buf = (*h).buf.as_mut_ptr();
    n = 0 as libc::c_int as size_t;
    loop  {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        if n ==
               (::std::mem::size_of::<[*mut libc::c_char; 32]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
           {
            fputs(b"Ignoring excessive words in match string.\n\x00" as
                      *const u8 as *const libc::c_char, stderr);
            break ;
        } else {
            (*h).words[n as usize] = buf;
            n = n.wrapping_add(1);
            s = strchr(buf, ' ' as i32);
            if s.is_null() { break ; }
            *s = '\u{0}' as i32 as libc::c_char;
            buf = s.offset(1 as libc::c_int as isize)
        }
        /* skip separator */
    }
    (*h).words[n as usize] = 0 as *mut libc::c_char;
    /* terminate list */
}
/*
 * Find entries from the source index which match
 *
 * Copy the subset of the source index which matches the given
 * string into the destination.
 *
 * Return: 0 on success, or -1 on memory allocation failure
 * Post: on failure, dest is valid but incomplete
 */
#[no_mangle]
pub unsafe extern "C" fn index_match(mut src: *mut index,
                                     mut dest: *mut index,
                                     mut match_0: *const match_0)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut re: *mut record = 0 as *mut record;
    index_blank(dest);
    n = 0 as libc::c_int;
    while (n as libc::c_ulong) < (*src).entries {
        re = *(*src).record.offset(n as isize);
        if record_match(re, match_0) {
            if index_reserve(dest, 1 as libc::c_int as libc::c_uint) ==
                   -(1 as libc::c_int) {
                return -(1 as libc::c_int)
            }
            index_add(dest, re);
        }
        n += 1
    }
    return 0 as libc::c_int;
}
/*
 * Binary search of sorted index
 *
 * We implement our own binary search rather than using the bsearch()
 * from stdlib.h, because we need to know the position to insert to if
 * the item is not found.
 *
 * Pre: base is sorted
 * Return: position of match >= item
 * Post: on exact match, *found is true
 */
unsafe extern "C" fn bin_search(mut base: *mut *mut record, mut n: size_t,
                                mut item: *mut record, mut sort: libc::c_int,
                                mut found: *mut bool) -> size_t {
    let mut r: libc::c_int = 0;
    let mut mid: size_t = 0;
    let mut x: *mut record = 0 as *mut record;
    /* Return the first entry ordered after this one */
    if n == 0 as libc::c_int as libc::c_ulong {
        *found = 0 as libc::c_int != 0;
        return 0 as libc::c_int as size_t
    }
    mid = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    x = *base.offset(mid as isize);
    match sort {
        0 => { r = record_cmp_artist(item, x) }
        1 => { r = record_cmp_bpm(item, x) }
        2 | _ => { abort(); }
    }
    if r < 0 as libc::c_int {
        return bin_search(base, mid, item, sort, found)
    }
    if r > 0 as libc::c_int {
        return mid.wrapping_add(1 as libc::c_int as
                                    libc::c_ulong).wrapping_add(bin_search(base.offset(mid
                                                                                           as
                                                                                           isize).offset(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             isize),
                                                                           n.wrapping_sub(mid).wrapping_sub(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong),
                                                                           item,
                                                                           sort,
                                                                           found))
    }
    *found = 1 as libc::c_int != 0;
    return mid;
}
/*
 * Insert or re-use an entry in a sorted index
 *
 * Pre: index is sorted
 * Pre: at least one entry is reserved
 * Return: pointer to item, or existing entry (ie. not NULL)
 * Post: index is sorted and contains item or a matching item
 */
#[no_mangle]
pub unsafe extern "C" fn index_insert(mut ls: *mut index,
                                      mut item: *mut record,
                                      mut sort: libc::c_int) -> *mut record {
    let mut found: bool = false;
    let mut z: size_t = 0;
    z = bin_search((*ls).record, (*ls).entries, item, sort, &mut found);
    if found { return *(*ls).record.offset(z as isize) }
    if has_space(ls) {
    } else {
        __assert_fail(b"has_space(ls)\x00" as *const u8 as
                          *const libc::c_char,
                      b"index.c\x00" as *const u8 as *const libc::c_char,
                      370 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"struct record *index_insert(struct index *, struct record *, int)\x00")).as_ptr());
    }
    memmove((*ls).record.offset(z as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            (*ls).record.offset(z as isize) as *const libc::c_void,
            (::std::mem::size_of::<*mut record>() as
                 libc::c_ulong).wrapping_mul((*ls).entries.wrapping_sub(z)));
    let ref mut fresh2 = *(*ls).record.offset(z as isize);
    *fresh2 = item;
    (*ls).entries = (*ls).entries.wrapping_add(1);
    return item;
}
/*
 * Reserve space in the index for the addition of n new items
 *
 * This function exists separately to the insert and addition
 * functions because it carries the error case.
 *
 * Return: -1 if not enough memory, otherwise zero
 * Post: if zero is returned, index has at least n free slots
 */
#[no_mangle]
pub unsafe extern "C" fn index_reserve(mut i: *mut index, mut n: libc::c_uint)
 -> libc::c_int {
    return enlarge(i, (*i).entries.wrapping_add(n as libc::c_ulong));
}
/*
 * Find an identical entry, or the nearest match
 */
#[no_mangle]
pub unsafe extern "C" fn index_find(mut ls: *mut index, mut item: *mut record,
                                    mut sort: libc::c_int) -> size_t {
    let mut found: bool = false;
    let mut z: size_t = 0;
    z = bin_search((*ls).record, (*ls).entries, item, sort, &mut found);
    return z;
}
/*
 * Debug the content of a index to standard error
 */
#[no_mangle]
pub unsafe extern "C" fn index_debug(mut ls: *mut index) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while (n as libc::c_ulong) < (*ls).entries {
        fprintf(stderr, b"%d: %s\n\x00" as *const u8 as *const libc::c_char,
                n, (**(*ls).record.offset(n as isize)).pathname);
        n += 1
    };
}
