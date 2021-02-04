use tonic::{transport::Server, Request, Response, Status};
use landing::landing_service_server::{LandingService, LandingServiceServer};
use landing::{TalkRequest, TalkResponse, ResultType, TalkResult};
use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::prelude::*;
use rand::Rng;

static hellos: Vec<&str> = vec!["Hello", "Bonjour", "Hola", "こんにちは", "Ciao", "안녕하세요"];

pub mod landing {
    tonic::include_proto!("org.feuyeux.grpc");
}

#[derive(Default)]
pub struct ProtoServer {}

#[tonic::async_trait]
impl LandingService for ProtoServer {
    type TalkOneAnswerMoreStream = Pin<Box<dyn Stream<Item=Result<TalkResponse, Status>> + Send + Sync + 'static>>;
    type TalkBidirectionalStream = Pin<Box<dyn Stream<Item=Result<TalkResponse, Status>> + Send + Sync + 'static>>;

    async fn talk(
        &self,
        request: Request<TalkRequest>,
    ) -> Result<Response<TalkResponse>, Status> {
        let result = buildResult(request.get_ref().data);
        let response = TalkResponse {
            status: 200,
            results: vec![result],
        };
        Ok(Response::new(response))
    }

    async fn talk_one_answer_more(
        &self,
        request: tonic::Request<TalkRequest>,
    ) -> Result<tonic::Response<Self::TalkOneAnswerMoreStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(4);

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn talk_more_answer_one(
        &self,
        request: tonic::Request<tonic::Streaming<TalkRequest>>,
    ) -> Result<tonic::Response<TalkResponse>, tonic::Status> {
        Ok(Response::new(TalkResponse::default()))
    }

    async fn talk_bidirectional(
        &self,
        request: tonic::Request<tonic::Streaming<TalkRequest>>,
    ) -> Result<tonic::Response<Self::TalkBidirectionalStream>, tonic::Status> {
        let output = async_stream::try_stream! {

        };
        Ok(Response::new(Box::pin(output) as Self::TalkBidirectionalStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(LandingServiceServer::new(ProtoServer {}))
        .serve(addr)
        .await?;

    Ok(())
}

fn buildResult(id: String) -> TalkResult {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..5);
    let uuid = Uuid::new_v4();
    map.insert("id".to_string(), uuid.to_string());
    map.insert("idx".to_string(), id);
    map.insert("data".to_string(), hellos[index].to_string());
    map.insert("meta".to_string(), "RUST".to_string());
    let ok = ResultType::Ok as i32;
    let result = TalkResult {
        id: Utc::now().timestamp_millis(),
        r#type: ok,
        kv: map,
    };
    return result;
}