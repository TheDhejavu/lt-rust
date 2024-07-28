use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
        
        // Build the adjacency list
        for ticket in tickets {
            adj_list.entry(ticket[0].clone())
                .or_insert_with(Vec::new)
                .push(ticket[1].clone());
        }
        
        // Sort destinations in lexicographical order
        for destinations in adj_list.values_mut() {
            destinations.sort_by(|a, b| b.cmp(a));  
        }
        
        let mut result: Vec<String> = Vec::new();
        
        // DFS function
        fn dfs(
            airport: &str, 
            adj_list: &mut HashMap<String, Vec<String>>, 
            result: &mut Vec<String>
        ) {
            while let Some(destination) = adj_list.get_mut(airport).and_then(|dests| dests.pop()) {
                dfs(&destination, adj_list, result);
            }
            result.push(airport.to_string());
        }
        
        // Start DFS from JFK
        dfs("JFK", &mut adj_list, &mut result);
        
        result.reverse();
        result
    }
}