use chrono::prelude::*;
use futures::{Stream, StreamExt};
use landing::landing_service_server::{LandingService, LandingServiceServer};
use landing::{ResultType, TalkRequest, TalkResponse, TalkResult};
use std::collections::HashMap;
use std::pin::Pin;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use uuid::Uuid;
use env_logger::Env;
use log::info;
use tonic::metadata::{KeyAndValueRef, MetadataMap};

static HELLOS: [&'static str; 6] = [
    "Hello",
    "Bonjour",
    "Hola",
    "こんにちは",
    "Ciao",
    "안녕하세요",
];

pub mod landing {
    tonic::include_proto!("org.feuyeux.grpc");
}

#[derive(Default)]
pub struct ProtoServer {}

#[tonic::async_trait]
impl LandingService for ProtoServer {
    async fn talk(&self, request: Request<TalkRequest>) -> Result<Response<TalkResponse>, Status> {
        let talk_request: &TalkRequest = request.get_ref();
        let data: &String = &talk_request.data;
        let meta: &String = &talk_request.meta;
        info!("TALK REQUEST: data={:?},meta={:?}", data, meta);
        print_metadata(request.metadata());
        let result = build_result(data.clone());
        let response = TalkResponse {
            status: 200,
            results: vec![result],
        };
        Ok(Response::new(response))
    }

    type TalkOneAnswerMoreStream =
    Pin<Box<dyn Stream<Item=Result<TalkResponse, Status>> + Send + Sync + 'static>>;

    async fn talk_one_answer_more(&self, request: Request<TalkRequest>) -> Result<Response<Self::TalkOneAnswerMoreStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let talk_request: &TalkRequest = request.get_ref();
            let data: &String = &talk_request.data;
            let meta: &String = &talk_request.meta;
            info!("TalkOneAnswerMore REQUEST: data={:?},meta={:?}", data, meta);
            print_metadata(request.metadata());
            let datas = data.split(",");
            for data in datas {
                let result = build_result(data.to_string());
                let response = TalkResponse {
                    status: 200,
                    results: vec![result],
                };
                tx.send(Ok(response)).await.unwrap();
            }
        });
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn talk_more_answer_one(&self, request: Request<tonic::Streaming<TalkRequest>>) -> Result<Response<TalkResponse>, Status> {
        info!("TalkMoreAnswerOne REQUEST: ");
        print_metadata(request.metadata());
        let mut stream = request.into_inner();
        let mut rs = vec![];
        while let Some(talk_request) = stream.next().await {
            let talk_request = talk_request?;
            let data: &String = &talk_request.data;
            let meta: &String = &talk_request.meta;
            info!("data={:?},meta={:?}", data, meta);
            let result = build_result(data.to_string());
            rs.push(result);
        }
        let response = TalkResponse {
            status: 200,
            results: rs,
        };
        Ok(Response::new(response))
    }

    type TalkBidirectionalStream =
    Pin<Box<dyn Stream<Item=Result<TalkResponse, Status>> + Send + Sync + 'static>>;

    async fn talk_bidirectional(&self, request: Request<Streaming<TalkRequest>>) -> Result<Response<Self::TalkBidirectionalStream>, Status> {
        info!("TalkBidirectional REQUEST:");
        print_metadata(request.metadata());
        let mut stream = request.into_inner();
        let output = async_stream::try_stream! {
            while let Some(talk_request) = stream.next().await {
                let talk_request = talk_request?;
                let data: &String = &talk_request.data;
                let meta: &String = &talk_request.meta;
                info!("data={:?},meta={:?}", data, meta);
                let result = build_result(data.to_string());
                let response = TalkResponse {
                    status: 200,
                    results:  vec![result],
                };
                yield response;
             }
        };
        Ok(Response::new(
            Box::pin(output) as Self::TalkBidirectionalStream
        ))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).format_timestamp_millis().init();
    let address = "[::1]:9996".parse().unwrap();
    Server::builder()
        .add_service(LandingServiceServer::new(ProtoServer {}))
        .serve(address)
        .await?;
    Ok(())
}

fn build_result(id: String) -> TalkResult {
    let mut map: HashMap<String, String> = HashMap::new();
    let index = id.parse::<usize>().unwrap();
    let uuid = Uuid::new_v4();
    map.insert("id".to_string(), uuid.to_string());
    map.insert("idx".to_string(), id);
    map.insert("data".to_string(), HELLOS[index].to_string());
    map.insert("meta".to_string(), "RUST".to_string());
    let ok = ResultType::Ok as i32;
    let result = TalkResult {
        id: Utc::now().timestamp_millis(),
        r#type: ok,
        kv: map,
    };
    return result;
}

fn print_metadata(header: &MetadataMap) {
    for kv in header.iter() {
        match kv {
            KeyAndValueRef::Ascii(ref k, ref v) => info!("H: {:?}: {:?}", k, v),
            KeyAndValueRef::Binary(ref k, ref v) => info!("H: {:?}: {:?}", k, v),
        }
    }
}