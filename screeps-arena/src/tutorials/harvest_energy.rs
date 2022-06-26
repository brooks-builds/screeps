struct Data {}

impl Data {
    pub fn init() -> Self {
        Self {}
    }
}

#[allow(dead_code)]
pub fn run() {
    let _data = Data::init();
}
