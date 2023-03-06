use sovcore::context;
use sovcore::SovVec;

pub fn to_execute() {
    //read 2 value from host
    let a: u32 = context::read();
    let b: u32 = context::read();

    let mut vec = SovVec::<2>::new();
    vec.push(a).unwrap();
    vec.push(b).unwrap();
    vec.sorted();
}
