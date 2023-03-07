use sovcore::context;
// use sovcore::SovVec;
use sovcore::SovMap;

pub fn to_execute() {

    let mut smap = SovMap::new();

    smap.insert(2,200);
    smap.insert(3,300);

    assert_eq!(smap.get(1), None);
    assert_eq!(*smap.get(2).unwrap(), 200);
    assert_eq!(smap.get(4), None);
    assert_eq!(smap.get(5), None);

    smap.insert(4,400);
    smap.insert(5,500);

    assert_eq!(*smap.get(4).unwrap(), 400);
    assert_eq!(*smap.get(5).unwrap(), 500);

    smap.insert(6,400);
    smap.insert(7,400);
    smap.insert(8,400);

    assert_eq!(smap.get(9), None);
    assert_eq!(smap.get(10), None);
    assert_eq!(smap.get(11), None);
    assert_eq!(*smap.get(2).unwrap(), 200);
    assert_eq!(*smap.get(2).unwrap(), 200);
    assert_eq!(*smap.get(2).unwrap(), 200);

    smap.insert(9,900);

    assert_eq!(*smap.get(9).unwrap(), 900);
    assert_eq!(smap.get(10), None);
    assert_eq!(smap.get(10), None);

}
