#[cfg(test)]

use super::hello;

#[tokio::main]
#[test]
async fn hello_test() {
    assert_eq!(hello().await, "hello");
}
