use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Worker {
    pub worker: String,
    pub worker_name: String,
    pub worker_wallet: String,
    pub rpc_id: u64,
    pub hash: u64,
    pub share_index: u128,
    pub accept_index: u128,
    pub invalid_index: u128,
}

impl Worker {
    pub fn new(worker: String, worker_name: String, worker_wallet: String) -> Self {
        Self {
            worker,
            worker_wallet,
            worker_name,
            hash: 0,
            share_index: 0,
            accept_index: 0,
            invalid_index: 0,
            rpc_id: 0,
        }
    }
}

//TODO 分成四个变量是否可以提升速度。减少同一变量写锁的时间
// 或者就保留目前的全局状态值。方便处理。
#[derive(Debug)]
pub struct State {
    pub report_hashrate: HashMap<String, String>,
    pub workers: HashMap<String,Worker>,
    pub proxy_jobs: HashSet<String>,
    pub proxy_share: u64,
    pub mine_jobs: HashMap<String,u64>,
    pub mine_jobs_queue: VecDeque<(u64,String)>,
    pub mine_share: u64,
    pub develop_jobs: HashSet<String>,
    pub develop_jobs_queue: VecDeque<String>,
    pub develop_share: u64,
}

impl State {
    pub fn new() -> Self {
        Self {
            report_hashrate: HashMap::new(),
            proxy_jobs: HashSet::new(),
            mine_jobs: HashMap::new(),
            develop_jobs: HashSet::new(),
            proxy_share: 0,
            mine_share: 0,
            develop_share: 0,
            mine_jobs_queue: VecDeque::new(),
            develop_jobs_queue: VecDeque::new(),
            workers: HashMap::new(),
        }
    }
}
