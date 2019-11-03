fn main() {
    let true_love_str = "My true love gave to me";

    let days: [&'static str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ]; 

    let twelve_days_of_christmas: [&'static str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "12 drummers drumming"
    ];

    for (i, _line) in twelve_days_of_christmas.iter().enumerate() {
        // Inner index
        let mut j = i;

        // Print the language variant of the number
        println!("On the {} day of Christmas", &days[i]);

        // Standard line
        println!("{}",true_love_str);

        loop {
            /* 
            If the current iteration is above 0 and 
            inner index is equal to the first element 
            in the array the prepend "and". Else
            print as normal
                */
            if i > 0 && j == 0 {
                println!("And {}", twelve_days_of_christmas[j]);
            } else {
                println!("{}", twelve_days_of_christmas[j]);
            }

            // Break Loop
            if j == 0 {
                break;
            }

            // Decrement inner index
            j -= 1;
        }
        
        // New line
        println!("");
    }
}