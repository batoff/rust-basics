trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
}

impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index);
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |= 1 << index;
    }
}

fn main() {
    let mut num = 0;
    num.set(5);
    println!("{num}");

    println!("{}", num.is_set(5));
    num.clear(5);
    println!("{num}");
}
