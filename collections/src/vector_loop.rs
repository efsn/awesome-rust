pub fn l_o_o_p() {
    let v = vec![100, 23, 57, 31];

    for i in &v {
        println!("{}", i)
    }

    let mut v1 = vec![23, 42, 32, 4];
    for i in &mut v1 {
        *i += 100;
    }
}
