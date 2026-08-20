use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

struct Twitter {
    count: i32,
    tweet_map: HashMap<i32, Vec<(i32, i32)>>, // user -> (time, tweet_id)
    follow_map: HashMap<i32, HashSet<i32>>, // user -> user's following
}

impl Twitter {
    pub fn new() -> Self {
        Self {
            count: i32::MIN,
            tweet_map: HashMap::new(),
            follow_map: HashMap::new(),
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweet_map
            .entry(user_id).or_insert_with(Vec::new)
            .push((self.count, tweet_id));
        self.count += 1;
    }

    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        if let Some(followee_list) = self.follow_map.get(&user_id) {
            for followee_id in followee_list.iter() {
                if let Some(tweet_list) = self.tweet_map.get(followee_id) {
                    for &tweet in tweet_list {
                        min_heap.push(Reverse(tweet));
                        if min_heap.len() > 10 {
                            min_heap.pop();
                        }
                    }
                }
            }
        }

        if let Some(tweet_list) = self.tweet_map.get(&user_id) {
            for &tweet in tweet_list {
                min_heap.push(Reverse(tweet));
                if min_heap.len() > 10 {
                    min_heap.pop();
                }
            }
        }

        let mut res = Vec::with_capacity(10);
        while let Some(Reverse((_, tweet_id))) = min_heap.pop() {
            res.push(tweet_id);
        }
        res.reverse();
        res
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.follow_map
            .entry(follower_id).or_insert_with(HashSet::new)
            .insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.follow_map.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}
