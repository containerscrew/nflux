use containerd_client::{
    connect,
    services::v1::{
        containers_client::ContainersClient, version_client::VersionClient, ListContainersRequest,
    },
    tonic::transport::Channel,
};
use podman_api::{opts::ContainerListOpts, Podman};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ContainerData {
    pub id: String,
    pub cgroup_path: String,
    pub name: String,
}

#[async_trait::async_trait]
pub trait ContainerRuntime {
    async fn list_containers(&self) -> Result<Vec<ContainerData>, anyhow::Error>;
}
pub struct PodmanRuntime {
    client: Podman,
}

impl PodmanRuntime {
    pub fn new(podman_socket_path: &str) -> Self {
        let client = Podman::unix(podman_socket_path);
        Self { client }
    }
}

#[async_trait::async_trait]
impl ContainerRuntime for PodmanRuntime {
    async fn list_containers(&self) -> Result<Vec<ContainerData>, anyhow::Error> {
        let opts = ContainerListOpts::builder().all(true).build();
        let containers = self.client.containers().list(&opts).await?;

        let mut available_containers = Vec::new();
        for data in containers {
            if let Some(id) = data.id {
                let container = self.client.containers().get(&id);
                let inspect = container.inspect().await?;
                if let Some(state) = inspect.state {
                    if let Some(cgroup_path) = state.cgroup_path {
                        let container_data = ContainerData {
                            id,
                            cgroup_path: format!("/sys/fs/cgroup{}", cgroup_path),
                            name: data.names.unwrap_or_default().join(","),
                        };
                        available_containers.push(container_data);
                    }
                }
            }
        }
        Ok(available_containers)
    }
}

pub struct ContainerdRuntime {
    client: ContainersClient<Channel>,
}
impl ContainerdRuntime {
    pub async fn new(containerd_socket_path: &str) -> Self {
        let channel = connect(containerd_socket_path).await.unwrap();
        let client = ContainersClient::<Channel>::new(channel);
        Self { client }
    }
}

#[async_trait::async_trait]
impl ContainerRuntime for ContainerdRuntime {
    async fn list_containers(&self) -> Result<Vec<ContainerData>, anyhow::Error> {
        let request = ListContainersRequest {
            ..Default::default()
        };

        let response = self.client.clone().list(request).await?;
        let containers = response.into_inner().containers;

        let mut result = Vec::new();
        for c in containers {
            let id = c.id;
            let name = "unknown".to_string();
            let cgroup_path = format!("/sys/fs/cgroup/{}", id);

            result.push(ContainerData {
                id,
                name,
                cgroup_path,
            });
        }

        Ok(result)
    }
}
