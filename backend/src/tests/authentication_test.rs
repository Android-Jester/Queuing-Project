// use crate::{data::models::UserQuery, data_source::db_actions::*};
// use diesel::prelude::*;
// #[test]
// fn test_db() {
//     let connection = &mut establish_conn();
//     let results = crate::data::schema::Users::dsl::Users
//         .select(UserQuery::as_select())
//         .load(connection)
//         .expect("Error Reading");

//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{}", post.national_id);
//         // println!("-----------\n");
//         // println!("{}", post.password);
//     }
// }
// #[test]
// fn test_insert() {
//     let account_number = format!("1111111122dd222");
//     let password = format!("xyz");
//     // let data = signup_user(&mut establish_connection(), account_number, password);
//     // println!("data: {}", data);
// }

// #[test]
// fn login_user_test() {
//     let user_name = format!("1111111122dd222");
//     let password = format!("xyz");
//     // let user = login_user();
//     // println!("User: {:#?}", user);
// }
