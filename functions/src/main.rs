
fn main() {
    let height:i32 = 5;
    let mut width: i32 = 3;
    let depth:i32 = 2;
    let mut arr : [i32; 3] = [113, 45, 34];

    arr.sort();
    let area = get_area(&mut width, height);
    println!("The area of height {} and width {} is {}", height, width, area);
    let volume = get_volume(&mut width, height, depth);
    println!("The volume of height {} and width {} and depth {} is {}", height, width, depth, volume);
    println!("The array value is {:?}", arr)
}

fn get_volume(width: &mut i32, height: i32, depth: i32) -> i32{
    *width = 89;
    return *width * height * depth;
}

fn get_area(width: &mut i32, height:i32) -> i32{
    *width = 9;
    // println!("The ptr is {} and value is {}", *width);
    return *width * height;
}