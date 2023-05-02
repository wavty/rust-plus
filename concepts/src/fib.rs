// 斐波那契数列实践
fn fib_for(n: u8) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    for _i in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

fn fib_loop(n: u8) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut i = 0;
    loop {
        if i == n {
            break;
        }
        c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    c
}

fn fib_while(n: u8) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut i = 0;
    while i < n {
        c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_for() {
        assert_eq!(fib_for(0), 1);
        assert_eq!(fib_for(2), 2);
        assert_eq!(fib_for(3), 3);
        assert_eq!(fib_for(5), 8);
        assert_eq!(fib_for(9), 55);
        assert_eq!(fib_for(10), 89);
    }

    #[test]
    fn test_fib_loop() {
        assert_eq!(fib_loop(0), 1);
        assert_eq!(fib_loop(2), 2);
        assert_eq!(fib_loop(3), 3);
        assert_eq!(fib_loop(5), 8);
        assert_eq!(fib_loop(9), 55);
        assert_eq!(fib_loop(10), 89);
    }

    #[test]
    fn test_fib_while() {
        assert_eq!(fib_while(0), 1);
        assert_eq!(fib_while(2), 2);
        assert_eq!(fib_while(3), 3);
        assert_eq!(fib_while(5), 8);
        assert_eq!(fib_while(9), 55);
        assert_eq!(fib_while(10), 89);
    }
}
