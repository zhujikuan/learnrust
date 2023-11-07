trait Vehicle {
    fn get_price(&self)->u64;
}

trait Car: Vehicle {
    fn get_model(&self)->String;
}

struct TeslaRoadster {
    model: String,
    year: u32,
}
impl TeslaRoadster {
    fn new(model: String, year: u32) -> Self {
        Self {
            model,
            year,
        }
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200000
    }
}
impl Car for TeslaRoadster {
    fn get_model(&self) -> String {
        self.model.clone()
    }
}

fn main(){
    let tesla = TeslaRoadster::new("Roadster".to_string(), 2020);
    println!("Tesla Roadster: {}", tesla.get_model());
}