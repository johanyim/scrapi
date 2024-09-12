```rust
use std::fs;
const HOME_DIR: &'static str = "/home/johan/";
#[rustfmt::skip]
fn list_dotfile_paths(
    dir_name: &'static str,
) -> impl Iterator<Item = impl Iterator<Item = String>> {
    let directories = HOME_DIR.split_inclusive("/").map(String::from);

    // TODO: Handle file not found Error
    let paths = fs::read_dir(HOME_DIR)
        .unwrap().flatten()
        .filter_map(move |file| {
            file.path()
                .strip_prefix(HOME_DIR).ok()
                .and_then(|relative_path| relative_path.to_str().map(String::from))

        })
        .filter(|x| x.starts_with('.'))


    // NOTE: includes directories
    paths.map(move |path| directories.clone().chain(std::iter::once(path)))
}

```



