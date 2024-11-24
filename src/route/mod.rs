use actix_web::{Error, rt, web, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

pub async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(_)) => {
                    // echo text message
                    session.text("Hello Peter").await.unwrap();
                }

                Ok(AggregatedMessage::Binary(_)) => {
                    // echo binary message
                    session.binary("Hello Bin").await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}
