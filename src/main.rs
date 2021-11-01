use std::path::PathBuf;
use regex::Regex;
use std::fs;
use std::fs::DirEntry;

fn find_files_in_order(dir: &str, filename_to_key: Box<Fn(&PathBuf) -> Option<i32>>) -> Vec<PathBuf> {
    let mut all_paths : Vec<(PathBuf, i32)> = fs::read_dir(dir)
        .unwrap_or_else(|err| {panic!("Unable to read {}: {} ", dir, err)})
        .filter(|result| result.is_ok()).map(|result| result.unwrap().path())
        .map(|p| {
            let k = filename_to_key(&p);
            (p, k)
        })
        .filter(|(_, k)| k.is_some())
        .map(|(p, k)| (p, k.unwrap()))
        .collect();
    all_paths.sort_by(|(p1, k1), (p2, k2)| k1.cmp(k2));
    return all_paths.into_iter().map(|(p, _)| p).collect();
}


fn main() {
    let fn_regex: Regex = Regex::new("([0-9]+).csv").unwrap();

    let get_index_from_filename : Box<Fn(&PathBuf) -> Option<i32>> = Box::new(move |path: &PathBuf| {
        
        let filename: &str = path.file_name().unwrap().to_str().unwrap();

        match fn_regex.captures(filename) {
            Some(captures) => {
                captures.get(1).and_then(|num_str| {
                    match num_str.as_str().parse() {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    }
            })},
            None => None,
        }
    });

    find_files_in_order("./test_dir/", get_index_from_filename)
        .into_iter()
        .for_each(|path: PathBuf| {
            println!("{}", path.into_os_string().into_string().unwrap());
        });
}
