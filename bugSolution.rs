fn main() {
    let mut v = vec![1, 2, 3];
    let v_clone = v.clone();
    let ptr = v_clone.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!("The first element is: {}", v_clone[0]);
    println!("The original vector: {:?}", v);
}