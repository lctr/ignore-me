#![allow(unused)]

pub struct Data<I, K>(pub I, pub K);

impl<I: Clone, K: Clone> Clone for Data<I, K> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<I: Copy, K: Copy> Copy for Data<I, K> {}

impl<I: PartialEq, K: PartialEq> PartialEq for Data<I, K> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<I: Eq, K: Eq> Eq for Data<I, K> {}

pub type Str = &'static str;
pub type Strs = &'static [&'static str];

pub const ADA_EXTS: Data<Strs, Strs> = Data(&["Ada"], &["adb", "ads"]);

pub const C_EXTS: Data<Strs, Strs> = Data(&["C"], &["c", "h"]);
pub const CPP_EXTS: Data<Strs, Strs> = Data(
    &["C++"],
    &[
        "c", "cc", "cpp", "cxx", "c++", "h", "hh", "hpp", "hxx", "h++",
    ],
);
pub const CLOJURE_EXTS: Data<Strs, Strs> = Data(&["Clojure"], &["clj", "cljs", "cljc", "edn"]);
pub const COMMON_LISP_EXTS: Data<Strs, Strs> =
    Data(&["CommonLisp"], &["lisp", "cl", "l", "lsp", "fasl"]);
pub const D_EXTS: Data<Strs, Strs> = Data(&["D"], &["d"]);
pub const DART_EXTS: Data<Strs, Strs> = Data(&["Dart"], &["dart"]);
pub const ELISP_EXTS: Data<Strs, Strs> = Data(&["Elisp"], &["el", "elc", "eln"]);
pub const ELIXIR_EXTS: Data<Strs, Strs> = Data(&["Elixir", "Phoenix"], &["ex", "exs"]);
pub const ERLANG_EXTS: Data<Strs, Strs> = Data(&["Erlang"], &["erl", "hrl"]);
pub const ELM_EXTS: Data<Strs, Strs> = Data(&["Elm"], &["elm"]);
pub const GO_EXTS: Data<Strs, Strs> = Data(&["Go"], &["go"]);
pub const JAVA_EXTS: Data<Strs, Strs> =
    Data(&["Java", "Maven"], &["java", "class", "jar", "classpath"]);
pub const JULIA_EXTS: Data<Strs, Strs> = Data(&["Julia"], &["jl"]);
pub const LUA_EXTS: Data<Strs, Strs> = Data(&["Lua"], &["lua"]);
pub const NIM_EXTS: Data<Strs, Strs> = Data(&["Nim"], &["nim"]);
pub const PERL_EXTS: Data<Strs, Strs> =
    Data(&["Perl"], &["plx", "pl", "pm", "xs", "t", "pod", "cgi"]);
pub const PURESCRIPT_EXTS: Data<Strs, Strs> = Data(&["PureScript"], &["ps"]);
pub const PYTHON_EXTS: Data<Strs, Strs> = Data(&["Python"], &["py"]);
pub const RACKET_EXTS: Data<Strs, Strs> = Data(&["Racket"], &["rkt"]);
pub const RAKU_EXTS: Data<Strs, Strs> =
    Data(&["Raku"], &["raku", "rakumod", "rakudoc", "t", "rakutest"]);
pub const RUBY_EXTS: Data<Strs, Strs> = Data(&["Ruby", "Rails"], &["rb"]);
pub const SCALA_EXTS: Data<Strs, Strs> = Data(&["Scala"], &["scala", "class"]);
pub const SCHEME_EXTS: Data<Strs, Strs> = Data(&["Scheme"], &["scm", "ss"]);
pub const SWIFT_EXTS: Data<Strs, Strs> = Data(&["Swift"], &["swift"]);
pub const TEX_EXTS: Data<Strs, Strs> = Data(&["TeX"], &["tex", "latex"]);

pub fn lang_ext_data() -> Vec<Data<Strs, Strs>> {
    vec![
        ADA_EXTS,
        C_EXTS,
        CPP_EXTS,
        CLOJURE_EXTS,
        COMMON_LISP_EXTS,
        D_EXTS,
        DART_EXTS,
        ELISP_EXTS,
        ELIXIR_EXTS,
        ERLANG_EXTS,
        ELM_EXTS,
        GO_EXTS,
        JAVA_EXTS,
        JULIA_EXTS,
        LUA_EXTS,
        NIM_EXTS,
        PERL_EXTS,
        PURESCRIPT_EXTS,
        PYTHON_EXTS,
        RACKET_EXTS,
        RAKU_EXTS,
        RUBY_EXTS,
        SCALA_EXTS,
        SCHEME_EXTS,
        SWIFT_EXTS,
        TEX_EXTS,
    ]
}

pub const IMG_TERMS: Data<Str, Strs> = Data(
    "Images",
    &[
        "img", "jpg", "jpeg", "png", "gif", "bmp", "svg", "picture", "pictures", "photo", "photos",
    ],
);

pub const JS_TS_TERMS: Data<Strs, Strs> = Data(
    &["Yeoman", "Node", "Vue", "VisualStudioCode"],
    &[
        "js",
        "javascript",
        "ts",
        "typescript",
        "node",
        "nodeJS",
        "package.json",
        "node_modules",
        "yarn",
        "gulp",
    ],
);

pub const RUST_TERMS: Data<Str, Strs> = Data(
    "Rust",
    &["Cargo.toml", "rustc", "Cargo", "rs", "crate", "crates"],
);

pub const HASKELL_TERMS: Data<Str, Strs> = Data(
    "Haskell",
    &[
        "Cabal",
        "Stack",
        "hs",
        "lhs",
        "hie",
        "hie.yaml",
        "ghc",
        "ghci",
        "runhaskell",
        "cabal-install",
        "hoogle",
        "hackage",
    ],
);
