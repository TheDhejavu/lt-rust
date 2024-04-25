use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Twitter {
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    following: HashMap<i32, Vec<i32>>,
    count: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Self{tweets: HashMap::new(), count: 0 , following: HashMap::new()}
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.count += 1;
        let mut tweets = self.tweets.entry(user_id).or_insert(vec![]);
        tweets.push((self.count, tweet_id))
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut max_heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut result = vec![];
        if let Some(tweets) = self.tweets.get(&user_id) {
            max_heap.extend(tweets);
        }

        if let Some(following) = self.following.get(&user_id)  {
            for f in following {
                if let Some(tweets) = self.tweets.get(&f) {
                    max_heap.extend(tweets);
                }
            }
        }
        while max_heap.len() > 0 && result.len() < 10 {
            let r = max_heap.pop().unwrap();
            result.push(r.1);
        }
        result
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(following) = self.following.get(&follower_id) {
            if following.contains(&followee_id) {
                return
            }
        }
        let mut following = self.following.entry(follower_id).or_insert(vec![]);
        following.push(followee_id)
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(following) = self.following.get_mut(&follower_id) {
            following.retain(|&x| x != followee_id);
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */