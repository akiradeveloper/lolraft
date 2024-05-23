use anyhow::Result;
use lol_tests::*;
use lolraft::client::TimeoutNow;
use rand::Rng;
use serial_test::serial;
use std::sync::Arc;

#[serial]
#[tokio::test(flavor = "multi_thread")]
async fn N3_L100_K3_multi_raft_cluster() -> Result<()> {
    let cluster = Arc::new(Cluster::new(3, 100).await?);

    let mut futs = vec![];
    for lane_id in 0..100 {
        let cluster = cluster.clone();
        let fut = async move {
            cluster.add_server(lane_id, 0, 0).await?;
            cluster.add_server(lane_id, 0, 1).await?;
            cluster.add_server(lane_id, 0, 2).await?;

            // Evenly distribute the leaders.
            let leader = (lane_id % 3) as u8;
            cluster
                .admin(leader)
                .send_timeout_now(TimeoutNow { lane_id })
                .await?;

            Ok::<(), anyhow::Error>(())
        };
        futs.push(fut);
    }
    futures::future::try_join_all(futs).await?;
    
    Ok(())
}

const L: u32 = 1000;
const REP: u32 = 300;

#[serial]
#[tokio::test(flavor = "multi_thread")]
async fn N3_L20_K3_multi_raft_io() -> Result<()> {
    dbg!("start");
    let cluster = Arc::new(Cluster::new(3, L).await?);

    let mut futs = vec![];
    for lane_id in 0..L {
        let cluster = cluster.clone();
        let fut = async move {
            cluster.add_server(lane_id, 0, 0).await?;
            cluster.add_server(lane_id, 0, 1).await?;
            cluster.add_server(lane_id, 0, 2).await?;

            // Evenly distribute the leaders.
            let leader = (lane_id % 3) as u8;
            dbg!(leader);
            cluster
                .admin(0)
                .send_timeout_now(TimeoutNow { lane_id })
                .await?;

            Ok::<(), anyhow::Error>(())
        };
        futs.push(fut);
    }
    futures::future::try_join_all(futs).await?;

    let mut cur_state = [0; 1000];
    for i in 0..REP {
        let lane_id = rand::thread_rng().gen_range(0..L);
        let add_v = rand::thread_rng().gen_range(1..=9);
        dbg!(i, lane_id, add_v);
        let leader = (lane_id % 3) as u8;
        let old_v = cluster.user(0).fetch_add(lane_id, add_v).await?;
        assert_eq!(old_v, cur_state[lane_id as usize]);
        cur_state[lane_id as usize] += add_v;
    }

    dbg!("done.");
    Ok(())
}
