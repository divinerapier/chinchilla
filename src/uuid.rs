pub fn v4() -> String {
    let u = uuid::Uuid::new_v4();
    let u = u.to_hyphenated().to_string();
    u
}
