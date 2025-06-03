#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;

const N:usize =  50;
// simple twitter clone
//
// news feed : merge k with a min-heap
// follow / unfollow HashMap
// post tweet : HashMap<VecDeque> could be rolling buffer make static size (recent tweets)

type UserId = usize;
type TweetId = usize;
type UnixTime = usize;

#[derive(Default, Clone)]
struct Recent {
    history: VecDeque<(TweetId, UnixTime)>,
}

struct Tweet {
    user_id: UserId,
    tweet_id: TweetId,
}

impl Recent {
    fn new() -> Self {
        Recent { history: VecDeque::with_capacity(N) }
    }
    fn push(&mut self, tweet_id:TweetId, time:UnixTime) {
        if self.history.len() >= N {
            self.history.pop_back();
        }
        self.history.push_front((tweet_id, time));
    }
    fn pop(&mut self) -> Option<(TweetId, UnixTime)> {
        self.history.pop_front()
    }
}

struct Twitter {
    follows: HashMap<UserId, HashSet<UserId>>,
    tweets: HashMap<UserId, Recent>,
    time: UnixTime, 
}

#[derive(Eq, PartialEq)]
struct NewsHeapEntry {
    user_id:UserId,
    tweet_id:TweetId,
    time:UnixTime,
}

impl NewsHeapEntry {
    fn new(user_id:UserId, tweet_id:TweetId, time:UnixTime) -> Self {
        Self { user_id, tweet_id, time }
    }
}

impl Ord for NewsHeapEntry {
    fn cmp(&self, other:&Self) -> Ordering {
        // default cmp is self > other
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for NewsHeapEntry {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Clone)]
enum TwitApi {
    Post,
    Follow,
    Unfollow,
    NewsFeed,
}

impl Twitter {
    fn new() -> Self {
        // UnixTime here just for simple prototyping will be replaced with calls
        Self { follows:HashMap::new(), tweets:HashMap::new(), time:0 }
    }
    fn publish(&mut self, user_id:UserId, tweet_id:TweetId) {
        self.tweets.entry(user_id).or_default().push(tweet_id, self.time);
        self.time+=1;

    }
    fn follow(&mut self, follower_id:UserId, followee_id:UserId) {
        self.follows.entry(follower_id).or_default().insert(followee_id);
    }
    fn unfollow(&mut self, follower_id:UserId, followee_id:UserId) {
        self.follows.entry(follower_id).or_default().remove(&followee_id);
    }
    fn news_feed(&mut self, user_id:UserId) -> Vec<(UserId, TweetId)> {
        let mut news: HashMap<UserId, Recent> = HashMap::new();
        if let Some(subscribes) = self.follows.get(&user_id) {
            for ldr in subscribes {
                if let Some(twts) = self.tweets.get(ldr) {
                    news.insert(*ldr, twts.clone());
                }
            }
        }
        self._nf_merge_k_(10, &mut news)
    }

    fn _nf_merge_k_(&mut self, k:usize, subscribes:&mut HashMap<UserId, Recent>) -> Vec<(UserId, TweetId)> {
        let mut sorted:BinaryHeap<NewsHeapEntry>= BinaryHeap::new();
        let mut recents:Vec<(UserId, TweetId)> = Vec::with_capacity(k);

        for (user, history) in subscribes.into_iter() {
            if let Some((tweet_id, time)) = history.pop() {
                sorted.push(NewsHeapEntry::new(*user, tweet_id, time));
            }
        }
        // prototype -- could have number of availables and decrement to break early
        for _ in 0..k {
            if let Some(tweet) =  sorted.pop() {
                if let Some(unseen)  = subscribes.get_mut(&tweet.user_id) {
                    if let Some(r) = unseen.pop() {
                        sorted.push(NewsHeapEntry::new(tweet.user_id, r.0, r.1));
                    }
                }
                recents.push((tweet.user_id, tweet.tweet_id))
            }
        }
        recents
    }
}


fn testing_interface(actions: Vec<TwitApi>, parms: Vec<Vec<usize>>) {
    let mut app = Twitter::new();
    println!("Results:");
    for i in 0..actions.len() {
        let result = testing_action(&mut app, actions[i].clone(), parms[i].clone());
        println!("-----");
        if let Some(news) = result {
            println!("NewsFeed: {:?}", news);
        }
    }
}

fn testing_action(app: &mut Twitter, action: TwitApi, parms: Vec<usize>) -> Option<Vec<(UserId, TweetId)>> {
    match action {
        TwitApi::Post => {
            app.publish(parms[0], parms[1]);
            None
        },
        TwitApi::Follow => {
            app.follow(parms[0], parms[1]);
            None
        },
        TwitApi::Unfollow => {
            app.unfollow(parms[0], parms[1]);
            None
        },
        TwitApi::NewsFeed => {
            Some(app.news_feed(parms[0]))
        },
    }
}
fn main() {
    use TwitApi::*;

    let actions = vec![
        Post,           // user 1 posts
        Post,           // user 2 posts
        NewsFeed,       // user 1 asks for news
        Follow,         // user 2 follows 1
        Follow,         // user 2 follows 3
        Post,           // user 3 posts
        Follow,         // user 1 follows 3
        NewsFeed,       // get news (1)
        NewsFeed,       // get news (2)
        Unfollow,       // user 2 unfollows 1
        NewsFeed,       // get news (2)
    ];

    let params = vec![
        vec![1, 100],    // Post by user 1
        vec![2, 200],    // Post by user 2
        vec![1],         // NewsFeed for user 1
        vec![2, 1],      // user 2 follows 1
        vec![2, 3],      // user 2 follows 3
        vec![3, 300],    // Post by user 3
        vec![1, 3],      // user 1 follows 3
        vec![1],         // NewsFeed for user 1
        vec![2],         // NewsFeed for user 2
        vec![2, 1],      // user 2 unfollows 1
        vec![2],         // NewsFeed for user 2
    ];

    testing_interface(actions, params);
}


// fn main() {
//     let actions:Vec<TwitApi> = vec![TwitApi::Post, TwitApi::NewsFeed, TwitApi::Follow, TwitApi::Post, TwitApi::NewsFeed, TwitApi::Unfollow, TwitApi::NewsFeed];
//     let parms = vec![vec![1, 5], vec![1], vec![1, 2], vec![2, 6], vec![1], vec![1, 2], vec![1]];
//     testing_interface(actions, parms);
// }

