use futures::stream;
use landing::landing_service_client::LandingServiceClient;
use landing::{TalkRequest, TalkResponse};
use rand::Rng;
use std::time::Duration;
use tokio::time;
use tonic::{Request};
use env_logger::Env;
use log::info;

pub mod landing {
    tonic::include_proto!("org.feuyeux.grpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).format_timestamp_millis().init();
    let address = "http://[::1]:9996";
    let mut client = LandingServiceClient::connect(address).await?;
    info!("Talk");

    let message = TalkRequest {
        data: "0".to_string(),
        meta: "RUST".to_string(),
    };
    let mut request = Request::new(message);
    request.metadata_mut().insert("k1", "v1".parse().unwrap());
    request.metadata_mut().insert("k2", "v2".parse().unwrap());

    let response = client.talk(request).await?;
    print_response(response.get_ref());

    info!("TalkOneAnswerMore");
    let mut request = Request::new(TalkRequest {
        data: "0,1,2".to_string(),
        meta: "RUST".to_string(),
    });
    request.metadata_mut().insert("k1", "v1".parse().unwrap());
    request.metadata_mut().insert("k2", "v2".parse().unwrap());
    let mut stream = client
        .talk_one_answer_more(request)
        .await?
        .into_inner();

    while let Some(response) = stream.message().await? {
        print_response(&response);
    }

    info!("TalkMoreAnswerOne");
    let mut requests = vec![];
    requests.push(TalkRequest {
        data: random_id(5),
        meta: "RUST".to_string(),
    });
    requests.push(TalkRequest {
        data: random_id(5),
        meta: "RUST".to_string(),
    });
    requests.push(TalkRequest {
        data: random_id(5),
        meta: "RUST".to_string(),
    });

    let mut request = Request::new(stream::iter(requests));
    request.metadata_mut().insert("k1", "v1".parse().unwrap());
    request.metadata_mut().insert("k2", "v2".parse().unwrap());
    match client.talk_more_answer_one(request).await {
        Ok(response) => print_response(&response.into_inner()),
        Err(e) => info!("something went wrong: {:?}", e),
    }

    info!("TalkBidirectional");
    let mut interval = time::interval(Duration::from_secs(1));
    let mut times = 3;
    let outbound = async_stream::stream! {
        while times > 0 {
            interval.tick().await;
            let request = TalkRequest { data: random_id(5), meta: "RUST".to_string() };
            yield request;
            times -= 1;
        }
    };

    let mut request = Request::new(outbound);
    request.metadata_mut().insert("k1", "v1".parse().unwrap());
    request.metadata_mut().insert("k2", "v2".parse().unwrap());
    let response = client.talk_bidirectional(request).await?;
    let mut inbound = response.into_inner();
    while let Some(resp) = inbound.message().await? {
        print_response(&resp);
    }
    info!("Done");
    Ok(())
}

fn random_id(max: i32) -> String {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..max);
    let s = format!("{}", r);
    s
}

fn print_response(response: &TalkResponse) {
    for result in &response.results {
        let map = &result.kv;
        let (meta, id, idx, data): (String, String, String, String);
        match map.get("meta") {
            Some(_meta) => meta = _meta.to_string(),
            None => meta = "".to_string(),
        }
        match map.get("id") {
            Some(_id) => id = _id.to_string(),
            None => id = "".to_string(),
        }
        match map.get("idx") {
            Some(_idx) => idx = _idx.to_string(),
            None => idx = "".to_string(),
        }
        match map.get("data") {
            Some(_data) => data = _data.to_string(),
            None => data = "".to_string(),
        }
        info!("[{:?}] {:?} [{:?} {:?} {:?},{:?}:{:?}]", response.status, result.id, meta, result.r#type, id, idx, data);
    }
}