use integration_test::add_two;

#[test]
fn test_run(){
  let result = add_two(2);
  assert_eq!(result, 4);
}