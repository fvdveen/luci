use core::fmt;
use std::sync::{Mutex, OnceLock};

use bumpalo::Bump;
use indexmap::IndexSet;

static GLOBAL: OnceLock<Interner> = OnceLock::new();

pub struct Interner(Mutex<InternerInner>);

struct InternerInner {
    arena: Bump,
    strings: IndexSet<&'static str>,
}

impl Interner {
    fn prefill(init: &[&'static str]) -> Self {
        Interner(Mutex::new(InternerInner {
            arena: Default::default(),
            strings: init.iter().copied().collect(),
        }))
    }

    #[inline]
    fn intern(&self, string: &str) -> Symbol {
        let mut inner = self.0.lock().unwrap();
        if let Some(idx) = inner.strings.get_index_of(string) {
            return Symbol::new(idx as u32);
        }

        let string: &str = inner.arena.alloc_str(string);

        // SAFETY: as the global arena lives to the end of the program,
        // we can safely extend this lifetime to `'static'`
        let string: &'static str = unsafe { &*(string as *const str) };

        let (idx, is_new) = inner.strings.insert_full(string);
        debug_assert!(is_new); // due to the get_index_of check above

        Symbol::new(idx as u32)
    }

    fn get(&self, symbol: Symbol) -> &str {
        self.0
            .lock()
            .unwrap()
            .strings
            .get_index(symbol.index as usize)
            .unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    index: u32,
}

impl Symbol {
    const fn new(idx: u32) -> Self {
        Symbol { index: idx }
    }

    pub fn intern(str: impl AsRef<str>) -> Self {
        let interner = GLOBAL.get_or_init(|| Interner::fresh());

        interner.intern(str.as_ref())
    }

    pub fn as_str(&self) -> &str {
        let interner = GLOBAL.get_or_init(|| Interner::fresh());

        unsafe { std::mem::transmute::<&str, &str>(interner.get(*self)) }
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Symbol").field(&self.as_str()).finish()
    }
}

macros::symbols! {
    Keywords {
        Empty: "",
        Underscore: "_",

        Else: "else",
        Fn: "fn",
        If: "if",
        Return: "return",
    }

    Symbols {
        main,
        println,
    }
}

pub mod kw {
    #[allow(unused_imports)]
    pub use super::kw_generated::*;
}

pub mod sym {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::sym_generated::*;
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, Rng};

    use super::Symbol;

    #[test]
    fn simple_interning() {
        let str = "Hello, test!";

        let sym = Symbol::intern(str);

        assert_eq!(str, sym.as_str())
    }

    #[test]
    fn multiple_interned() {
        let strings = (0..1000)
            .into_iter()
            .map(|idx| {
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(idx)
                    .map(char::from)
                    .collect::<String>()
            })
            .map(|str| {
                let sym = Symbol::intern(&str);
                Interned { str: str, sym: sym }
            });

        for Interned { str, sym } in strings {
            assert_eq!(str.as_str(), sym.as_str())
        }

        struct Interned {
            str: String,
            sym: Symbol,
        }
    }
}
