// async fn process_egress_events(
//     mut buf: AsyncPerfEventArrayBuffer<MapData>,
//     cpu_id: u32,
// ) -> Result<(), PerfBufferError> {
//     let mut buffers = vec![BytesMut::with_capacity(1024); 10];

//     loop {
//         // Wait for events
//         let events = buf.read_events(&mut buffers).await?;

//         // Process each event in the buffer
//         for i in 0..events.read {
//             let buf = &buffers[i];
//             match parse_egress_event(buf) {
//                 Ok(event) => {
//                     info!(
//                         "direction=outgoing ip={}, port={}, fqdn={}",
//                         Ipv4Addr::from(event.dst_ip),
//                         event.dst_port,
//                         lookup_address(event.dst_ip),
//                     );
//                 }
//                 Err(e) => error!("Failed to parse egress event on CPU {}: {}", cpu_id, e),
//             }
//         }
//     }
// }


// fn parse_egress_event(buf: &BytesMut) -> anyhow::Result<EgressEvent> {
//     if buf.len() >= std::mem::size_of::<EgressEvent>() {
//         let ptr = buf.as_ptr() as *const EgressEvent;
//         let event = unsafe { ptr::read_unaligned(ptr) };
//         Ok(event)
//     } else {
//         Err(anyhow::anyhow!(
//             "Buffer size is too small for EgressEvent"
//         ))
//     }
// }
