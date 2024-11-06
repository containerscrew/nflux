// Read events from ring buffer
// let cpus = online_cpus().unwrap();
// let num_cpus = cpus.len();
// let mut events = AsyncPerfEventArray::try_from(bpf.map_mut("CONNECTION_EVENTS").unwrap())?;

// for cpu in cpus {
//     let mut buf = events.open(cpu, None)?;
//
//     tokio::spawn(async move {
//         let mut buffers = (0..num_cpus)
//             .map(|_| BytesMut::with_capacity(9000))
//             .collect::<Vec<_>>();
//
//         loop {
//             // Attempt to read events from the perf buffer into the prepared buffers.
//             let events = match buf.read_events(&mut buffers).await {
//                 Ok(events) => events,
//                 Err(e) => {
//                     warn!("Error reading events: {}", e);
//                     continue;
//                 }
//             };
//
//             // Iterate over the number of events read. `events.read` indicates how many events were read.
//             for i in 0..events.read {
//                 let buf = &mut buffers[i];
//                 let data = buf.as_ptr() as *const ConnectionEvent; // Cast the buffer pointer to a Data pointer.
//                 info!("{:?}", unsafe { *data });
//             }
//         }
//     });
// }

// Log connection events for TCP and UDP
// fn log_connection_event(
//     ctx: &XdpContext,
//     src_addr: u32,
//     dst_addr: u32,
//     src_port: u16,
//     dst_port: u16,
//     protocol: u8,
// ) {
//     let event = ConnectionEvent {
//         src_addr,
//         dst_addr,
//         src_port,
//         dst_port,
//         protocol,
//     };
//     let _ = CONNECTION_EVENTS.output(ctx, &event, 0);
// }
