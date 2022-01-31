fn main() {
    let mut x =5; //by default it is unmutable
    println!("The value of x is: {}",x);
    x = 6; // to declare a var as mut/not mut a fucking life saver, in bigger codes, it is insane to
    //expect to remember all such declarations
    println!("The value of x is: {}",x);
    //in cases where you’re using large data structures, 
    //mutating an instance in place may be faster than copying 
    //and returning newly allocated instances. 
    //With smaller data structures, creating new instances and writing 
    //in a more functional programming style may be easier to think through, 
    //so lower performance might be a worthwhile penalty for gaining that clarity.
    
    //There is also declaration as a constant - permanently immutable.

    let y=5; //bind

    let y = y + 1; //shadow y=6, effectively new var

    {//inner scope
        let y = y*2; //inner shadow y=12
        println!("The value of y in the inner scope is: {}",y);
    }//end of inner scope
    println!("The value of y is {}",y); //y returns to 6!

    let spaces = "   ";
    let _spaces = spaces.len();
}
