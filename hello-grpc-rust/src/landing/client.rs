use futures::stream;
use landing::landing_service_client::LandingServiceClient;
use landing::TalkRequest;
use rand::Rng;
use std::time::Duration;
use tokio::time;
use tonic::Request;

pub mod landing {
    tonic::include_proto!("org.feuyeux.grpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LandingServiceClient::connect("http://[::1]:9996").await?;

    println!("Talk");
    let response = client
        .talk(Request::new(TalkRequest {
            data: "0".to_string(),
            meta: "RUST".to_string(),
        }))
        .await?;
    println!("RESPONSE = {:?}", response.get_ref());

    println!("TalkOneAnswerMore");
    let mut stream = client
        .talk_one_answer_more(Request::new(TalkRequest {
            data: "0,1,2".to_string(),
            meta: "RUST".to_string(),
        }))
        .await?
        .into_inner();

    while let Some(response) = stream.message().await? {
        println!("RESPONSE = {:?}", response);
    }

    println!("TalkMoreAnswerOne");
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

    let request = Request::new(stream::iter(requests));
    match client.talk_more_answer_one(request).await {
        Ok(response) => println!("RESPONSE = {:?}", response.into_inner()),
        Err(e) => println!("something went wrong: {:?}", e),
    }

    println!("TalkBidirectional");

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

    let response = client.talk_bidirectional(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();
    while let Some(resp) = inbound.message().await? {
        println!("RESPONSE = {:?}", resp);
    }
    println!("DONE");
    Ok(())
}

fn random_id(max: i32) -> String {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..max);
    let s = format!("{}", r);
    s
}
