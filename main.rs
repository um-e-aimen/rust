// Function to concatenate two string slices and return a new String
fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new(); // Create a new String to store the concatenated result
    result.push_str(str1); // Append the first string slice to the result
    result.push_str(str2); // Append the second string slice to the result
    result // Return the concatenated string
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the concatenated result
    println!("{}", concatenated_string);
}
