use ndarray::arr2;

pub fn mat() {
    let a = arr2(&[[1,2,3],[4,5,6]]);
    
    let b = arr2(&[[-1,2,3],
                    [4,-5,6]]);

    let sum = &a + &b;
    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!{"="};
    println!("{}", sum);

}