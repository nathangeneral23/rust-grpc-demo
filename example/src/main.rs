fn main() {
    let msg = rust_grpc_demo_api::types::HelloReply {
        message: "Hello, World!".to_owned(),
    };

    println!("{:#?}", msg);
}
