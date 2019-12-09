
fn create_memory(data: String) -> Vec<u32> {
    data.split(",").map(|x| x.parse().unwrap()).collect()
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_memory() {
        let mem = create_memory("1,9,10,3,2,3,11,0,99,30,40,50".to_string());
        assert_eq!(mem.len(), 12 as usize);
        assert_eq!(mem[3], 3);
    }
}
