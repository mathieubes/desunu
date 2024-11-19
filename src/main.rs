mod file_handler;
mod project;

fn main() {
    println!(
        "Running in {} folder.",
        std::env::current_dir().unwrap().display()
    );

    //let walker = WalkDir::new(".").into_iter();
    //for entry in walker {
    //    let entry = entry.unwrap();
    //    if entry
    //        .file_name()
    //        .to_str()
    //        .map(is_code_file)
    //        .unwrap_or(false)
    //    {
    //        let f = std::fs::File::open(entry.path().to_str().unwrap()).unwrap();
    //        let buf = read_file(f).unwrap();
    //        let mut used_package_indexes = Vec::new();
    //        for (i, package_name) in package_names.iter().enumerate() {
    //            if string_exists_in_multiline_text(package_name, &buf) {
    //                used_package_indexes.push(i);
    //            }
    //        }
    //
    //        let used_package_names: Vec<&String> = used_package_indexes
    //            .iter()
    //            .map(|i| &package_names[*i])
    //            .collect();
    //
    //        println!("{}", entry.path().display());
    //        println!("\tPackages used in this file : {:?}", used_package_names);
    //        println!("\tPackages still not used : {}", package_names.len());
    //
    //        for (offset, i) in used_package_indexes.iter().enumerate() {
    //            package_names.remove(*i - offset);
    //        }
    //    }
    //}

    //println!("packages not used : {:?}", package_names);
}
