use crate::error::{Error, Result};
use crate::manifest::Name;
use crate::{Args, Test};
use std::collections::BTreeMap as Map;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct ExpandedTest<'a> {
    pub name: Name,
    pub test: Test,
    pub error: Option<Error>,
    pub args: &'a Args,
    is_from_glob: bool,
}

pub(crate) fn expand_globs<'a>(tests: &'a [(Test, Args)]) -> Vec<ExpandedTest<'a>> {
    let mut set = ExpandedTestSet::new();

    for (test, args) in tests {
        match test.path.to_str() {
            Some(utf8) if utf8.contains('*') => match glob(utf8) {
                Ok(paths) => {
                    let expected = test.expected;
                    for path in paths {
                        set.insert(Test { path, expected }, args, None, true);
                    }
                }
                Err(error) => set.insert(test.clone(), args, Some(error), false),
            },
            _ => set.insert(test.clone(), args, None, false),
        }
    }

    set.vec
}

struct ExpandedTestSet<'a> {
    vec: Vec<ExpandedTest<'a>>,
    path_to_index: Map<PathBuf, usize>,
}

impl<'a> ExpandedTestSet<'a> {
    fn new() -> Self {
        ExpandedTestSet {
            vec: Vec::new(),
            path_to_index: Map::new(),
        }
    }

    fn insert(&mut self, test: Test, args: &'a Args, error: Option<Error>, is_from_glob: bool) {
        if let Some(&i) = self.path_to_index.get(&test.path) {
            let prev = &mut self.vec[i];
            if prev.is_from_glob {
                prev.test.expected = test.expected;
                return;
            }
        }

        let index = self.vec.len();
        let name = Name(format!("trybuild{:03}", index));
        self.path_to_index.insert(test.path.clone(), index);
        self.vec.push(ExpandedTest {
            name,
            test,
            error,
            args,
            is_from_glob,
        });
    }
}

fn glob(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut paths = glob::glob(pattern)?
        .map(|entry| entry.map_err(Error::from))
        .collect::<Result<Vec<PathBuf>>>()?;
    paths.sort();
    Ok(paths)
}
