use auth::establish_connection;
use auth::models::User;
use auth::{login_user, signup_user};
use diesel::prelude::*;
#[test]
fn test_db() {
    let connection = &mut establish_connection();
    let results = auth::schema::users::dsl::users
        .select(User::as_select())
        .load(connection)
        .expect("Error Reading");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.account_number);
        println!("-----------\n");
        println!("{}", post.password);
    }
}
#[test]
fn test_insert() {
    let account_number = format!("1111111122dd222");
    let password = format!("xyz");
    let data = signup_user(&mut establish_connection(), account_number, password);
    println!("data: {}", data);
}

#[test]
fn login_user_test() {
    let user_name = format!("1111111122dd222");
    let password = format!("xyz");
    let user = login_user(&mut establish_connection(), user_name, password);
    println!("User: {:#?}", user);
}
