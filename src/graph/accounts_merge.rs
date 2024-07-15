use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn dfs(
        email: String,
        email_to_account_indices: &HashMap<String, Vec<usize>>,
        visited_emails: &mut HashSet<String>,
        accounts: &Vec<Vec<String>>,
    ) -> Vec<String> {
        if !visited_emails.insert(email.clone()) {
            return vec![];
        }

        let mut merged_emails = vec![email.clone()];
        if let Some(account_indices) = email_to_account_indices.get(&email) {
            for &index in account_indices {
                let emails = &accounts[index][1..];
                for e in emails {
                    merged_emails.extend(Solution::dfs(
                        e.clone(),
                        email_to_account_indices,
                        visited_emails,
                        accounts,
                    ));
                }
            }
        }

        merged_emails
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_account_indices: HashMap<String, Vec<usize>> = HashMap::new();
        let mut visited_emails: HashSet<String> = HashSet::new();

        for (index, account) in accounts.iter().enumerate() {
            for email in &account[1..] {
                email_to_account_indices
                    .entry(email.clone())
                    .or_insert_with(Vec::new)
                    .push(index);
            }
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        for account in &accounts {
            let account_name = &account[0];
            for email in &account[1..] {
                if !visited_emails.contains(email) {
                    let mut email_group = Solution::dfs(
                        email.clone(),
                        &email_to_account_indices,
                        &mut visited_emails,
                        &accounts,
                    );
                    if !email_group.is_empty() {
                        email_group.sort_unstable();
                        email_group.insert(0, account_name.clone());
                        result.push(email_group);
                    }
                }
            }
        }

        result
    }
}
