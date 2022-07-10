
fn main() {
    // Creating a New Vector
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    println!("{:?} {:?}",v1, v2);

    // Updating a Vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("{:?}",v3);

    // Dropping a Vector Drops Its Elements
    {
        let v4 = vec![1, 2, 3, 4];
        println!("{:?}",v4);
        // do stuff with v4
    } // <- v4 goes out of scope and is freed here

    // Reading Elements of Vectors
    let v5 = vec![1, 2, 3, 4, 5];
    // via indexing -- not safe may panic
    let third: &i32 = &v5[2];
    println!("The third element is {}", third);

    // via get -- safe wont panic
    match v5.get(2) {
        Some(element) => println!("The third element is {}", element),
        None => println!("There is no third element."),
    }

    // Iterating over the Values in a Vector - immutable
    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("{}", i);
    }

    // Iterating over the Values in a Vector - mutable & updating values
    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        *i += 50;
        println!("{}", i);
    }

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

}
