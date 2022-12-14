use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::data::{self, Data, Str, Strs};

use super::term::Term;

const ASSETS_DIR: &'static str = "assets/gitignore";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Flag {
    Default,
    Community,
    Global,
}

impl Flag {
    pub fn prefix(&self) -> Option<&str> {
        match self {
            Flag::Default => None,
            Flag::Community => Some("community"),
            Flag::Global => Some("Global"),
        }
    }
    pub fn as_path(&self) -> PathBuf {
        let p: &Path = ASSETS_DIR.as_ref();
        if let Some(prefix) = self.prefix() {
            p.join(prefix)
        } else {
            p.to_path_buf()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AssetName(String, Flag);

impl AssetName {
    pub fn as_term(&self) -> Term {
        Term::intern(&self.0)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<S: AsRef<str>> PartialEq<S> for AssetName {
    fn eq(&self, other: &S) -> bool {
        self.0.as_str().eq_ignore_ascii_case(other.as_ref())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Asset {
    terms: Vec<Term>,
    git_ignore: GitIgnore,
}

impl Asset {
    pub fn terms(&self) -> &[Term] {
        &self.terms[..]
    }
    pub fn git_ignore(&self) -> &GitIgnore {
        &self.git_ignore
    }
    pub fn terms_iter(&self) -> std::slice::Iter<'_, Term> {
        self.terms.iter()
    }
    pub fn add_term(&mut self, term: Term) {
        if !self.terms.contains(&term) {
            self.terms.push(term);
        }
    }
    fn add_terms(&mut self, terms: &[Term]) {
        for term in terms {
            self.add_term(*term);
        }
    }
    pub fn has_term(&self, term: &Term) -> bool {
        self.terms.contains(term)
    }
    pub fn contents(&self) -> std::io::Result<String> {
        self.git_ignore.contents()
    }
}

impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.git_ignore())?;
        match self.terms() {
            [] => (),
            [a, bs @ ..] => {
                write!(f, " <~ {a}")?;
                for b in bs {
                    write!(f, ", {b}")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GitIgnore {
    name: String,
    flag: Flag,
}

impl std::fmt::Display for GitIgnore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_filepath().display())
    }
}

impl GitIgnore {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_flag(&self) -> Flag {
        self.flag
    }
    pub fn get_filepath(&self) -> PathBuf {
        self.flag.as_path().join(self.get_name())
    }
    pub fn contents(&self) -> std::io::Result<String> {
        std::fs::read_to_string(self.get_filepath())
    }
}

pub fn is_gitignore_file(p: impl AsRef<Path>) -> bool {
    matches!(
        p.as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s == "gitignore"),
        Some(true)
    )
}

/// Walks the static `assets` directory, collects the `.gitignore`
/// asset file metadata (whose list of terms contains only the
/// term corresponding to the file name without the extension, but
/// may later be populated) and returns the assets in an `AssetDB`.
pub fn walk_assets() -> AssetDB {
    use std::fs;
    let mut assets = HashMap::new();
    let mut paths = vec![Flag::Default, Flag::Community, Flag::Global]
        .into_iter()
        .map(|flag| {
            let path = flag.as_path();
            (path, flag)
        })
        .collect::<Vec<_>>();
    while let Some((p, flag)) = paths.pop() {
        for rd in fs::read_dir(p) {
            for de in rd.into_iter().flatten() {
                let path = de.path();
                if path.is_file() && is_gitignore_file(&path) {
                    let name = de.file_name().into_string().unwrap();
                    let asset_name =
                        AssetName(name.trim_end_matches(".gitignore").to_string(), flag);
                    let git_ignore = GitIgnore { name, flag };
                    let asset = Asset {
                        terms: vec![asset_name.as_term()],
                        git_ignore,
                    };
                    assets.insert(asset_name, asset);
                }
            }
        }
    }
    AssetDB { assets }
}

#[derive(Clone, Debug)]
pub struct AssetDB {
    assets: HashMap<AssetName, Asset>,
}

impl Default for AssetDB {
    fn default() -> Self {
        walk_assets()
    }
}

impl AssetDB {
    #![allow(unused)]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_decorated() -> Self {
        Self::new().decorated()
    }
    pub fn decorated(mut self) -> Self {
        AssetDecorator::new(&mut self).decorate_all();
        self
    }
    pub fn assets(&self) -> &HashMap<AssetName, Asset> {
        &self.assets
    }
    pub fn assets_mut(&mut self) -> &mut HashMap<AssetName, Asset> {
        &mut self.assets
    }
    pub fn get(&self, asset_name: &AssetName) -> Option<&Asset> {
        self.assets.get(asset_name)
    }
    pub fn get_mut(&mut self, asset_name: &AssetName) -> Option<&mut Asset> {
        self.assets.get_mut(asset_name)
    }
    pub fn entry(
        &mut self,
        asset_name: AssetName,
    ) -> std::collections::hash_map::Entry<'_, AssetName, Asset> {
        self.assets.entry(asset_name)
    }
    pub fn get_by_name(&self, name: impl AsRef<str>) -> Option<&Asset> {
        let name = name.as_ref();
        self.assets.iter().find_map(|(asset_name, asset)| {
            if asset_name.as_str().eq_ignore_ascii_case(&name) {
                Some(asset)
            } else {
                None
            }
        })
    }
    pub fn get_mut_by_name(&mut self, name: impl AsRef<str>) -> Option<&mut Asset> {
        let name = name.as_ref();
        self.assets.iter_mut().find_map(|(asset_name, asset)| {
            if asset_name.as_str().eq_ignore_ascii_case(&name) {
                Some(asset)
            } else {
                None
            }
        })
    }
    pub fn filter_by_name<'a>(
        &'a self,
        name: impl AsRef<str> + 'a,
    ) -> impl Iterator<Item = &'a Asset> + '_ {
        self.assets.iter().filter_map(move |(asset_name, asset)| {
            if asset_name.as_str().eq_ignore_ascii_case(name.as_ref()) {
                Some(asset)
            } else {
                None
            }
        })
    }
    pub fn filter_mut_by_name<'a>(
        &'a mut self,
        name: impl AsRef<str> + 'a,
    ) -> impl Iterator<Item = &'a mut Asset> + '_ {
        self.assets
            .iter_mut()
            .filter_map(move |(asset_name, asset)| {
                if asset_name.as_str().eq_ignore_ascii_case(name.as_ref()) {
                    Some(asset)
                } else {
                    None
                }
            })
    }
    pub fn get_by_names<'a, S>(
        &'a self,
        names: impl IntoIterator<Item = S> + 'a,
    ) -> impl Iterator<Item = &'a Asset> + '_
    where
        S: AsRef<str> + 'a,
    {
        names
            .into_iter()
            .flat_map(|s| self.get_by_name(s).into_iter())
    }

    pub fn filter_by_terms<'a>(
        &'a self,
        terms: &'a [Term],
    ) -> impl Iterator<Item = &'a Asset> + '_ {
        self.assets.iter().filter_map(|(asset_name, asset)| {
            if terms
                .iter()
                .chain(std::iter::once(&asset_name.as_term()))
                .any(|term| asset.has_term(term))
            {
                Some(asset)
            } else {
                None
            }
        })
    }
    pub fn filter_mut_by_terms<'a>(
        &'a mut self,
        terms: &'a [Term],
    ) -> impl Iterator<Item = &'a mut Asset> + '_ {
        self.assets.iter_mut().filter_map(|(asset_name, asset)| {
            if terms
                .iter()
                .chain(std::iter::once(&asset_name.as_term()))
                .any(|term| asset.has_term(term))
            {
                Some(asset)
            } else {
                None
            }
        })
    }
    pub fn add_terms_for_asset_by_names<I: IntoIterator<Item = N>, N: AsRef<str>>(
        &mut self,
        names: I,
        terms: &[Term],
    ) {
        for name in names {
            self.filter_mut_by_name(name)
                .for_each(|asset| asset.add_terms(terms))
        }
    }
}

