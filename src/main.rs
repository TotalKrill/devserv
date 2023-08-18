use std::path::PathBuf;

use rocket::{
    fairing::{Fairing, Info, Kind},
    fs::FileServer,
    http::{Header, HeaderMap},
    Request, Response,
};

use clap::Parser;

#[derive(Parser)]
struct Opt {
    #[arg(long, short, default_value = ".")]
    path: PathBuf,
    #[arg(long, num_args = 0..)]
    header: Vec<String>,
}

pub struct HeaderFairing {
    headers: Vec<String>,
}

#[rocket::async_trait]
impl Fairing for HeaderFairing {
    fn info(&self) -> Info {
        Info {
            name: "HeaderFairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        for header in &self.headers {
            let mut split = header.split("=");
            let key = split.next().unwrap().to_string();
            let val = split.next().unwrap().to_string();
            let header = Header::new(key, val);
            res.adjoin_header(header);
        }
    }
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let r = rocket::build()
        .attach(HeaderFairing {
            headers: vec![
                "Cross-Origin-Opener-Policy=same-origin".to_string(),
                "Cross-Origin-Embedder-Policy=require-corp".to_string(),
            ],
        })
        .mount(
            "/",
            FileServer::from(opt.path.canonicalize().expect("check your path variable")),
        );
    r.launch().await.ok();
}
