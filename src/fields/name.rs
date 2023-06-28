use calamine::DataType;

pub fn getting_results_name(
    name: &str,
    res: Vec<Vec<DataType>>,
) -> Option<(Vec<Vec<DataType>>, usize)> {
    // let res = merging_content_in_files(&get_file());
    let results = res
        .into_iter()
        .filter(|row| {
            row[1]
                .get_string()
                .unwrap()
                .contains(name.to_uppercase().as_str())
                || row[2]
                    .get_string()
                    .unwrap()
                    .contains(name.to_uppercase().as_str())
        })
        .collect::<Vec<_>>();
    if !results.is_empty() {
        Some((results.clone(), results.len()))
    } else {
        None
    }
}
