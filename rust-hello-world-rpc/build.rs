fn main() {
  let proto_dir = String::from("../proto");

  tower_grpc_build::Config::new()
    .enable_server(true)
    .enable_client(false)
    .build(
      &[format!("{}{}", proto_dir, "/helloworld.proto")],
      &[proto_dir.clone()],
    )
    .unwrap_or_else(|e| panic!("Protobuff falied to compile: {}", e));

  tower_grpc_build::Config::new()
    .enable_server(true)
    .enable_client(false)
    .build(
      &[format!("{}{}", proto_dir, "/bluetooth.proto")],
      &[proto_dir.clone()],
    )
    .unwrap_or_else(|e| panic!("Protobuff falied to compile: {}", e));
}
