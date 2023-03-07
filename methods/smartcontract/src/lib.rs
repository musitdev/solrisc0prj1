use serde::{Deserialize, Serialize};
use sovcore::context;
use sovcore::SovMap;
use sovcore::SovVec;

pub fn to_execute() {
    //vec initial example
    let a: u32 = context::read();
    let b: u32 = context::read();
    let c: u32 = context::read();

    let mut vec = SovVec::<3>::new();
    vec.push(a).unwrap();
    vec.push(b).unwrap();
    vec.push(c).unwrap();
    let vec = vec.sorted();

    let mut smap = SovMap::new();

    smap.insert(2, 200);
    smap.insert(3, 300);

    assert_eq!(smap.get(1), None);
    assert_eq!(smap.get(2).unwrap(), 200);
    assert_eq!(smap.get(4), None);
    assert_eq!(smap.get(5), None);

    smap.insert(4, 400);
    smap.insert(5, 500);

    assert_eq!(smap.get(4).unwrap(), 400);
    assert_eq!(smap.get(5).unwrap(), 500);

    smap.insert(6, 400);
    smap.insert(7, 400);
    smap.insert(8, 400);

    assert_eq!(smap.get(9), None);
    assert_eq!(smap.get(10), None);
    assert_eq!(smap.get(11), None);
    assert_eq!(smap.get(2).unwrap(), 200);
    assert_eq!(smap.get(2).unwrap(), 200);
    assert_eq!(smap.get(2).unwrap(), 200);

    smap.insert(9, 900);

    assert_eq!(smap.get(9).unwrap(), 900);
    assert_eq!(smap.get(10), None);
    assert_eq!(smap.get(10), None);

    let commit = Commit {
        sorted_vec: [
            *vec.get(0).unwrap(),
            *vec.get(1).unwrap(),
            *vec.get(2).unwrap(),
        ],
        hashtable: [
            smap.get(0).unwrap_or(0),
            smap.get(1).unwrap_or(0),
            smap.get(2).unwrap_or(0),
            smap.get(3).unwrap_or(0),
            smap.get(4).unwrap_or(0),
            smap.get(5).unwrap_or(0),
            smap.get(6).unwrap_or(0),
            smap.get(7).unwrap_or(0),
            smap.get(8).unwrap_or(0),
            smap.get(9).unwrap_or(0),
        ],
    };

    context::commit(&commit);
}

#[derive(Debug, Serialize, Deserialize)]
struct Commit {
    sorted_vec: [u32; 3],
    hashtable: [u32; 10], //none is 0.
}
