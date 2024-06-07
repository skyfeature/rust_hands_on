use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn works() {
    assert!(true);

    let a = 1;
    let b = 2;
    let c : i32 = a + b;
    assert_eq!(c, 3);
    assert_eq!(c, 3, "we are testing addition of a = {}, b = {} to be equal to {c}", a, b);
}

#[test]
fn test_ls() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn test_main() {
    let mut cmd = Command::cargo_bin("rust_hands_on").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");

}


#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![2, 4, 9];
        
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), Some(&9));
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}