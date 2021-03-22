// cmd: build, dev, start
// args: web, webview
use web_view::*;

pub struct Arg {
  args: Vec<String>,
}

impl Arg {
  pub fn new (args: &Vec<String>) -> Self {
    Arg {
      args: args.to_owned(),
    }
  }
}

pub struct Cmd {
  arg: Arg,
}

impl Cmd {
  pub fn new (arg: Arg) -> Self {
    Cmd { arg }
  }

  pub fn run (&self) {
    println!("running..");
    build_webview();
  }
}

fn main () -> Result<(), String> {
  let args = std::env::args().skip(1).collect::<Vec<_>>();

  Cmd::new(Arg::new(&args)).run();

  Ok(())
}

fn build_webview () {
  let dom = "<html><body><h1>Hello, World!</h1></body></html>";
  // let url = "http://localhost:8080";

  web_view::builder()
    .title("test")
    // .content(Content::Html(dom))
    .content(Content::Url(url))
    .size(320, 480)
    .resizable(false)
    .debug(true)
    .user_data(())
    .invoke_handler(|_webview, _arg| Ok(()))
    .run()
    .unwrap();
}
