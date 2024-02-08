
struct  Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_string();
        }

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut t_count = [0; 128]; 
        let mut window_set = [0; 128];
        let mut result = (0, s.len() + 1);

        for &byte in t_bytes.iter() {
            t_count[byte as usize] += 1;
        }

        let mut seen = 0;
        let mut need = t.len();
        let mut left = 0;

        for (right, &byte) in s_bytes.iter().enumerate() {
            window_set[byte as usize] += 1;

            if t_count[byte as usize] > 0 && window_set[byte as usize] <= t_count[byte as usize] {
                seen += 1;
            }

            while seen == need {
                if right - left + 1 < result.1 - result.0 + 1{
                    result = (left, right);
                }

                window_set[s_bytes[left] as usize] -= 1;

                if t_count[s_bytes[left] as usize] > 0 && window_set[s_bytes[left] as usize] < t_count[s_bytes[left] as usize] {
                    seen -= 1;
                }

                left += 1;
            }
        }

        if result.1 - result.0 <= s.len() {
            return s[result.0..=result.1].to_string();
        }

        "".to_string()
    }
}
