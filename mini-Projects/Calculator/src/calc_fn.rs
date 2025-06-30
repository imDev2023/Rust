pub mod calculator {

    
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
    pub fn sub(a: u32, b: u32) -> u32 {
        if a > b {
            return a - b;
        }
        return b - a;
    }
    pub fn mul(a: u32, b: u32) -> u32 {
        a * b
    }
    pub fn div(a: u32, b: u32) -> Option<f64> {
        if b == 0 {
            return None;
        }
        Some((a as f64) / (b as f64))
    }
    pub fn square_root(a: u32) -> f64 {
        (a as f64).sqrt()
    }
}