

#[allow(dead_code)]
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_into_iter = v1.into_iter();
    for v1 in v1_into_iter {
        println!("{}", v1);
    }
    // will get error
    // println!("{:?}", v1);
    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    for v2 in v2_iter {
        // error
       //  *v2 = 1;
        println!("{}", v2);
    }

    let mut v3 = vec![1, 2, 3];
    let v3_iter = v3.iter_mut();
    for v3 in v3_iter {
        *v3 = 1;
        println!("{}", v3);
    }
    assert_eq!(v3, vec![1, 1, 1]);
}