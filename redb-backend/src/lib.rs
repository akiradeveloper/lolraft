use anyhow::Result;
use async_trait::async_trait;
use lolraft::process::*;
use redb::{Database, ReadableTable, TableDefinition};
use std::sync::Arc;

mod entry {
    use super::*;

    #[derive(serde::Deserialize, serde::Serialize)]
    struct OnDiskStruct {
        prev_term: u64,
        cur_index: u64,
        cur_term: u64,
        command: bytes::Bytes,
    }

    pub fn ser(x: Entry) -> Vec<u8> {
        let x = OnDiskStruct {
            prev_term: x.prev_clock.term,
            cur_index: x.this_clock.index,
            cur_term: x.this_clock.term,
            command: x.command,
        };
        let bin = bincode::serialize(&x).unwrap();
        bin
    }

    pub fn desr(bin: &[u8]) -> Entry {
        let x: OnDiskStruct = bincode::deserialize(bin).unwrap();
        Entry {
            prev_clock: Clock {
                index: x.cur_index - 1,
                term: x.prev_term,
            },
            this_clock: Clock {
                index: x.cur_index,
                term: x.prev_term,
            },
            command: x.command,
        }
    }
}

struct LazyInsert {
    index: Index,
    inner: Entry,
    space: String,
    notifier: oneshot::Sender<()>,
}

struct Reaper {
    db: Arc<redb::Database>,
    recv: flume::Receiver<LazyInsert>,
}
impl Reaper {
    fn table_def(space: &str) -> redb::TableDefinition<u64, Vec<u8>> {
        redb::TableDefinition::new(space)
    }

    fn reap(&self) -> Result<()> {
        // wait for a entry
        let head = self.recv.recv()?;
        let tail = self.recv.drain();

        let mut notifiers = vec![];
        let tx = self.db.begin_write()?;

        // insert the first entry
        {
            let mut tbl = tx.open_table(Self::table_def(&head.space))?;
            tbl.insert(head.index, entry::ser(head.inner))?;
            notifiers.push(head.notifier);
        }

        for e in tail {
            let mut tbl = tx.open_table(Self::table_def(&e.space))?;
            tbl.insert(e.index, entry::ser(e.inner))?;
            notifiers.push(e.notifier);
        }
        tx.commit()?;

        for notifier in notifiers {
            notifier.send(()).ok();
        }
        Ok(())
    }
}

struct LogStore {
    db: Arc<Database>,
    space: String,
    reaper_queue: flume::Sender<LazyInsert>,
}
impl LogStore {
    fn table_def(&self) -> TableDefinition<u64, Vec<u8>> {
        TableDefinition::new(&self.space)
    }

    async fn enqueue_entry(&self, i: Index, e: Entry) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        let e = LazyInsert {
            index: i,
            inner: e,
            space: self.space.clone(),
            notifier: tx,
        };
        self.reaper_queue.send(e).ok();
        rx.await?;
        Ok(())
    }
}
#[async_trait]
impl RaftLogStore for LogStore {
    async fn insert_entry(&self, i: Index, e: Entry) -> Result<()> {
        self.enqueue_entry(i, e).await?;
        Ok(())
    }

    async fn delete_entries_before(&self, i: Index) -> Result<()> {
        let tx = self.db.begin_write()?;
        {
            let mut tbl = tx.open_table(self.table_def())?;
            tbl.retain(|k, _| k >= i)?;
        }
        tx.commit()?;
        Ok(())
    }
    async fn get_entry(&self, i: Index) -> Result<Option<Entry>> {
        let tx = self.db.begin_read()?;
        let tbl = tx.open_table(self.table_def())?;
        match tbl.get(i)? {
            Some(bin) => Ok(Some(entry::desr(&bin.value()))),
            None => Ok(None),
        }
    }
    async fn get_head_index(&self) -> Result<Index> {
        let tx = self.db.begin_read()?;
        let tbl = tx.open_table(self.table_def())?;
        let out = tbl.first()?;
        Ok(match out {
            Some((k, _)) => k.value(),
            None => 0,
        })
    }
    async fn get_last_index(&self) -> Result<Index> {
        let tx = self.db.begin_read()?;
        let tbl = tx.open_table(self.table_def())?;
        let out = tbl.last()?;
        Ok(match out {
            Some((k, _)) => k.value(),
            None => 0,
        })
    }
}

mod ballot {
    use super::*;

    #[derive(serde::Deserialize, serde::Serialize)]
    struct OnDiskStruct {
        term: u64,
        voted_for: Option<lolraft::NodeId>,
    }

    pub fn ser(x: Ballot) -> Vec<u8> {
        let x = OnDiskStruct {
            term: x.cur_term,
            voted_for: x.voted_for,
        };
        let bin = bincode::serialize(&x).unwrap();
        bin
    }

    pub fn desr(bin: &[u8]) -> Ballot {
        let x: OnDiskStruct = bincode::deserialize(bin).unwrap();
        Ballot {
            cur_term: x.term,
            voted_for: x.voted_for,
        }
    }
}

struct BallotStore {
    db: Arc<Database>,
    space: String,
}
impl BallotStore {
    fn table_def(&self) -> TableDefinition<(), Vec<u8>> {
        TableDefinition::new(&self.space)
    }
}
#[async_trait]
impl RaftBallotStore for BallotStore {
    async fn save_ballot(&self, ballot: Ballot) -> Result<()> {
        let tx = self.db.begin_write()?;
        {
            let mut tbl = tx.open_table(self.table_def())?;
            tbl.insert((), ballot::ser(ballot))?;
        }
        tx.commit()?;
        Ok(())
    }
    async fn load_ballot(&self) -> Result<Ballot> {
        let tx = self.db.begin_read()?;
        let tbl = tx.open_table(self.table_def())?;
        match tbl.get(())? {
            Some(bin) => Ok(ballot::desr(&bin.value())),
            None => Err(anyhow::anyhow!("No ballot")),
        }
    }
}

pub struct Backend {
    db: Arc<redb::Database>,
    tx: flume::Sender<LazyInsert>,
}
impl Backend {
    pub fn new(redb: redb::Database) -> Self {
        let db = Arc::new(redb);

        let (tx, rx) = flume::unbounded();
        let reaper = Reaper {
            db: db.clone(),
            recv: rx,
        };
        std::thread::spawn(move || loop {
            reaper.reap().ok();
        });

        Self { db, tx }
    }

    pub fn get(&self, lane_id: u32) -> (impl RaftLogStore, impl RaftBallotStore) {
        let log = LogStore {
            space: format!("log-{lane_id}"),
            db: self.db.clone(),
            reaper_queue: self.tx.clone(),
        };
        let ballot = BallotStore {
            space: format!("ballot-{lane_id}"),
            db: self.db.clone(),
        };
        (log, ballot)
    }
}