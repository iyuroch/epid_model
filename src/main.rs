// TODO: reorganize this as separate crate
// TODO: apis for
// TODO: handler state per connection

extern crate env_logger;
extern crate actix;
extern crate actix_web;

#[macro_use] 
extern crate serde_derive;

use std::sync::{Arc, Mutex};
use actix_web::{error, middleware, server, App, HttpRequest, Json, fs::NamedFile};

use error::{Error};


mod epid_sample;
use epid_sample::individual::{InfectionData};
use epid_sample::individual_group::IndividualGroup;


#[derive(Serialize)]
struct MyGroup {
    ind_group: Vec<((u32, u32), bool, Option<u32>)>,
}

struct AppState {
    ind_group: Arc<Mutex<IndividualGroup>>,
}

fn api(req: &HttpRequest<AppState>) -> Result<Json<MyGroup>, Error> {
    println!("{:?}", req);
    let mut ind_group = req.state().ind_group.lock().unwrap();
    ind_group.make_turns(1);

    Ok(Json(MyGroup {
        ind_group: ind_group.get_individuals(),
    }))
}

fn index(_req: &HttpRequest<AppState>) -> Result<NamedFile, Error> {
    // 
    Ok(NamedFile::open("/home/yurochko/Main/study/epid_model/src/index.html")?)
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
        App::with_state(AppState{ind_group: new_ind.clone()})
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(index))
            .resource("/api", |r| r.f(api))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");

    let _ = sys.run();
    // api's
}
