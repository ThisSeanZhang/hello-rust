use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::{Duration, Instant};
use futures::executor::ThreadPool;
use futures::{select, FutureExt, StreamExt};
use futures::task::SpawnExt;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[cfg(test)]
pub mod config;
pub mod errors;
pub mod persister;
#[cfg(test)]
mod tests;
mod events;

use self::errors::*;
use self::persister::*;
use crate::proto::raftpb::*;

/// Raft peer heart beat interval in millis,
/// should not be too small to ensure passing tests.
const HEART_BEAT_MILLIS: u64 = 120;

/// As each Raft peer becomes aware that successive log entries are committed,
/// the peer should send an `ApplyMsg` to the service (or tester) on the same
/// server, via the `apply_ch` passed to `Raft::new`.
pub enum ApplyMsg {
    Command {
        data: Vec<u8>,
        index: u64,
    },
    // For 2D:
    Snapshot {
        data: Vec<u8>,
        term: u64,
        index: u64,
    },
}

/// State of a raft peer.
#[derive(Default, Clone, Debug)]
pub struct State {
    pub term: u64,
    pub is_leader: bool,
}

impl State {
    /// The current term of this peer.
    pub fn term(&self) -> u64 {
        self.term
    }
    /// Whether this peer believes it is the leader.
    pub fn is_leader(&self) -> bool {
        self.is_leader
    }

    pub fn per_election(&mut self) {
        self.term += 1;
        self.is_leader = false;
    }
}
/// Persistent state of a raft peer.
#[derive(Serialize, Deserialize, Debug)]
struct PersistentState {
    pub current_term: u64,
    pub voted_for: Option<usize>,
    pub log: RaftLog,
}

impl Default for PersistentState {
    fn default() -> Self {
        Self {
            current_term: 0,
            voted_for: None, 
            log: RaftLog::new()
        }
    }
}

// struct VolatileState {
//     pub commit_index: usize,
//     pub last_applied: usize,
// }

// pub enum Role {
//     Follower,
//     Leader,

// }

#[derive(Serialize, Deserialize, Debug)]
struct RaftLog {
    data: Vec<(u64, Vec<u8>)>,
}

impl RaftLog {

    fn new() -> RaftLog {
        RaftLog{
            data: vec![]
        }
    }
    fn get(&self, index: u64) -> Option<(u64, Vec<u8>)> {
        if index == 0 {
            return Some((0, vec![]));
        }
        if let Some((term, data)) = self.data.get(index as usize - 1) {
            Some((*term, data.clone()))
        } else {
            None
        }
    }
    fn get_append_info(&self, index: u64) -> Option<(u64, Option<Entry>)> {
        if index != 0 {
            match (self.get(index - 1), self.get(index)) {
                (Some((prev_log_term, _)), Some((term, data))) => {
                    Some((prev_log_term, Some(Entry{ term: term.clone(), data: data.clone()})))
                },
                (Some((prev_log_term, _) ), None) => {
                    Some((prev_log_term, None))
                },
                _ => {
                    error!("wrong index: {index}");
                    panic!()
                }
            }
        } else {
            Some((0, None))
        }
    }

    fn truncate(&mut self, size: u64) {
        self.data.truncate(size as usize);
    }

    fn get_last_info(&self) -> Option<(u64, u64)> {
        let index = self.len();
        if let Some((term, _)) = self.get(index) {
            Some((term, index))
        } else {
            None
        }
    }

    fn len(&self) -> u64 {
        self.data.len() as u64
    }

    fn next_index(&self) -> u64 {
        self.len() + 1
    }

    fn push(&mut self, item: (u64, Vec<u8>)) {
        self.data.push(item)
    }
}

// A single Raft peer.
pub struct Raft {
    // RPC end points of all peers
    peers: Vec<RaftClient>,
    // Object to hold this peer's persisted state
    persister: Box<dyn Persister>,
    // this peer's index into peers[]
    me: usize,
    state: State,
    // Your data here (2A, 2B, 2C).
    // Look at the paper's Figure 2 for a description of what
    // state a Raft server must maintain.
    // persist: PersistentState,
    
    // Persistent state on all servers
    // (Updated on stable storage before responding to RPCs)
    voted_for: Option<usize>, // candidateId that received vote in current term
    log: RaftLog, // log entries

