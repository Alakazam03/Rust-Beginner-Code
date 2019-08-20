pub fn run(){

    // length is fixed and data is of same type

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);
    println!("{}", numbers[0]);
    numbers.push(7);
    // for i in &numbers{
    //     print!("{}", i);
    // }

    let mut numbers2: [i32; 5] = [1,2,3,4,5];

    numbers2[2] = 20;
    println!("{:?}", numbers2);

    let slice: &[i32] = &numbers[0..2];
    println!("slcie {:?}", slice);


}