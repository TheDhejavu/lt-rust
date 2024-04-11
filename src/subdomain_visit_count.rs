impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut subdomains_mapping: std::collections::HashMap<String , i32> = std::collections::HashMap::new();
        for cpdomain in &cpdomains {
            let mut split_cpdomain: Vec<&str> = cpdomain.split_whitespace().collect();
            let subdomains = split_cpdomain[1];
            let visit_count = split_cpdomain[0];
            let count: i32 = visit_count.parse().unwrap();

            let mut split_subdomains: Vec<&str> = subdomains.split(".").collect();
            for i in 0..split_subdomains.len() {
                let subdomain = &split_subdomains[i..];
                let domain_from_string_slice = subdomain.join(".");
                let mut mapping_result = subdomains_mapping.entry(domain_from_string_slice.to_string()).or_insert(0);
                *mapping_result += count;
            }
        }
        subdomains_mapping.into_iter().map(|(subdomain, count)| -> String {
            format!("{count} {subdomain}")
        }).collect()
    }
}