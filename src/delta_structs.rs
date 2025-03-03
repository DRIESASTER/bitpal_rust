
//code just contains a strcut for delta H and V so i can keep track easy and get with values instead of indices for clarity
#[derive(Debug, Clone, Copy)]
pub struct DeltaH {
    pub values: [u64; 8],
}
//dH initialization DOES matter, ie all gap penalty (-3 here)
impl DeltaH {
    pub fn new(seq_len: usize) -> Self {
        let mut values = [0; 8];
        let mut neg_3 = 0;
        for i in 0..seq_len {
            neg_3 |= 1 << i;
        }
        values[0] = neg_3;
        DeltaH { values }
    }

    pub fn get(&self, index: i8) -> u64 {
        let uindex = (index + 3) as usize;
        if uindex < 8 {
            self.values[uindex]
        } else {
            panic!("Invalid DeltaH index: {}", index);
        }
    }

    pub fn set(&mut self, index: i8, value: u64) {
        let uindex = (index + 3) as usize;
        if uindex < 8 {
            self.values[uindex] = value;
        } else {
            panic!("Invalid DeltaH index: {}", index);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeltaV {
    pub values: [u64; 8],
}

//initialization does not matter no values are used before calculation
impl DeltaV {
    pub fn new() -> Self {
        DeltaV { values: [0; 8] }
    }

    pub fn get(&self, index: i8) -> u64 {
        let uindex = (index + 3) as usize;
        if uindex < 8 {
            self.values[uindex]
        } else {
            panic!("Invalid DeltaV index: {}", index)
        }
    }

    pub fn set(&mut self, index: i8, value: u64) {
        let uindex = (index + 3) as usize;
        if uindex < 8 {
            self.values[uindex] = value;
        } else {
            panic!("Invalid DeltaV index: {}", index)
        }
    }
}