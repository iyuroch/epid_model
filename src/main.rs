// TODO: reorganize this as separate crate
// TODO: apis for
// TODO: handler state per connection
extern crate futures;
extern crate tokio;
extern crate tokio_threadpool;
extern crate tokio_tungstenite;
extern crate tungstenite;

#[macro_use] 
extern crate serde_derive;

use std::time::Duration;
use std::env;
use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::thread;

use futures::stream::Stream;
use futures::Future;
use futures::future::{lazy, poll_fn};
use futures::Sink;
use tokio::timer::Interval;
use tokio::net::TcpListener;
use tokio_threadpool::blocking;
use tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;

mod epid_sample;
use epid_sample::individual::{InfectionData};
use epid_sample::individual_group::{IndividualGroup, GroupMetadata};

mod static_server;
use static_server::static_server::*;

fn fibonacci(n: u32) -> u128 {
    let mut x_0: u128 = 0;
    let mut x_1: u128 = 1;
	// let (x_0: u128, mut x_1: u128) = (0, 1);
	for _ in 0..n {
		let y = x_0;
		x_0 = x_1;
		x_1 = y + x_1;
	}
	x_1
}


// #[derive(Serialize)]
// struct MyGroup {
//     ind_group: Vec<((u32, u32), bool, Option<u32>)>,
// }

// #[derive(Serialize)]
// struct MyMeta {
//     meta_data: GroupMetadata,
// }



// fn api(req: &HttpRequest<AppState>) -> Result<Json<MyGroup>, Error> {
//     println!("{:?}", req);
//     let mut ind_group = req.state().ind_group.lock().unwrap();
//     ind_group.make_turns(1);

//     Ok(Json(MyGroup {
//         ind_group: ind_group.get_individuals(),
//     }))
// }


// fn meta(req: &HttpRequest<AppState>) -> Result<Json<MyMeta>, Error> {
//     println!("{:?}", req);
//     let ind_group = req.state().ind_group.lock().unwrap();

//     Ok(Json(MyMeta {
//         meta_data: ind_group.get_group_metadata(),
//     }))
// }

fn main() {

    let act = thread::spawn(move || {
        run();
    });

    // let inf_data = InfectionData::new(
    //     15, 1.0, 6, 2,
    // );

    // let new_ind = Arc::new(Mutex::new(IndividualGroup::new(
    //     100, 100, 6, 30, 5, inf_data,
    // )));

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8081".to_string());
    let addr = addr.parse().unwrap();

    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let srv = socket.incoming().for_each(move |stream| {

        let addr = stream.peer_addr().expect("connected streams should have a peer address");
        println!("Peer address: {}", addr);

        accept_async(stream).and_then(move |ws_stream| {
            println!("New WebSocket connection: {}", addr);

            // Let's split the WebSocket stream, so we can work with the
            // reading and writing halves separately.
            let (sink, stream) = ws_stream.split();
            let sink_cell = Arc::new(Mutex::new(sink));

            let ws_reader = stream.for_each(move |message: Message| {
                println!("Received a message from {}: {}", addr, message);
                Ok(())
            });
			
			let counter = Arc::new(Mutex::new(0));

			let ws_writer = Interval::new_interval(Duration::from_millis(3000))
							.for_each(move |_| {

                                let counter = counter.clone();
                                let sink_cell = sink_cell.clone();

								let fut = lazy(move || {
                                    let sink_cell = sink_cell.clone();
                                    let counter = counter.clone();
									poll_fn(move || {
                                        let sink_cell = sink_cell.clone();
                                        let counter = counter.clone();
										blocking(move || {
                                            let mut counter = counter.lock().unwrap();
                                            *counter += 1;
											let msg = fibonacci(*counter);
                                            let mut sink = sink_cell.lock().unwrap();
                                            sink.start_send(Message::from(msg.to_string())).unwrap();
										}).map_err(|_| panic!("the threadpool shut down"))
									})
								});

								tokio::spawn(
                                    fut
                                );
                                Ok(())
                            })
							.map_err(|e| panic!("interval errored; err={:?}", e));

            let connection = ws_reader.map(|_| ()).map_err(|_| ())
                                      .select(ws_writer.map(|_| ()).map_err(|_| ()));

            tokio::spawn(connection.then(move |_| {
                println!("Connection {} closed.", addr);
                Ok(())
            }));

            Ok(())
        }).map_err(|e| {
            println!("Error during the websocket handshake occurred: {}", e);
            Error::new(ErrorKind::Other, e)
        })
    });

    tokio::run(srv.map_err(|_e| ()));


    act.join().unwrap();
}
