// TODO: reorganize this as separate crate
// TODO: apis for
// TODO: handler state per connection
extern crate futures;
extern crate tokio;
extern crate tokio_threadpool;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate serde_json;

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
use serde_json::json;

mod epid_sample;
use epid_sample::individual::{InfectionData};
use epid_sample::individual_group::{IndividualGroup, GroupMetadata};

mod static_server;
use static_server::static_server::*;


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

    let _act = thread::spawn(move || {
        run();
    });

    
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

            let inf_data = InfectionData::new(
                15, 1.0, 6, 2,
            );

            let new_group = Arc::new(Mutex::new(IndividualGroup::new(
                100, 100, 6, 30, 5, inf_data,
            )));

            let reader_sink_cell = sink_cell.clone();
            let reader_new_group = new_group.clone();

            let ws_reader = stream.for_each(move |message: Message| {
                // we match "client" requests here and send them response
                // this is some type of rpc
                // it might be better idea to implement this with
                // basic rest api, but why not this way?)
                match message.to_text().unwrap() {
                    "get_meta" => {
                        let mut sink = reader_sink_cell.lock().unwrap();
                        let new_group = reader_new_group.lock().unwrap();
                        sink.start_send(Message::from(json!(
                            {
                                "name": "meta_data",
                                "data": new_group.get_group_metadata()
                            }
                        ).to_string())).unwrap();
                    },
                    _ => {},
                }
                // println!("Received a message from {}: {}", addr, message);
                Ok(())
            });

            // let sink_cell = sink_cell.clone();

			let ws_writer = Interval::new_interval(Duration::from_millis(3000))
							.for_each(move |_| {

                                let new_group = new_group.clone();
                                let sink_cell = sink_cell.clone();

								let fut = lazy(move || {
                                    let sink_cell = sink_cell.clone();
                                    let new_group = new_group.clone();
									poll_fn(move || {
                                        let turn_num = 1;
                                        let sink_cell = sink_cell.clone();
                                        let new_group = new_group.clone();
										blocking(move || {
                                            // first we need to get individuals
                                            // then push information about their position
                                            // through sink
                                            // and make turn
                                            let mut new_group = new_group.lock().unwrap();
                                            new_group.get_individuals();
                                            let mut sink = sink_cell.lock().unwrap();
                                            sink.start_send(Message::from(json!(
                                                {
                                                    "name": "ind_group",
                                                    "data": new_group.get_individuals()
                                                }
                                            ).to_string())).unwrap();
                                            new_group.make_turns(turn_num);
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

    // act.join().unwrap();
}
