use std::collections::HashMap;

pub fn naive(n: u32) -> u128 {
    if n <= 1 {
        return n as u128;
    }
    naive(n - 1) + naive(n - 2)
}

pub fn memo(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
    if n <= 1 {
        println!("n <= 1 | '{n}'");
        return n as u128;
    }
    let l = if let Some(existing) = cache.get(&(n - 1)) {
        *existing
    } else {
        memo(n - 1, cache)
    };
    let r = if let Some(existing) = cache.get(&(n - 2)) {
        *existing
    } else {
        memo(n - 2, cache)
    };

    let ans = l + r;
    cache.insert(n, ans);
    println!("ans = '{ans}' | '{n}'");
    ans
}

pub fn memo2(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
    if n <= 1 {
        cache.insert(n, n as u128);
        println!("n <= 1 | '{n}'");
        return n as u128;
    }

    let left = cache
        .get(&(n - 1))
        .copied()
        .unwrap_or_else(|| memo2(n - 1, cache));

    let right = cache
        .get(&(n - 2))
        .copied()
        .unwrap_or_else(|| memo2(n - 2, cache));

    let ans = left + right;
    cache.insert(n, ans);
    println!("ans = '{ans}' | '{n}'");
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::fib_naive1(12, 144)]
    #[case::fib_naive2(23, 28657)]
    //#[case::fib_naive3(80, 23_416_728_348_467_685)]
    fn test_naive(#[case] n: u32, #[case] expect: u128) {
        assert_eq!(naive(n), expect);
    }

    #[rstest]
    #[case::fib_memo1(12, 144)]
    #[case::fib_memo2(23, 28657)]
    #[case::fib_memo3(80, 23_416_728_348_467_685)]
    fn test_memo(#[case] n: u32, #[case] expect: u128) {
        let mut cache = HashMap::new();
        assert_eq!(memo(n, &mut cache), expect);
    }

    #[rstest]
    #[case::fib_memo1(2, 1)]
    //#[case::fib_memo1(12, 144)]
    //#[case::fib_memo2(23, 28657)]
    //#[case::fib_memo3(80, 23_416_728_348_467_685)]
    fn test_memo2(#[case] n: u32, #[case] expect: u128) {
        let mut cache = HashMap::new();
        assert_eq!(memo2(n, &mut cache), expect);
    }
}
