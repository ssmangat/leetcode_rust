pub struct FlowerBed {}

impl FlowerBed {
    pub fn can_place_flowers(bed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let bed_len = bed.len();
        let mut flower_bed = bed.clone();
        let mut number = n;
        for i in 0..bed_len {
            if flower_bed[i] == 0
                && (i == 0 || flower_bed[i - 1] == 0)
                && (i == bed_len - 1 || flower_bed[i + 1] == 0)
            {
                flower_bed[i] = 1;
                number -= 1;
                if number == 0 {
                    return true;
                }
            }
        }
        false
    }
}
