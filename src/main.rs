// TODO: reorganize this as separate crate
// TODO: apis for
extern crate env_logger;
extern crate actix;
extern crate actix_web;

use std::sync::{Arc, Mutex};
use actix_web::{middleware, server, App, HttpRequest, HttpResponse};

mod epid_sample;

use epid_sample::individual::{InfectionData};
use epid_sample::individual_group::IndividualGroup;

struct AppState {
    ind_group: Arc<Mutex<IndividualGroup>>,
}

fn index(req: &HttpRequest<AppState>) -> HttpResponse {
    println!("{:?}", req);
    let mut ind_group = req.state().ind_group.lock().unwrap();
    // let mut ind_group = req.state().ind_group.lock().unwrap();
    // ind_group.make_turns(1);
    // req.state().ind_group.lock().unwrap().make_turns(1);
    // let data = ind_group.get_individuals();
    ind_group.make_turns(1);
    // println!("{:?}", ind_group.get_individuals());

    HttpResponse::Ok().body(
        format!("Population: {:?}", 
            "Yo")
    )
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let inf_data = InfectionData::new(
        15, 1.0, 6, 3,
    );

    let new_ind = Arc::new(Mutex::new(IndividualGroup::new(
        100, 100, 2, 30, 5, inf_data,
    )));

    let sys = actix::System::new("ws-example");

    //move is necessary to give closure below ownership of counter
    server::new(move || {
        App::with_state(AppState{ind_group: new_ind.clone()}) // <- create app with shared state
            // enable logger
            .middleware(middleware::Logger::default())
            // register simple handler, handle all methods
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");

    let _ = sys.run();
    // api's
}