struct AssetDecorator<'a>(&'a mut AssetDB);

impl<'a> AssetDecorator<'a> {
    pub fn new(asset_db: &'a mut AssetDB) -> Self {
        Self(asset_db)
    }

    pub fn run_with<I, N, K, S>(self, names: I, terms: K) -> Self
    where
        I: IntoIterator<Item = N>,
        N: AsRef<str>,
        K: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.0.add_terms_for_asset_by_names(
            names,
            Term::intern_iter(terms).collect::<Vec<_>>().as_slice(),
        );
        self
    }
    pub fn run_with_data(self, Data(names, terms): Data<Strs, Strs>) -> Self {
        self.run_with(names, terms)
    }
    pub fn run_with_lang(self, Data(name, terms): Data<Str, Strs>) -> Self {
        self.run_with([name], terms)
    }
    pub fn filetype_terms(self) -> Self {
        let mut this = self;
        for dt in data::lang_ext_data() {
            this = this.run_with_data(dt)
        }
        this
    }
    pub fn decorate_all(self) -> Self {
        self.run_with_data(data::JS_TS_TERMS)
            .run_with_lang(data::RUST_TERMS)
            .run_with_lang(data::HASKELL_TERMS)
            .run_with_lang(data::IMG_TERMS)
            .filetype_terms()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rust_gitignore_undecorated() {
        let assets = AssetDB::default();
        let asset_name = AssetName(String::from("Rust"), Flag::Default);
        let asset = Asset {
            terms: vec![Term::intern("Rust")],
            git_ignore: GitIgnore {
                name: String::from("Rust.gitignore"),
                flag: Flag::Default,
            },
        };
        assert_eq!(assets.get_by_name("Rust"), Some(&asset));
        assert_eq!(assets.get(&asset_name), assets.get_by_name("Rust"));
    }
}