    // Volatile state on all servers
    commit_index: u64, // index of highest log entry known to be committed (initialized to 0)
    last_applied: u64, // index of highest log entry applied to state machine (initialized to 0)

    // Volatile state on leaders
    // (Reinitialized after election)
    next_index: Vec<u64>, // index of the next log entry to send to that server (initialized to leader last log index + 1)
    match_index: Vec<u64>, // index of highest log entry known to be replicated on server (initialized to 0)

    // other state
    apply_ch: UnboundedSender<ApplyMsg>,

    current_leader: u64,

    append_channel: (SyncSender<(usize, RaftClient)>, Receiver<(usize, RaftClient)>),
}

impl Raft {
    /// the service or tester wants to create a Raft server. the ports
    /// of all the Raft servers (including this one) are in peers. this
    /// server's port is peers[me]. all the servers' peers arrays
    /// have the same order. persister is a place for this server to
    /// save its persistent state, and also initially holds the most
    /// recent saved state, if any. apply_ch is a channel on which the
    /// tester or service expects Raft to send ApplyMsg messages.
    /// This method must return quickly.
    pub fn new(
        peers: Vec<RaftClient>,
        me: usize,
        persister: Box<dyn Persister>,
        apply_ch: UnboundedSender<ApplyMsg>,
    ) -> Raft {
        // let (raft_event_sender, rx) =  mpsc::unbounded::<RaftEvent>();
        let raft_state = persister.raft_state();
        let append_channel = sync_channel::<(usize, RaftClient)>(peers.len());
        for (index, peer) in peers.iter().enumerate() {
            if index == me {continue;}
            append_channel.0.try_send((index, peer.clone())).unwrap();
        }
        let client_size = peers.len();
        // Your initialization code here (2A, 2B, 2C).
        let mut rf = Raft {
            peers,
            persister,
            me,
            state: State::default(),
            // persist: PersistentState::default(),
            voted_for: None,
            log: RaftLog::new(),
            commit_index: 0,
            last_applied: 1,
            next_index:  vec![0; client_size],
            match_index:  vec![0; client_size],
            apply_ch,
            current_leader: me as u64,
            append_channel,
        };

        // initialize from state persisted before a crash
        rf.restore(&raft_state);
        rf.last_applied = rf.log.next_index();
        
        rf
    }

    fn commit_command(&mut self, leader_commit: u64) {
        let no = self.me;
        let start_commit_index = self.commit_index + 1;
        if leader_commit < start_commit_index { return; }
        info!("No: {no}, start commit range is: ({} ~ {}]", self.commit_index, leader_commit);
        for index in start_commit_index..=leader_commit {
            if let Some((_, data)) = self.log.get(index) {
                info!("No: {no}, commit index: {index}, last_applied: {}", self.last_applied);
                self.apply_ch.unbounded_send(ApplyMsg::Command { data: data.clone(), index }).unwrap();
                self.commit_index = index;
            }
        }
    }

    /// save Raft's persistent state to stable storage,
    /// where it can later be retrieved after a crash and restart.
    /// see paper's Figure 2 for a description of what should be persistent.
    fn persist(&mut self) {
        // Your code here (2C).
        // Example:
        // labcodec::encode(&self.xxx, &mut data).unwrap();
        // labcodec::encode(&self.yyy, &mut data).unwrap();
        // self.persister.save_raft_state(data);
    }

