use aya::maps::{MapData, RingBuf};
use nflux_common::{Kind, TLSData};
use tracing::info;

fn _get_preface_header(data: &[u8]) -> &[u8] {
    if data.len() >= 24 {
        &data[0..24]
    } else {
        &[]
    }
}

pub async fn process_tls_event(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            // Get the data from the event
            let data = event.as_ref();

            // Make sure the data is the correct size
            if data.len() == std::mem::size_of::<TLSData>() {
                let event: &TLSData = unsafe { &*(data.as_ptr() as *const TLSData) };

                // https://github.com/mlalic/hpack-rs/tree/master
                // https://docs.rs/httlib-hpack/latest/httlib_hpack/

                // Decoding the HTTP/2 headers using HPACK
                // let mut decoder = Decoder::default();
                // let mut dst = Vec::new();

                // // Decode the data in event.buf as a raw HPACK buffer
                // decoder.decode(&mut event.buf.to_vec(), &mut dst).unwrap();

                // Log the connection and data with the decoded headers included
                //let buffer_limit = event.buf.len().min(event.len as usize);
                // let mut buffer: Vec<u8> = event.buf.to_vec().clone();
                // if buffer.is_empty() {
                //     warn!("Empty buffer");
                // } else {
                //     let mut decoder = Decoder::default();
                //     let mut dst = Vec::new();
                //     println!("Buffer: {:?}", buffer);
                //     decoder.decode(&mut buffer, &mut dst).unwrap();
                // }
                // Remove first 24 bytes from the buffer
                // if event.kind == Kind::Read {
                //     // Obtener y mostrar el Preface Header
                //     let preface_header = get_preface_header(&event.buf);
                //     println!("Preface Header: {:?}", preface_header.to_vec());

                //     // El resto del buffer se considera HPACK
                //     let hpack_data = &mut event.buf[24..];
                //     println!("Buffer (HPACK): {:?}", hpack_data.to_vec());

                //     // Decodificar los datos HPACK
                //     let mut decoder = Decoder::default();
                //     let mut headers = Vec::new();

                //     let src = &mut event.buf[24..].to_vec();

                //     decoder.decode(src, &mut headers)
                // }

                info!(
                    "comm={}, kind={}, len={}, buf=\n{}",
                    String::from_utf8_lossy(&event.comm)
                        .trim_end_matches(char::from(0)),
                    match event.kind {
                        Kind::Read => "read",
                        Kind::Write => "write",
                    },
                    event.len,
                    String::from_utf8_lossy(&event.buf[..event.len as usize]),
                );
            }
        }

        // Sleep for a while
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

// Attach uprobes for NSS
// let program: &mut UProbe = ebpf.program_mut("pr_read").unwrap().try_into()?;
// program.load()?;
// program.attach(Some("PR_Read"), 0, NSS_LIB_PATH, cli.pid)?;

// let program: &mut UProbe = ebpf.program_mut("pr_write").unwrap().try_into()?;
// program.load()?;
// program.attach(Some("PR_Write"), 0, NSS_LIB_PATH, cli.pid)?;

// //Attach uretprobes for NSS
// let p_write_ret: &mut UProbe = ebpf.program_mut("pr_read_ret").unwrap().try_into()?;
// p_write_ret.load()?;
// p_write_ret.attach(Some("PR_Read"), 0, NSS_LIB_PATH, cli.pid)?;

// let p_write_ret: &mut UProbe = ebpf.program_mut("pr_write_ret").unwrap().try_into()?;
// p_write_ret.load()?;
// p_write_ret.attach(Some("PR_Write"), 0, NSS_LIB_PATH, cli.pid)?;
