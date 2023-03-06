use sovcore::context;
use sovcore::SovVec;

pub fn to_execute() {
    //read 2 value from host
    let a: u32 = context::read();
    let b: u32 = context::read();
    let c: u32 = context::read();

    let mut vec = SovVec::<3>::new();
    vec.push(a).unwrap();
    vec.push(b).unwrap();
    vec.push(c).unwrap();
    vec.sorted();
    context::commit(&vec.get(0));
    context::commit(&vec.get(1));
    context::commit(&vec.get(2));
}
