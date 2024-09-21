use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use uuid::Uuid;
use tokio::time::{Duration, Instant};
use tokio::net::TcpListener;

/// Represents a node in the cluster, which can either be a leader or a follower.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,             // Unique ID for the node
    pub is_leader: bool,      // Indicates whether the node is the leader
    pub address: String,      // The address (host:port) of the node
    pub last_heartbeat: Instant, // Last heartbeat received from the node (for health checks)
}

/// Cluster manager responsible for managing nodes and communication in the cluster.
#[derive(Debug, Clone)]
pub struct Cluster {
    nodes: Arc<RwLock<HashMap<Uuid, Node>>>,  // All nodes in the cluster
    self_node: Node,                          // The node running this instance
    leader: Arc<Mutex<Option<Uuid>>>,         // Current leader's ID (if elected)
}

impl Cluster {
    /// Creates a new node and adds it to the cluster.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the current node (host:port).
    ///
    pub async fn new(address: String) -> Cluster {
        let node_id = Uuid::new_v4();
        let self_node = Node {
            id: node_id,
            is_leader: false,
            address,
            last_heartbeat: Instant::now(),
        };

        let mut nodes = HashMap::new();
        nodes.insert(node_id, self_node.clone());

        Cluster {
            nodes: Arc::new(RwLock::new(nodes)),
            self_node,
            leader: Arc::new(Mutex::new(None)), // No leader initially
        }
    }

    /// Adds a new node to the cluster.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to add to the cluster.
    ///
    pub async fn add_node(&self, node: Node) {
        let mut nodes = self.nodes.write().await;
        
        // Insert the cloned node into the map
        nodes.insert(node.id, node.clone());
        
        println!("Node added to the cluster: {:?}", node);
    }

    /// Removes a node from the cluster by its ID.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node to remove.
    pub async fn remove_node(&self, node_id: Uuid) {
        let mut nodes = self.nodes.write().await;
        nodes.remove(&node_id);
        println!("Node removed from the cluster: {:?}", node_id);
    }

    /// Elects a leader from the nodes in the cluster.
    pub async fn elect_leader(&self) {
        let mut leader = self.leader.lock().await;
        let nodes = self.nodes.read().await;

        // Choose the first node as the leader for simplicity
        if let Some((leader_id, _)) = nodes.iter().next() {
            *leader = Some(*leader_id);
            println!("Leader elected: {:?}", leader_id);
        }
    }

    /// Starts listening for other nodes joining the cluster and heartbeat messages.
    pub async fn start_listener(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.self_node.address).await?;
        println!("Listening on {}", self.self_node.address);
    
        loop {
            let (_socket, _) = listener.accept().await?;
            println!("Connection established with another node");
    
            // Handle incoming requests from other nodes (e.g., heartbeats, join requests)
            tokio::spawn(async move {
                // Handle the socket (e.g., read messages, send responses)
            });
        }
    }

    /// Sends a heartbeat message to all other nodes in the cluster to indicate this node is still alive.
    pub async fn send_heartbeat(&self) {
        let nodes = self.nodes.read().await;
        for (_, node) in nodes.iter() {
            if node.id != self.self_node.id {
                // Send heartbeat to each node (for simplicity, we just print here)
                println!("Sending heartbeat to node at {}", node.address);
            }
        }
    }

    /// Performs a health check on the nodes by checking the last heartbeat time.
    ///
    /// If any nodes have not sent a heartbeat within the threshold, they are removed from the cluster.
    pub async fn perform_health_check(&self) {
        let mut nodes = self.nodes.write().await;
        let current_time = Instant::now();

        nodes.retain(|_, node| {
            let is_alive = current_time.duration_since(node.last_heartbeat) < Duration::from_secs(30);
            if !is_alive {
                println!("Node at {} failed health check and was removed", node.address);
            }
            is_alive
        });
    }
}
