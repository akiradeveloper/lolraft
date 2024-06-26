use anyhow::{ensure, Result};
use bollard::*;
use log::*;
use std::collections::HashMap;
use std::sync::Arc;
use tonic::transport::{Channel, Endpoint, Uri};

const NETWORK_NAME: &str = "lolraft_raft-network";

pub fn id_from_address(address: &str) -> u8 {
    let id = address
        .strip_prefix("http://lol-testapp-")
        .unwrap()
        .strip_suffix(":50000")
        .unwrap();
    id.parse().unwrap()
}

pub fn address_from_id(id: u8) -> String {
    format!("http://lol-testapp-{id}:50000")
}

#[derive(Clone)]
struct Container(String);

pub struct Env {
    docker: Arc<Docker>,
    containers: HashMap<u8, Container>,
    conn_cache: spin::Mutex<HashMap<u8, Channel>>,
}
impl Env {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;
        Ok(Self {
            docker: docker.into(),
            containers: HashMap::new(),
            conn_cache: spin::Mutex::new(HashMap::new()),
        })
    }

    pub async fn create(&mut self, id: u8, n_lanes: u32) -> Result<()> {
        ensure!(n_lanes > 0);
        ensure!(!self.containers.contains_key(&id));
        let options = container::CreateContainerOptions {
            name: format!("lol-testapp-{}", id),
            ..Default::default()
        };
        let address = address_from_id(id);
        let config = container::Config {
            image: Some("lol-testapp:latest".to_string()),
            env: Some(vec![
                format!("address={address}"),
                format!("n_lanes={n_lanes}"),
                "RUST_LOG=info".to_string(),
            ]),
            ..Default::default()
        };
        let resp = self.docker.create_container(Some(options), config).await?;
        let container_id = resp.id;
        self.containers.insert(id, Container(container_id));
        Ok(())
    }

    pub async fn start(&mut self, id: u8) -> Result<()> {
        ensure!(self.containers.contains_key(&id));
        let container_id = &self.containers.get(&id).unwrap().0.clone();
        self.docker
            .start_container::<&str>(&container_id, None)
            .await?;
        Ok(())
    }

    pub async fn stop(&mut self, id: u8) -> Result<()> {
        ensure!(self.containers.contains_key(&id));
        let container_id = self.containers.get(&id).unwrap().0.clone();
        self.docker.stop_container(&container_id, None).await?;
        Ok(())
    }

    pub async fn connect_network(&mut self, id: u8) -> Result<()> {
        ensure!(self.containers.contains_key(&id));
        let container_id = self.containers.get(&id).unwrap().0.clone();
        let config = network::ConnectNetworkOptions {
            container: container_id,
            ..Default::default()
        };
        self.docker.connect_network(NETWORK_NAME, config).await?;

        let config = network::InspectNetworkOptions {
            verbose: true,
            ..Default::default()
        };
        dbg!(
            self.docker
                .inspect_network::<&str>(NETWORK_NAME, Some(config))
                .await?
        );

        Ok(())
    }

    /// Wait for connection to be ready at most 5 seconds.
    pub async fn check_connectivity(&self, id: u8) -> Result<()> {
        for _ in 0..50 {
            let uri: Uri = address_from_id(id).parse().unwrap();
            let endpoint = Endpoint::from(uri)
                .connect_timeout(std::time::Duration::from_secs(1));
            match endpoint.connect().await {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        }
        Ok(())
    }

    pub async fn connect_ping_client(&self, id: u8) -> Result<testapp::PingClient<Channel>> {
        let uri: Uri = address_from_id(id).parse().unwrap();
        let endpoint = Endpoint::from(uri)
            .timeout(std::time::Duration::from_secs(1))
            .connect_timeout(std::time::Duration::from_secs(1));
        let chan = endpoint.connect().await?;
        let cli = testapp::PingClient::new(chan);
        Ok(cli)
    }

    pub fn get_connection(&self, id: u8) -> Channel {
        self.conn_cache
            .lock()
            .entry(id)
            .or_insert_with(|| {
                let uri: Uri = address_from_id(id).parse().unwrap();
                let endpoint = Endpoint::from(uri)
                    .http2_keep_alive_interval(std::time::Duration::from_secs(1))
                    .keep_alive_while_idle(true)
                    .timeout(std::time::Duration::from_secs(5))
                    .connect_timeout(std::time::Duration::from_secs(5));
                let chan = endpoint.connect_lazy();
                chan
            })
            .clone()
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        for (id, container) in self.containers.drain() {
            let docker = self.docker.clone();
            let fut = async move {
                let resp = docker
                    .remove_container(
                        &container.0,
                        Some(container::RemoveContainerOptions {
                            force: true,
                            v: true,
                            ..Default::default()
                        }),
                    )
                    .await;
                match resp {
                    Ok(_) => info!("removed container id={id}"),
                    Err(e) => error!("failed to remove container id={id} (err={e})"),
                }
            };
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(fut);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn id_address() {
        for id in 0..=255 {
            let address = address_from_id(id);
            assert_eq!(id, id_from_address(&address));
        }
    }
}
