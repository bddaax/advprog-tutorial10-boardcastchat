use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    // TODO: For a hint, see the description of the task below.
    println!("Connected to the chat!");

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                match line {
                    Ok(Some(msg)) => {
                        if !msg.trim().is_empty() {
                            ws_stream.send(Message::text(msg)).await?;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("stdin error: {e}");
                        break;
                    }
                }
            }

            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("{text}");
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Connection error: {e}");
                        break;
                    }
                    None => {
                        println!("Server closed connection.");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}