impl Solution {
    pub fn find(parent: &mut Vec<i32>, rank: &mut Vec<i32>, i: i32) -> i32 {
        // when index (child) is thesame as the value (parent) then
        // we know that index (i) is its own parent, which means we have gotten to the root of the tree.
        if parent[i as usize] != i {
            return Solution::find(parent, rank, parent[i as usize]);
        }
        return parent[i as usize];
    }
    pub fn union(parent: &mut Vec<i32>, rank: &mut Vec<i32>, p: i32, q: i32) -> bool {
        let root_q = Solution::find(parent, rank, parent[q as usize]);
        let root_p = Solution::find(parent, rank, parent[p as usize]);
        if root_q != root_p {
            if rank[root_p as usize] > rank[root_q as usize] {
                parent[root_q as usize] = root_p;
            } else if rank[root_p as usize] < rank[root_q as usize] {
                parent[root_p as usize] = root_q;
            } else {
                parent[root_q as usize] = root_p;
                rank[root_p as usize] += 1;
            }

            return true;
        }
        // by returning false,we know that the p & q already exist and has already been grouped into
        // a set
        return false;
    }
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = (edges.len() + 1) as i32;
        let mut rank: Vec<i32> = vec![1; n as usize];
        let mut parent: Vec<i32> = (0..=n).collect();
        let data: Vec<(i32, i32)> = edges.iter().map(|x| (x[0], x[1])).collect();
        for (p, q) in data {
            if !Solution::union(&mut parent, &mut rank, p, q) {
                println!("{:?}", parent);
                return vec![p, q];
            }
        }
        vec![]
    }
}
