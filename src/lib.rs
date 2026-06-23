
// ============================================================
// Approach 1: Naive Border Check
// ============================================================
// Checks for every possible length from 1 to n-1. 
// Then it compares the prefix and suffix slices directly for each.
//
// Time: O(n²) | Space: O(1)
// ============================================================
pub fn naive_border_check(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut result = Vec::new();
    for len in 1..n {
        if &s[..len] == &s[n - len..] {
            result.push(len);
        }
    }
    result
}

// ============================================================
// Approach 2: KMP Border Compute
// ============================================================
// Computes the prefix function π, then follows π[n-1] to
// collect all border lengths. 
//
// Time: O(n) | Space: O(n)
// ============================================================
pub fn kmp_border_compute(s: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let n = s.len();
    if n == 0 {
        return vec![];
    }

    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }

    let mut borders = Vec::new();
    let mut k = pi[n - 1];
    while k > 0 {
        borders.push(k);
        k = pi[k - 1];
    }
    borders.reverse();
    borders
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let s = "abcababcab";
        assert_eq!(naive_border_check(s), vec![2, 5]);
        assert_eq!(kmp_border_compute(s), vec![2, 5]);
    }

    #[test]
    fn test_no_border() {
        let s = "abc";
        assert!(naive_border_check(s).is_empty());
        assert!(kmp_border_compute(s).is_empty());
    }

    #[test]
    fn test_all_same() {
        let s = "aaaa";
        assert_eq!(naive_border_check(s), vec![1, 2, 3]);
        assert_eq!(kmp_border_compute(s), vec![1, 2, 3]);
    }

    #[test]
    fn both_agree() {
        let strings = ["abcababcab", "abc", "aaaa", "abab", "abcabcabc"];
        for s in strings {
            assert_eq!(naive_border_check(s), kmp_border_compute(s));
        }
    }
}