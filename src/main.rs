// Based on:
// https://tokio.rs/tokio/tutorial/shared-state
// https://github.com/alttch/rmodbus/blob/master/examples/servers/tcp.rs
use std::io;
use std::sync::{Arc, Mutex};

use rmodbus::{
    server::{context::ModbusContext, ModbusFrame},
    ModbusFrameBuf, ModbusProto,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn process_socket(
    unit: u8,
    modbus_context: Arc<std::sync::Mutex<ModbusContext>>,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let ip = stream.peer_addr()?;
    println!("Client connected: {}", &ip);
    loop {
        let mut buf: ModbusFrameBuf = [0; 256];
        let mut response = Vec::new();
        if stream.read(&mut buf).await.unwrap_or(0) == 0 {
            break;
        }
        let mut frame = ModbusFrame::new(unit, &buf, ModbusProto::TcpUdp, &mut response);
        let parsing_result = frame.parse();
        if parsing_result.is_err() {
            println!(
                "Frame parsing error {:?}. Buffer={:?}",
                parsing_result, &buf
            );
            return parsing_result.map_err(|err_kind| format!("{:?}", err_kind).into());
        }
        if frame.processing_required {
            let result = match frame.readonly {
                true => frame.process_read(&modbus_context.lock().unwrap()),
                false => frame.process_write(&mut modbus_context.lock().unwrap()),
            };
            if result.is_err() {
                println!("Frame processing error {:?}", result);
                return result.map_err(|err_kind| format!("{:?}", err_kind).into());
            }
        }

        let mut result: Vec<u16> = vec![];
        modbus_context
            .lock()
            .unwrap()
            .get_holdings_bulk(0, 10, &mut result)?;
        println!("16-bit registers from address 0 to 9: {:?}", result);

        if frame.response_required {
            frame.finalize_response()?;
            println!(
                "Responding to client {} with: {:x?}",
                &ip,
                response.as_slice()
            );
            let stream_write_response = stream.write(response.as_slice()).await;
            if stream_write_response.is_err() {
                return stream_write_response.map(|_| ()).map_err(|err| err.into());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "0.0.0.0:5502";
    let mut listener = TcpListener::bind(addr).await?;
    // The Arc and Mutex allow sharing the context between tokio's async green threads
    let modbus_context = Arc::new(Mutex::new(ModbusContext::new()));

    println!("Listening on {}", addr);
    loop {
        let (mut socket, _) = listener.accept().await?;
        let modbus_context_wrapper_copy = modbus_context.clone();
        tokio::spawn(async move {
            let result = process_socket(1, modbus_context_wrapper_copy, &mut socket).await;
            if result.is_err() {
                println!("Error: {:?}", result);
            }
        });
    }
}
