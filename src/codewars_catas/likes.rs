fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => {
            let others_count = names.len() - 2;

            format!("{}, {} and {} others like this", names[0], names[1], others_count)
        }
    }
}