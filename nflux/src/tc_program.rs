// use anyhow::Context;
// use aya::{
//     Ebpf,
//     maps::{Array, RingBuf},
//     programs::{SchedClassifier, TcAttachType, tc},
// };
// use nflux_common::dto::Configmap;
// use tracing::{debug, error, warn};

// use crate::{
//     network_event::{process_arp_events, process_networking_event},
//     utils::wait_for_shutdown,
// };

// pub async fn start_traffic_control(
//     ebpf: &mut Ebpf,
//     interface: &str,
//     disable_egress: bool,
//     disable_ingress: bool,
//     configmap: Configmap,
//     log_format: String,
//     exclude_ports: Option<Vec<u16>>,
// ) -> anyhow::Result<()> {
//     // Attach TC programs and populate configmap
//     try_traffic_control(ebpf, interface, disable_ingress, disable_egress, configmap)?;

//     let tc_event_ring_map = ebpf
//         .take_map("NETWORK_EVENT")
//         .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer NETWORK_EVENT map"))?;
//     let ring_buf_net = RingBuf::try_from(tc_event_ring_map)?;

//     let arp_event_ring_map = ebpf
//         .take_map("ARP_EVENTS")
//         .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer ARP_EVENTS map"))?;
//     let ring_buf_arp = RingBuf::try_from(arp_event_ring_map)?;

//     let net_task = tokio::spawn(async move {
//         if let Err(e) = process_networking_event(ring_buf_net, log_format, exclude_ports).await {
//             error!("process_tc_events failed: {:?}", e);
//         }
//     });

//     let arp_task = tokio::spawn(async move {
//         if let Err(e) = process_arp_events(ring_buf_arp).await {
//             error!("process_arp_events failed: {:?}", e);
//         }
//     });

//     // Wait for shutdown signal or any task to end
//     tokio::select! {
//         _ = wait_for_shutdown() => {
//             warn!("You pressed Ctrl-C, shutting down nflux...");
//         }
//         _ = net_task => {
//             warn!("NETWORK_EVENT task ended");
//         }
//         _ = arp_task => {
//             warn!("ARP_EVENTS task ended");
//         }
//     }

//     Ok(())
// }

// fn try_traffic_control(
//     ebpf: &mut Ebpf,
//     interface: &str,
//     disable_ingress: bool,
//     disable_egress: bool,
//     configmap: Configmap,
// ) -> Result<(), anyhow::Error> {
//     if !disable_egress {
//         attach_tc_program(ebpf, "tc_egress", interface, TcAttachType::Egress)?;
//     }

//     if !disable_ingress {
//         attach_tc_program(ebpf, "tc_ingress", interface, TcAttachType::Ingress)?;
//     }

//     populate_configmap(ebpf, configmap)?;

//     Ok(())
// }

// fn attach_tc_program(
//     bpf: &mut Ebpf,
//     program_name: &str,
//     interface: &str,
//     attach_type: TcAttachType,
// ) -> anyhow::Result<()> {
//     // Retrieve the eBPF program
//     let program = match bpf.program_mut(program_name) {
//         Some(p) => p,
//         None => {
//             error!("Failed to find the {} program in BPF object", program_name);
//             return Err(anyhow::anyhow!("{} program not found", program_name));
//         }
//     };

//     // Try converting the program into a SchedClassifier
//     let program: &mut SchedClassifier = match program.try_into() {
//         Ok(p) => p,
//         Err(e) => {
//             error!(
//                 "Failed to convert {} program to SchedClassifier: {:?}",
//                 program_name, e
//             );
//             return Err(e.into());
//         }
//     };

//     // Load the eBPF program into the kernel
//     if let Err(e) = program.load() {
//         error!("Failed to load {} program: {:?}", program_name, e);
//         return Err(e.into());
//     }

//     // Iterate over interfaces and attach the program
//     if let Err(e) = tc::qdisc_add_clsact(interface) {
//         debug!(
//             "Failed to add clsact qdisc to interface {}: {:?}",
//             interface, e
//         );
//     }

//     // Attach the eBPF program to the egress path of the interface
//     if let Err(e) = program.attach(interface, attach_type) {
//         error!(
//             "Failed to attach {} program to interface {}: {:?}",
//             program_name, interface, e
//         );
//         return Err(anyhow::anyhow!(
//             "Failed to attach {} program to interface {}",
//             program_name,
//             interface
//         ));
//     }

//     Ok(())
// }

// fn populate_configmap(
//     bpf: &mut Ebpf,
//     config: Configmap,
// ) -> anyhow::Result<(), anyhow::Error> {
//     let mut tc_config = Array::<_, Configmap>::try_from(
//         bpf.map_mut("TC_CONFIG")
//             .context("Failed to find TC_CONFIG map")?,
//     )?;

//     tc_config
//         .set(0, config, 0)
//         .context("Failed to set TC_CONFIG")?;

//     debug!("eBPF map TC_CONFIG successfully loaded with struct Configmap");

//     Ok(())
// }
