// Based on:
// https://tokio.rs/tokio/tutorial/shared-state
// https://github.com/alttch/rmodbus/blob/master/examples/servers/tcp.rs
use std::io;
use std::sync::{Arc, Mutex};

use rmodbus::server::{context::ModbusContext, ModbusFrame};
use rmodbus::{ModbusFrameBuf, ModbusProto};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// Modbus server configuration
const SERVER_UNIT: u8 = 1;
const ADDR: &str = "0.0.0.0:5502";

fn print_modbus_context(
    modbus_context: &Arc<std::sync::Mutex<ModbusContext>>,
    start_addr: u16,
    count: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut result: Vec<u16> = vec![];
    modbus_context
        .lock()
        .unwrap()
        .get_holdings_bulk(start_addr, count, &mut result)?;
    println!(
        "16-bit registers from address {} to {}: {:?}",
        start_addr,
        start_addr + count,
        result
    );
    Ok(())
}

async fn handle_connection(
    modbus_context: Arc<std::sync::Mutex<ModbusContext>>,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let ip = stream.peer_addr()?;
    println!("New TCP connection from {}", &ip);
    // Repeatedly read modbus frames from this TCP connection
    loop {
        let mut buf: ModbusFrameBuf = [0; 256];
        let mut response = Vec::new();
        // Read from socket until there's no more data immediately available
        if stream.read(&mut buf).await.unwrap_or(0) == 0 {
            break;
        }

        // Parse bytes into a modbus frame
        let mut frame = ModbusFrame::new(SERVER_UNIT, &buf, ModbusProto::TcpUdp, &mut response);
        if let Err(err) = frame.parse() {
            println!("Modbus frame parsing error {}. Buffer={:?}", err, &buf);
            // Convert an rmodbus error into this function's error type
            return Err(err.into());
        }

        // Modify modbus context if necessary
        if frame.processing_required {
            let result = match frame.readonly {
                true => frame.process_read(&modbus_context.lock().unwrap()),
                false => frame.process_write(&mut modbus_context.lock().unwrap()),
            };
            if let Err(err) = result {
                println!("Modbus frame processing error {}", err);
                return Err(err.into());
            }
        }

        print_modbus_context(&modbus_context, 0, 16)?;

        // Send response to client if necessary
        if frame.response_required {
            frame.finalize_response()?;
            println!("Responding to {}: {:?}", &ip, response);
            if let Err(err) = stream.write(response.as_slice()).await {
                println!("Sending response to client failed: {}", err);
                return Err(err.into());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(ADDR).await?;
    // The Arc and Mutex allow sharing the context between tokio's async green threads
    let modbus_context = Arc::new(Mutex::new(ModbusContext::new()));

    println!("Modbus server unit={} listening on {}", SERVER_UNIT, ADDR);
    loop {
        let (mut socket, _) = listener.accept().await?;
        let modbus_context_arc_copy = modbus_context.clone();
        tokio::spawn(async move {
            if let Err(err) = handle_connection(modbus_context_arc_copy, &mut socket).await {
                println!("Error: {}", err);
            }
        });
    }
}
