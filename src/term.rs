use std::{
    collections::HashMap,
    ffi::OsStr,
    sync::{Arc, Mutex},
};

///! Note that the `.gitignore` files in the static `assets` may not
///! necessarily provide a one-to-one relationship between programming
///! languages and respective `.gitignore` files.
///!
///! For example, while `Rust.gitignore` provides a simple bijective
///! relationship between the `Rust` language and the corresponding
///! `.gitignore` file, the same does not hold for `JavaScript`, which
///! not only does not have a corresponding `.gitignore` file named
///! after itself (i.e., there is no `JavaScript.gitignore`); this is
///! because the language's ecosystem is not restricted to a single
///! project manager! In fact, for `JavaScript` we have the options of
///! `Node.gitignore` for standard `Node` projects, as well as
///! `Yeoman.gitignore` for VSCode extensions.
///!
///! Thus, our solution is to build a small database of *terms* and
///! their associated `.gitignore` files. These `Term`s will
///! effectively be pointers to interned strings so that `String`s
///! need not be allocated more than necessary. This also allows for
///! faster comparison, as each `Term` contains a `u32`, which is
///! cheaper to compare than `String`s alone.
///!

/// An interned `String` corresponding to text for which some
/// information is stored regarding a corresponding `.gitignore` file.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Term(u32);

impl Term {
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.lookup()
    }

    pub fn intern<S: AsRef<str>>(s: S) -> Self {
        match TERMS.lock() {
            Ok(mut guard) => guard.intern(s.as_ref().trim()),
            Err(e) => panic!("{e}"),
        }
    }

    pub fn lookup(&self) -> &str {
        let guard = match TERMS.lock() {
            Ok(guard) => guard,
            Err(e) => panic!("{e}"),
        };

        // Safety: we are extending the lifetime of the
        // string, however since it is interned with a
        // `'static` lifetime, the data pointed to should
        // always be valid. CONFIRM!
        unsafe { std::mem::transmute::<_, &str>(guard.lookup(self)) }
    }

    pub fn intern_iter<S: AsRef<str>>(
        iter: impl IntoIterator<Item = S>,
    ) -> impl Iterator<Item = Self> {
        iter.into_iter().map(Self::intern)
    }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Term({})", &self.0)
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lookup())
    }
}

impl AsRef<str> for Term {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for Term {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<String> for Term {
    fn from(s: String) -> Self {
        Term::intern(s)
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Self {
        Term::intern(s)
    }
}

impl PartialEq<String> for Term {
    fn eq(&self, other: &String) -> bool {
        self.as_str().eq_ignore_ascii_case(other.trim())
    }
}

impl PartialEq<Term> for String {
    fn eq(&self, other: &Term) -> bool {
        other.eq(self)
    }
}

impl PartialEq<&str> for Term {
    fn eq(&self, other: &&str) -> bool {
        self.as_str().eq_ignore_ascii_case(other.trim())
    }
}

impl PartialEq<Term> for &str {
    fn eq(&self, other: &Term) -> bool {
        other.eq(self)
    }
}

impl PartialEq<&OsStr> for Term {
    fn eq(&self, other: &&OsStr) -> bool {
        other.eq_ignore_ascii_case(self.as_str())
    }
}

impl PartialEq<Term> for &OsStr {
    fn eq(&self, other: &Term) -> bool {
        other.eq(self)
    }
}

/// String interner to hold stored strings with their corresponding
/// `Keyword` representations. A single instance of this type is used
/// statically (and globally).
// #[derive(Debug)]
struct Lexicon {
    map: HashMap<&'static str, Term>,
    vec: Vec<&'static str>,
    buf: String,
    all: Vec<String>,
}

impl Default for Lexicon {
    fn default() -> Self {
        Self {
            map: HashMap::with_capacity(Self::BASE_CAPACITY),
            vec: Vec::with_capacity(Self::BASE_CAPACITY),
            buf: Default::default(),
            all: Vec::with_capacity(Self::BASE_CAPACITY),
        }
    }
}

impl Lexicon {
    const BASE_CAPACITY: usize = 100;

    fn intern(&mut self, string: &str) -> Term {
        if let Some(&id) = self.map.get(string) {
            return id;
        }

        let string = unsafe { self.alloc(string) };
        let id = Term(self.map.len() as u32);

        self.map.insert(string, id);
        self.vec.push(string);

        id
    }

    fn lookup(&self, kw: &Term) -> &str {
        self.vec[kw.as_usize()]
    }

    unsafe fn alloc(&mut self, string: &str) -> &'static str {
        let cap = self.buf.capacity();
        if cap < self.buf.len() + string.len() {
            // just doubling isn't enough -- need to ensure the new string
            // actually fits
            let new_cap = (cap.max(string.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = std::mem::replace(&mut self.buf, new_buf);
            self.all.push(old_buf);
        }

        let interned = {
            let start = self.buf.len();
            self.buf.push_str(string);
            &self.buf[start..]
        };

        &*(interned as *const str)
    }
}

// Since this is for a command line utility, it might not be
// necessary for stored keywords to be thread-safe?
lazy_static::lazy_static! {
    static ref TERMS: Arc<Mutex<Lexicon>> = Arc::new(Mutex::new(Lexicon::default()));
}