    /// restore previously persisted state.
    fn restore(&mut self, data: &[u8]) {
        if data.is_empty() {
            // bootstrap without any state?
            return;
        }
        
        match serde_json::from_slice::<PersistentState>(data) {
            Ok(persist) => {
                self.state.term = persist.current_term;
                self.voted_for = persist.voted_for;
                self.log = persist.log;
                // self.commit_index = persist.log.len() as u64 - 1 ;
                // self.last_applied = persist.log.len();
                // self.persist = persist;
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
        // Your code here (2C).
        // Example:
        // match labcodec::decode(data) {
        //     Ok(o) => {
        //         self.xxx = o.xxx;
        //         self.yyy = o.yyy;
        //     }
        //     Err(e) => {
        //         panic!("{:?}", e);
        //     }
        // }
    }

    fn get_append_entries_args(&self, index:usize) -> Option<AppendEntriesArgs> {
        let next_index = self.next_index[index];
        let match_index = self.match_index[index];
        let need_data = next_index - match_index == 1;
        if let Some((prev_log_term, entry)) = self.log.get_append_info(next_index) {
            info!("using index: {next_index}, get prev_log_term: {prev_log_term}, entry: {entry:?}");
            Some(AppendEntriesArgs{
                term: self.state.term,
                leader_id:self.me as u64,
                prev_log_term,
                prev_log_index: next_index - 1,
                entries: if need_data && entry.is_some() { vec![entry.unwrap()] } else { vec![] },
                leader_commit:self.commit_index
            })
        } else {
            None
        }
    }

    fn send_request_append(&self, raft: Arc<Mutex<Raft>>) {
        let no = self.me;
        let append_timeout_build = || futures_timer::Delay::new(Duration::from_millis(HEART_BEAT_MILLIS / 2)).fuse();
        while let Ok((index, client)) = self.append_channel.1.try_recv() {
            warn!("No:{no}, create appen client index: {index}");
            let client_clone = client.clone();
            let rf_clone = raft.clone();
            let return_clinet = self.append_channel.0.clone();
            let mut append_timeout = append_timeout_build();
            client.spawn(async move {
                loop {
                    let args = if let Ok(rf) = rf_clone.try_lock() {
                        if !rf.state.is_leader() {
                            return_clinet.send((index, client_clone)).unwrap();
                            break;
                        }
                        rf.get_append_entries_args(index)
                    } else {
                        None
                    };
    
                    if let Some(args) = args {
                        let instant = Instant::now();
                        info!("No:{no}, send append request to: {index}");
                        let result = select! {
                            _ = append_timeout => {
                                None
                            },
                            result = client_clone.append_entries(&args).fuse() => {
                                Some(result.map_err(Error::Rpc))
                            }
                        };
                        // let result = client_clone.append_entries(&args).await.map_err(Error::Rpc);
                        if let Some(Ok(AppendEntriesReply{ term, success, match_index})) = result {
                            info!("No:{no} to :{index}, AppendEntries term: {term}, result: {success}, match_index: {match_index}");
                            if let Ok(mut rf) = rf_clone.lock() {
                                if success {
                                    rf.match_index[index] = match_index;
                                    rf.next_index[index] = rf.last_applied.min(match_index + 1);
                                    info!("No:{no}, update index :{index}, leader last_applied: {}, match_index: {match_index}, next_index: {}", rf.last_applied, rf.next_index[index]);
                                    let mut match_size = 1;
                                    let mut min_match_index = rf.last_applied;
                                    for &each in rf.match_index.iter() {
                                        if each > rf.commit_index {
                                            match_size += 1;
                                            min_match_index = min_match_index.min(each);
                                        }
                                    }
                                    if match_size * 2 > rf.peers.len() {
                                        rf.commit_command(min_match_index);
                                    }
                                    // same as leader
                                    if rf.next_index[index] == rf.last_applied {
                                        return_clinet.send((index, client_clone)).unwrap();
                                        break;
                                    }
                                } else if term > args.term{
                                    rf.state.is_leader = false;
                                    rf.state.term = term;
                                    return_clinet.send((index, client_clone)).unwrap();
                                    break;
                                } else {
                                    info!("No:{no}, Argument: {args:?}");
                                    rf.next_index[index] -= 1;
                                }
                            }
                        } else {
                            error!("No:{no} to :{index}, fail, elapsed: {:?}", instant.elapsed());
                            return_clinet.send((index, client_clone)).unwrap();
                            break;
                        }
                    }
                }
            })
        }
    }
    /// example code to send a RequestVote RPC to a server.
    /// server is the index of the target server in peers.
    /// expects RPC arguments in args.
    ///
    /// The labrpc package simulates a lossy network, in which servers
    /// may be unreachable, and in which requests and replies may be lost.
    /// This method sends a request and waits for a reply. If a reply arrives
    /// within a timeout interval, This method returns Ok(_); otherwise
    /// this method returns Err(_). Thus this method may not return for a while.
    /// An Err(_) return can be caused by a dead server, a live server that
    /// can't be reached, a lost request, or a lost reply.
    ///
    /// This method is guaranteed to return (perhaps after a delay) *except* if
    /// the handler function on the server side does not return.  Thus there
    /// is no need to implement your own timeouts around this method.
    ///
    /// look at the comments in ../labrpc/src/lib.rs for more details.
    fn get_vote_request_arg(&self) -> RequestVoteArgs{
        let (term, index) = self.log.get_last_info().unwrap();
        RequestVoteArgs {
            term: self.state.term,
            candidate_id: self.me as u64,
            last_log_index: index,
            last_log_term: term,
        }
    }
    fn send_request_vote_v2(&mut self, raft: Arc<Mutex<Raft>>) {
        let no = self.me;
    //     info!("No: {no}, My current term is {}", self.state.term);
        self.state.per_election();
    //     info!("No: {no}, after per_election term is {}", self.state.term);
        self.voted_for = Some(no);
        let args = Arc::new(self.get_vote_request_arg());
        info!("No:{no}, start election in term: {}, args: {args:?}", self.state.term);
        let vote = Arc::new(AtomicU64::new(1));
        let vote_term = Arc::new(self.state.term);
        let client_num = Arc::new(self.peers.len() as u64);
        let vote_timeout_build = || futures_timer::Delay::new(Duration::from_millis(HEART_BEAT_MILLIS / 2)).fuse();
        for (index, peer) in self.peers.iter().enumerate() {
            if index == self.me { continue;}
            let peer_clone = peer.clone();
            let arg_clone = args.clone();
            let vote_num_clone = vote.clone();
            let vote_term_clone = vote_term.clone();
            let client_num_clone = client_num.clone();
            let rf = raft.clone();
            let mut vote_timeout = vote_timeout_build();
            peer.spawn(async move {
                info!("No:{no}, send vote request to: {index}");
                let res = select!{
                    _ = vote_timeout => {
                        None
                    },
                    result = peer_clone.request_vote(&arg_clone).fuse() => {
                        Some(result.map_err(Error::Rpc))
                    }
                };
                // let res = peer_clone.request_vote(&arg_clone).await.map_err(Error::Rpc);
                if let Some(Ok(RequestVoteReply{term, vote_granted})) = res {        
                    if vote_granted && *vote_term_clone == term {
                    info!("No:{no}, get 1 vote from: {index} in term: {term}");
                        vote_num_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        if vote_num_clone.load(std::sync::atomic::Ordering::Relaxed) * 2 > *client_num_clone {
                            if let Ok(mut raft) = rf.try_lock() {
                                info!("No:{no}, i been the leader in {}", raft.state.term);
                                raft.state.is_leader = true;
                                raft.last_applied = raft.log.next_index();
                                raft.match_index = vec![0; raft.peers.len()];
                                raft.next_index = vec![raft.log.next_index(); raft.peers.len()];
                                raft.send_request_append(rf.clone());
                            }
                        }
                    } else {
                        warn!("No:{no}, can not get vote granted from index: {index}, because term: {term}");
                    }
                } else {
                    warn!("No:{no}, can not get vote granted from {index}");
                }
            });
        }
    }

    // fn send_request_vote(&mut self) {
    //     // Your code here if you want the rpc becomes async.
    //     // Example:
    //     // ```
    //     // let peer = &self.peers[server];
    //     // let peer_clone = peer.clone();
    //     // let (tx, rx) = channel();
    //     // peer.spawn(async move {
    //     //     let res = peer_clone.request_vote(&args).await.map_err(Error::Rpc);
    //     //     tx.send(res);
    //     // });
    //     // rx
    //     // ```
    //     let no = self.me;
    //     info!("No: {no}, My current term is {}", self.state.term);
    //     self.state.per_election();
    //     info!("No: {no}, after per_election term is {}", self.state.term);
    //     self.voted_for = Some(no);
    //     let (term, index) = self.log.get_last_info().unwrap();
    //     let args = Arc::new(RequestVoteArgs {
    //         term: self.state.term,
    //         candidate_id: self.me as u64,
    //         last_log_index: term,
    //         last_log_term: index,
    //     });
    //     let (tx, rx) = sync_channel::<Result<RequestVoteReply>>(self.peers.len() - 1);
    //     for (index, peer) in self.peers.iter().enumerate() {
    //         if index == self.me { continue;}
    //         let peer_clone = peer.clone();
    //         let tx_clone = tx.clone();
    //         let arg_clone = args.clone();
    //         peer.spawn(async move {
    //             let res = peer_clone.request_vote(&arg_clone).await.map_err(Error::Rpc);
    //             match tx_clone.send(res) {
    //                 // Ok(_) => info!("No:{no}, request_vote success"),
    //                 Ok(_) => (),
    //                 Err(e) => { error!("{e}")}
    //             }
    //         });
    //     }
    //     let mut vote_num = 1;
    //     for _ in 1..self.peers.len() {
    //         match rx.recv() {
    //             Ok(Ok(RequestVoteReply{ term, vote_granted})) => {
    //                 info!("No:{no}, replay recv success term: {term}, vote_granted: {vote_granted}");
    //                 if term > self.state.term {
    //                     self.state.term = term;
    //                     break;
    //                 }
    
    //                 if term == self.state.term && vote_granted {
    //                     vote_num += 1;
    //                 }
    //             },
    //             Err(e) => {
    //                 error!("No:{no}, replay recv err {e}");
    //             },
    //             Ok(Err(e)) => {
    //                 error!("No:{no}, replay vore request err {e}");
    //             }
    //         }
    //     }
    //     info!("No:{no}, i got {} vote", vote_num);
    //     if vote_num * 2 > self.peers.len() {
    //         info!("No:{no}, i been the leader in {}", self.state.term);
    //         self.state.is_leader = true;
    //         self.match_index = vec![0; self.peers.len()];
    //         self.next_index = vec![self.log.next_index(); self.peers.len()];
    //     }
    // }

    fn start<M>(&mut self, command: &M) -> Result<(u64, u64)>
    where
        M: labcodec::Message,
    {
        let mut buf = vec![];
        labcodec::encode(command, &mut buf).map_err(Error::Encode)?;
        // Your code here (2B).
        let term = self.state.term;
        self.log.push((term, buf));
        self.last_applied += 1;

        Ok((self.log.len(), term))
    }

    fn cond_install_snapshot(
        &mut self,
        last_included_term: u64,
        last_included_index: u64,
        snapshot: &[u8],
    ) -> bool {
        // Your code here (2D).
        crate::your_code_here((last_included_term, last_included_index, snapshot));
    }

    fn snapshot(&mut self, index: u64, snapshot: &[u8]) {
        // Your code here (2D).
        crate::your_code_here((index, snapshot));
    }
}

impl Raft {
    /// Only for suppressing deadcode warnings.
    #[doc(hidden)]
    pub fn __suppress_deadcode(&mut self) {
        let _ = self.start(&0);
        let _ = self.cond_install_snapshot(0, 0, &[]);
        self.snapshot(0, &[]);
        self.persist();
        let _ = &self.state;
        let _ = &self.me;
        let _ = &self.persister;
        let _ = &self.peers;
    }
}

// Choose concurrency paradigm.
//
// You can either drive the raft state machine by the rpc framework,
//
// ```rust
// struct Node { raft: Arc<Mutex<Raft>> }
// ```
//
// or spawn a new thread runs the raft state machine and communicate via
// a channel.
//
// ```rust
// struct Node { sender: Sender<Msg> }
// ```
#[derive(Clone)]
pub struct Node {
    raft: Arc<Mutex<Raft>>,
    // raft_event_sender:  UnboundedSender<RaftEvent>,
    pool: ThreadPool,
    // no: usize,
    // sender: Sender<Event>,
    timer_beat: UnboundedSender<()>,
    stop: Arc<Mutex<mpsc::Sender<()>>>
}

impl Node {
    /// Create a new raft service.
    pub fn new(raft: Raft) -> Node {
        info!("No: {}, create new Node", raft.me);
        let (tx, rx) =  mpsc::unbounded::<()>();
        // let (raft_event_sender, rx2) =  mpsc::unbounded::<RaftEvent>();
        let (stop_tx, stop_rx) =  mpsc::channel::<()>(1);

        let pool = ThreadPool::builder().pool_size(1).create().unwrap();
        let node = Node {
            // no: raft.me,
            raft: Arc::new(Mutex::new(raft)),
            // raft_event_sender,
            pool,
            timer_beat: tx,
            stop: Arc::new(Mutex::new(stop_tx)),
        };
        node.timer(rx, stop_rx);
        node
    }

    pub fn timer(&self,mut rece:UnboundedReceiver<()>,mut stop: mpsc::Receiver<()>) {
        // let no = self.no;
        let election_timeout_build = move || {
            // info!("No: {no}, Reset Election Timeout");
            futures_timer::Delay::new(Duration::from_millis(
                rand::thread_rng().gen_range(HEART_BEAT_MILLIS * 3, HEART_BEAT_MILLIS * 5),
            )).fuse()
        };

        let heart_beat_timeout_build = || futures_timer::Delay::new(Duration::from_millis(HEART_BEAT_MILLIS)).fuse();

        let rf = self.raft.clone();
        self.pool.spawn(async move { 
            let mut next_election_timeout = election_timeout_build();
            let mut heart_beat = heart_beat_timeout_build();

            loop {
                select! {
                    _ = stop.select_next_some() => {
                        break;
                    },
                    () = rece.select_next_some() => {
                        next_election_timeout = election_timeout_build();
                    },
                    _ = next_election_timeout => {
                        if let Ok(mut raft) = rf.try_lock() {
                            if (!raft.state.is_leader()) {
                                raft.send_request_vote_v2(rf.clone());
                                heart_beat = heart_beat_timeout_build();
                            }
                        }
                        next_election_timeout = election_timeout_build();
                    },
                    _ = heart_beat => {
                        if let Ok(mut raft) = rf.try_lock() {
                            if (raft.state.is_leader()) {
                                raft.send_request_append(rf.clone());
                            }
                        }
                        heart_beat = heart_beat_timeout_build();
                    }
                }
            }

        }).unwrap();
    }

    /// the service using Raft (e.g. a k/v server) wants to start
    /// agreement on the next command to be appended to Raft's log. if this
    /// server isn't the leader, returns [`Error::NotLeader`]. otherwise start
    /// the agreement and return immediately. there is no guarantee that this
    /// command will ever be committed to the Raft log, since the leader
    /// may fail or lose an election. even if the Raft instance has been killed,
    /// this function should return gracefully.
    ///
    /// the first value of the tuple is the index that the command will appear
    /// at if it's ever committed. the second is the current term.
    ///
    /// This method must return without blocking on the raft.
    pub fn start<M>(&self, command: &M) -> Result<(u64, u64)>
    where
        M: labcodec::Message,
    {
        if let Ok(mut raft) = self.raft.lock() {
            if raft.state.is_leader() {
                raft.start(command)
            } else { Err(Error::NotLeader) }
        } else { Err(Error::NotLeader) }
    }

    /// The current term of this peer.
    pub fn term(&self) -> u64 {
        // Your code here.
        // Example:
        // self.raft.term
        self.raft.lock().unwrap().state.term()
    }

    /// Whether this peer believes it is the leader.
    pub fn is_leader(&self) -> bool {
        // Your code here.
        // Example:
        // self.raft.leader_id == self.id
        self.raft.lock().unwrap().state.is_leader()
    }

    /// The current state of this peer.
    pub fn get_state(&self) -> State {
        State {
            term: self.term(),
            is_leader: self.is_leader(),
        }
    }

    /// the tester calls kill() when a Raft instance won't be
    /// needed again. you are not required to do anything in
    /// kill(), but it might be convenient to (for example)
    /// turn off debug output from this instance.
    /// In Raft paper, a server crash is a PHYSICAL crash,
    /// A.K.A all resources are reset. But we are simulating
    /// a VIRTUAL crash in tester, so take care of background
    /// threads you generated with this Raft Node.
    pub fn kill(&self) {
        if let Ok(mut tx) = self.stop.lock() {
            tx.try_send(()).unwrap();
        }
    }

    /// A service wants to switch to snapshot.  
    ///
    /// Only do so if Raft hasn't have more recent info since it communicate
    /// the snapshot on `apply_ch`.
    pub fn cond_install_snapshot(
        &self,
        last_included_term: u64,
        last_included_index: u64,
        snapshot: &[u8],
    ) -> bool {
        // Your code here.
        // Example:
        // self.raft.cond_install_snapshot(last_included_term, last_included_index, snapshot)
        crate::your_code_here((last_included_term, last_included_index, snapshot));
    }

    /// The service says it has created a snapshot that has all info up to and
    /// including index. This means the service no longer needs the log through
    /// (and including) that index. Raft should now trim its log as much as
    /// possible.
    pub fn snapshot(&self, index: u64, snapshot: &[u8]) {
        // Your code here.
        // Example:
        // self.raft.snapshot(index, snapshot)
        crate::your_code_here((index, snapshot));
    }
}

#[async_trait::async_trait]
impl RaftService for Node {
    // example RequestVote RPC handler.
    //
    // CAVEATS: Please avoid locking or sleeping here, it may jam the network.
    async fn request_vote(&self, args: RequestVoteArgs) -> labrpc::Result<RequestVoteReply> {
        if let Ok(mut raft) = self.raft.try_lock() {
            let no = raft.me;
            let RequestVoteArgs{
                term,
                candidate_id,
                last_log_index,
                last_log_term,
            } = args;

            // if term is smaller then current server return false
            if raft.state.term > term {
                error!("No: {no}, cause current term larger then the candidate: {candidate_id}");
                return Ok(RequestVoteReply{
                    term: raft.state.term,
                    vote_granted: false,
                });
            }
            // The current node has voted
            if raft.state.term == term && raft.voted_for.is_some() {
                error!("No: {no}, cause current servie is vote to: {:?}", raft.voted_for);
                return Ok(RequestVoteReply{
                    term: raft.state.term,
                    vote_granted: false,
                });
            }
            
            raft.state.term = term;
            raft.state.is_leader = false;

            // if candidate log id is not up-to-date as current server log
            let current_log_len = raft.log.len();
            if let Some((current_last_term, _)) = raft.log.get(current_log_len) {
                info!("No: {no}, current_log_len: {current_log_len}, current_last_term: {current_last_term}");
                if last_log_term > current_last_term || (last_log_term == current_last_term && last_log_index >= current_log_len) {
                    // reset election timeout
                    self.timer_beat.unbounded_send(()).unwrap();
                    raft.voted_for = Some(candidate_id as usize);
                    return Ok(RequestVoteReply{
                        term: raft.state.term,
                        vote_granted: true,
                    });
                }
            }
            
            Ok(RequestVoteReply{
                term: raft.state.term,
                vote_granted: false,
            })
        } else {
            Err(labrpc::Error::Unimplemented("ERROR".into()))
        }
    }

    async fn append_entries(&self, args: AppendEntriesArgs) -> labrpc::Result<AppendEntriesReply> {
        if let Ok(mut raft) = self.raft.try_lock() {
            info!("No:{}, receive AppendEntries {args:?}", raft.me);
            let mut match_index = 0;
            let AppendEntriesArgs{
                term,
                leader_id,
                prev_log_term,
                prev_log_index,
                entries,
                leader_commit
            } = args;
            
            // 1. ckeck the data term
            if raft.state.term > term {
                return Ok(AppendEntriesReply{
                    term: raft.state.term,
                    match_index,
                    success: false
                })
            }

            // reset election timeout
            if let Err(e) = self.timer_beat.unbounded_send(()) {
                error!("No: {}, can not reset election timeout with error {e}", raft.me);
            }

            // to be follower
            raft.state.term = term;
            raft.state.is_leader = false;
            raft.current_leader = leader_id;
            
            // 2. check prev_log_index is contain in log and term match
            if let Some((log_term, _)) = raft.log.get(prev_log_index) {
                if log_term != prev_log_term {
                    return Ok(AppendEntriesReply{
                        term: raft.state.term,
                        match_index,
                        success: false
                    })
                } else {
                    // error!("{:?}", raft.log);
                    raft.log.truncate(prev_log_index);
                    // error!("{:?}", raft.log);
                }
            } else {
                return Ok(AppendEntriesReply{
                    term: raft.state.term,
                    match_index,
                    success: false
                })
            }
            
            for Entry { term, data } in entries {
                raft.log.push((term, data));
            }
            match_index = raft.log.len();
            raft.last_applied = raft.log.next_index();
            // commit log
            raft.commit_command(leader_commit);
            
            return Ok(AppendEntriesReply{
                term: raft.state.term,
                match_index,
                success: true
            })
        } else {
            Err(labrpc::Error::Unimplemented("ERROR".into()))
        }
    }
}
