extern crate rumble;

use crate::bluetooth_rpc::{
  server,
  StartScanRequest,
  StartScanReply,
  FoundDevice,
  ListFoundDevicesRequest,
  ListFoundDevicesReply,
  PairDeviceRequest,
  PairDeviceReply, 
};

use std::sync::Arc;
use futures::{future, Future, Stream};
use log::error;
use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use tower_hyper::server::{Http, Server};
use rumble::bluez::manager::Manager;
use rumble::bluez::adapter::ConnectedAdapter;
use rumble::api::{Central, CentralEvent, Peripheral};

pub mod bluetooth_rpc {
  include!(concat!(env!("OUT_DIR"), "/bluetoothrpc.rs"));
}

#[derive(Clone)]
struct BluetoothInfo {
  adapter: ConnectedAdapter,
}

// impl BluetoothInfo {
//   fn new() -> BluetoothInfo {
//     let manager = Manager::new().unwrap();
//     let adapters = manager.adapters().unwrap();
//     let mut adapter = adapters.into_iter().nth(0).unwrap();

//     adapter = manager.down(&adapter).unwrap();
//     println!("Adapter: {:?}", adapter);
//     adapter = manager.up(&adapter).unwrap();

//     let central = adapter.connect().unwrap();
    
//     BluetoothInfo {
//       manager: manager,
//       adapter: central,
//     }
//   }
// }

#[derive(Clone)]
struct BluetoothRpc {
  bluetoothInfo: BluetoothInfo,
}

fn on_event_handler(adapter: & 'static ConnectedAdapter) -> Box<Fn(CentralEvent) + std::marker::Send> {
  Box::new(move |event| {
    println!("EVENT: {:?}", event);
    println!("PERIPHS: {:?}", adapter.peripherals());
  })
}

impl server::BluetoothRpc for BluetoothRpc {
    type StartScanFuture = future::FutureResult<Response<StartScanReply>, tower_grpc::Status>;
    type ListFoundDevicesFuture = future::FutureResult<Response<ListFoundDevicesReply>, tower_grpc::Status>;
    type PairDeviceFuture = future::FutureResult<Response<PairDeviceReply>, tower_grpc::Status>;

    fn start_scan(&mut self, request: Request<StartScanRequest>) -> Self::StartScanFuture {
        println!("REQUEST = {:?}", request);

        self.bluetoothInfo.adapter.start_scan().unwrap();

        let cloned_adapter = Arc::new(self.bluetoothInfo.adapter.clone());
        self.bluetoothInfo.adapter.on_event(
          Box::new(move |event| {
            println!("on_event {:?}", event.to_owned());
          })
        );
          
        let response = Response::new(StartScanReply {
            success: true,
        });

        future::ok(response)
    }

    fn list_found_devices(&mut self, request: Request<ListFoundDevicesRequest>) -> Self::ListFoundDevicesFuture {
        println!("REQUEST = {:?}", request);

        let peripherals = self.bluetoothInfo.adapter.peripherals().into_iter()
          .map(|peripheral| {
            let found_device = FoundDevice {
              address: peripheral.address().to_string(),
              name: peripheral.properties().local_name.unwrap_or(String::from("Unknown")),
            };

            found_device
          });

        let response = Response::new(ListFoundDevicesReply {
            devices: peripherals.collect(),
        });

        future::ok(response)
    }

    fn pair_device(&mut self, request: Request<PairDeviceRequest>) -> Self::PairDeviceFuture {
        println!("REQUEST = {:?}", request);

        let response = Response::new(PairDeviceReply {
            success: true,
        });

        future::ok(response)
    }
}

pub fn main() {
    let _ = ::env_logger::init();

    let manager = Manager::new().unwrap();
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    adapter = manager.down(&adapter).unwrap();
    println!("Adapter: {:?}", adapter);
    adapter = manager.up(&adapter).unwrap();

    let central = adapter.connect().unwrap();
    
    let bluetoothInfo = BluetoothInfo {
      adapter: central,
    };

    let new_service = server::BluetoothRpcServer::new(BluetoothRpc {
      bluetoothInfo: bluetoothInfo,
    });

    let mut server = Server::new(new_service);

    let http = Http::new().http2_only(true).clone();

    let addr = "[::1]:50051".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = server.serve_with(sock, http.clone());
            tokio::spawn(serve.map_err(|e| error!("hyper error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    tokio::run(serve)
}
