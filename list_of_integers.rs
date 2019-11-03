/*
Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn main() {
  let mut list_of_ints = vec![50, 50, 20, 23, 19, 50, 48, 13, 39, 8];
  let mut list_of_ints_odd = vec![50, 26, 20, 23, 19, 5, 48, 13, 8, 8, 9];

  println!("EVEN: Mean: {}, Median: {}, Mode: {:#?}", find_mean(&list_of_ints), find_median(&mut list_of_ints), find_mode(&list_of_ints));
  println!("\nODD: Mean: {}, Median: {}, Mode: {:#?}", find_mean(&list_of_ints_odd), find_median(&mut list_of_ints_odd), find_mode(&list_of_ints_odd));
}

fn find_mode (list: &Vec<i32>) -> Vec<i32> {
    // New hashmap
    let mut map = HashMap::new();
    let max_value;

    // Iterate over the list
    for number in list.iter() {
      // Returns a referenced value increment or insert 0
      let count = map.entry(number).or_insert(0);
      *count += 1;
    }

    /* 
    Return the values of the hashmap. Clone the values and return the max value
    max may return none so we have a fallback of 0 if there is no value
    */
    max_value = map.values().cloned().max().unwrap_or(0);

    /* 
    Turn the value into a iterator
    use filter against the key:value and only return the values
    that match the max value. Map over the iterators and turn the iterators
    into a new collection
    */
    return map.into_iter()
      .filter(|&(_k,v)| v == max_value)
      .map(|(&k,_v)| k)
      .collect();
}

fn find_mean (list: &Vec<i32>) -> f32 {
  // Calculate the sum
  let sum: i32 = list.iter().sum();

  // Return the mean
  return sum as f32 / list.len() as f32;
}

fn find_median (list: &mut Vec<i32>) -> i32 {
  // Return mid
  let mid = list.len() / 2;
  let result;

  // Sort
  list.sort();

  // Even
  if list.len() % 2 == 0 {
    /* 
    Create a new referenced vector with the mid offset and mid
    */
    result = find_mean(&vec![list[mid - 1], list[mid]]) as i32;
  // Odd
  } else {
    // Return mid
    result = list[mid];
  }

  return result;
}