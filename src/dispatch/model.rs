use std::fs::File;
use std::path::PathBuf;
use lifeguard::Recycleable;

#[derive(Default, Debug)]
pub struct Model {
    file: PathBuf
}

impl Model {
    pub fn new() -> Self {
        Model { ..Default::default() }
    }
}

impl Recycleable for Model {
    fn new() -> Self {
        Model::new()
    }

    fn reset(&mut self) {
        ()
    }
}
