pub fn rob(houses: Vec<u32>) -> u32 {
    let mut prev1 = 0;
    let mut prev2 = 0;

    for &n in houses.iter() {
        let c = prev1.max(prev2 + n);
        prev2 = prev1;
        prev1 = c;
    }

    prev1
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::rob1(vec![2, 7, 9, 3, 1], 12)]
    #[case::rob2(vec![1], 1)]
    #[case::rob3(vec![1, 2], 2)]
    #[case::rob4(vec![2, 1, 1, 2], 4)]
    #[case::rob4_2(vec![1, 2, 1, 2], 4)]
    #[case::rob4_3(vec![2, 1, 6, 2], 8)]
    #[case::rob5(vec![5, 5, 10, 100, 10, 5], 110)]
    #[case::rob6(vec![], 0)]
    fn test_robber(#[case] houses: Vec<u32>, #[case] expect: u32) {
        assert_eq!(rob(houses), expect);
    }
}
