pub fn solution(current: &str) -> String {
    match current {
        "green" => "yellow".to_owned(),
        "yellow" => "red".to_owned(),
        "red" => "green".to_owned(),
        _ => "error".to_owned()
    }
}