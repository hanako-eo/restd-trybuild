const IGNORED_LINTS: &[&str] = &["dead_code"];

pub(crate) fn toml<'s, I>(extra_flags: I) -> toml::Value
where
    I: IntoIterator<Item = &'s str>,
{
    let mut rustflags = vec!["--cfg", "trybuild", "--verbose"];
    rustflags.extend(extra_flags);

    for &lint in IGNORED_LINTS {
        rustflags.push("-A");
        rustflags.push(lint);
    }

    toml::Value::try_from(rustflags).unwrap()
}
