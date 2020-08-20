fn main() {
    // This code block will panic.
    // assert!(0.1 + 0.2 == 0.3);

    let result: f32 = 0.1 + 0.1;
    println!("result is {}", result);

    let desired: f32 = 0.2;
    println!("desired is {}", desired);

    let absolute_difference = (desired - result).abs();
    println!("absolute_difference is {}", absolute_difference);
    println!("f32::EPSILON is {}", f32::EPSILON);

    assert!(absolute_difference <= f32::EPSILON);

    assert_eq!(f32::NAN == f32::NAN, false);
}
