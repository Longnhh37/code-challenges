struct ParkingSystem {
    avail: [i32; 3]
}

impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            avail: [big, medium, small],
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        let i = match car_type {
            1 => 0,
            2 => 1,
            _ => 2,
        };
        if self.avail[i] > 0 {
            self.avail[i] -= 1;
            true
        } else {
            false
        }
        
    }
}

