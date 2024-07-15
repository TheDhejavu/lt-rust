const UN_VISITED: i32 = 0;
const VISITING: i32 = 1;
const VISITED: i32 = 2;

impl Solution {
    pub fn has_cycle(visit_state: &mut Vec<i32>, adj_list: &Vec<Vec<i32>> , course: usize) -> bool {
        if visit_state[course] == VISITING {
            return true;
        }

        if visit_state[course] == VISITED {
            return false;
        }

        visit_state[course] = VISITING;
        for neigbor in &adj_list[course] {
            if Solution::has_cycle(visit_state, adj_list, *neigbor as usize) {
                return true;
            }
        }
        visit_state[course] = VISITED;
        return false;
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adj_list: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
        for prerequisite in prerequisites {
            adj_list[prerequisite[0] as usize].push(prerequisite[1]);
        }

        let mut visit_state: Vec<i32> = vec![0; num_courses as usize];
        for course in 0..num_courses {
            if Solution::has_cycle(&mut visit_state, &adj_list, course as usize) {
                return false;
            }
        }
        return true;
    }
}