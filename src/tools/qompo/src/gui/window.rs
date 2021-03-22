pub struct Window {
  // title: String,
  // width: i32,
  // height: i32,
  // resizable: bool,
  // debug: bool,
  // theme: Theme,
  // style: String,
  // child: Option<Box<dyn Widget>>,
  // menubar: Option<MenuBar>,
  // listener: Option<Box<dyn WindowListener>>,
  // timer: Option<u32>,
  // keys: HashSet<Key>,
}

impl Default for Window {
  fn default() -> Self {
    Self {
      title: "Untitled".to_string(),
      width: 640,
      height: 480,
      resizable: false,
      debug: false,
      theme: Theme::Default,
      style: "".to_string(),
      child: None,
      menubar: None,
      listener: None,
      timer: None,
      keys: HashSet::new(),
    }
  }
}
