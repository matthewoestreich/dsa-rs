pub fn climb(n: i32, dp: Option<&mut Vec<i32>>) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }

    let nidx = n as usize;
    let dp = if let Some(dp) = dp {
        dp
    } else {
        &mut vec![0; nidx + 1]
    };

    if dp[nidx] != 0 {
        return dp[nidx];
    }

    dp[nidx] = climb(n - 1, Some(dp)) + climb(n - 2, Some(dp));
    dp[nidx]
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::climb_stairs0(-1, 0)]
    #[case::climb_stairs1(0, 1)]
    #[case::climb_stairs2(1, 1)]
    #[case::climb_stairs3(2, 2)]
    #[case::climb_stairs4(3, 3)]
    #[case::climb_stairs5(4, 5)]
    #[case::climb_stairs6(5, 8)]
    #[case::climb_stairs7(6, 13)]
    #[case::climb_stairs8(7, 21)]
    #[case::climb_stairs9(10, 89)]
    #[case::climb_stairs10(15, 987)]
    #[case::climb_stairs11(20, 10946)]
    #[case::climb_stairs12(30, 1346269)]
    #[case::climb_stairs13(35, 14930352)]
    #[case::climb_stairs14(40, 165580141)]
    #[case::climb_stairs15(45, 1836311903)]
    fn test_climb_stairs(#[case] n: i32, #[case] expect: i32) {
        assert_eq!(climb(n, None), expect);
    }
}
