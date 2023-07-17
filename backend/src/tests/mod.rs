pub mod authentication_test;
pub mod prediction_test;
pub mod queuing_test;

#[test]
fn solution() {
    let word: &str = "abc";
    let ending: &str = "d";
    let data = String::from(word);
    assert!(data.contains(ending))
    // assert_eq!(false, data.contains(ending));
}