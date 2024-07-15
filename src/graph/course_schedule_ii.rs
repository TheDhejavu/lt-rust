const UN_VISITED: i32 = 0;
const VISITING: i32 = 1;
const VISITED: i32 = 2;

impl Solution {
    pub fn dfs_has_cycle(visit_state: &mut Vec<i32>, adj_list: &Vec<Vec<i32>> , course: usize, result: &mut Vec<i32>) -> bool {
        if visit_state[course] == VISITING {
            return true;
        }

        if visit_state[course] == VISITED {
            return false;
        }

        visit_state[course] = VISITING;
        for neigbor in &adj_list[course] {
            if Solution::dfs_has_cycle(visit_state, adj_list, *neigbor as usize, result) {
                return true;
            }
        }
        visit_state[course] = VISITED;
        result.push(course as i32);
        return false;
    }
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj_list: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
        let p: Vec<(i32, i32)> = prerequisites.iter().map(|x| (x[0], x[1])).collect();
        for (ca, cb) in p {
            adj_list[ca as usize].push(cb);
        }
        let mut result: Vec<i32> = vec![];
        let mut visit_state: Vec<i32> = vec![0; num_courses as usize];
        for course in 0..num_courses {
            if Solution::dfs_has_cycle(&mut visit_state, &adj_list, course as usize, &mut result) {
                return vec![];
            }
        }
        
        return result;
    }
}