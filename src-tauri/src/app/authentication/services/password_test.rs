#[test]
fn test_hash_password() {
    let mut password = "password".to_string();

    let result = crate::app::authentication::services::password::hash_password(password.clone());

    println!("{:?}", result);

    let verif = crate::app::authentication::services::password::verify_password(
        password.as_str(),
        result.unwrap().as_str(),
    );
    println!("{:?}", verif.unwrap());
}
