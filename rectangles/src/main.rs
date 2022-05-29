struct Rectangle {
    width: u32,
    height: u32,
} //Using struct instead

fn main() {
    //let width1 = 30;
    //let height1 = 50;
    //change from two parameters to singular tuple parameter
    //let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };//Initializing the struct
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)//Send location value to continue maintenance of ownership

    );
    
}

//For multiple parameters
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

//For single tuple
fn area(rectangle: &rectangle) -> u32 {
    rectangle.width * rectangle.height
}

