// defines static server to server index.html

pub mod static_server {
    extern crate env_logger;
    extern crate actix;
    extern crate actix_web;

    use actix_web::{error, middleware, server, App, HttpRequest, fs::NamedFile};

    use error::{Error};
    struct AppState {}

    pub fn run() {
        fn index(_req: &HttpRequest<AppState>) -> Result<NamedFile, Error> {
            Ok(NamedFile::open("static/index.html")?)
        }

        ::std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();


        let sys = actix::System::new("ws-example");

        //move is necessary to give closure below ownership of counter
        server::new(move || {
            App::with_state(AppState{})
                .middleware(middleware::Logger::default())
                .resource("/", |r| r.f(index))
        }).bind("127.0.0.1:8080")
            .unwrap()
            .start();

        println!("Started http server: 127.0.0.1:8080");

        let _ = sys.run();
    }
}