pub mod mangostine_v0;

use mangostine_v0::rpc_server::{Rpc, RpcServer};
use mangostine_v0::{Transaction, TransactionResponse};
use tonic::{transport::Server, Request, Response, Status};

// defining a struct for our service
#[derive(Default)]
pub struct Mangostine {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Rpc for Mangostine {
    // our rpc impelemented as function
    async fn send_tx(
        &self,
        request: Request<Transaction>,
    ) -> Result<Response<TransactionResponse>, Status> {
        // returning a response as SayResponse message as defined in .proto
        Ok(Response::new(TransactionResponse {
            // reading data from request which is awrapper around our SayRequest message defined in .proto
            message: format!("sent transaction: {:?}", request.get_ref()),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "0.0.0.0:26657".parse().unwrap();
    // creating a service
    let rpc = Mangostine::default();
    println!("Mangostine Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(RpcServer::new(rpc))
        .serve(addr)
        .await?;
    Ok(())
}
