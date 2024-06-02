// Topic: Arrays & Slices
//
// Requirements:
// * Print pairs of numbers and their sums as they are streamed from a data source
// * If only one number is received, then print "Unpaired value: V",
//   where V is the value
// * If no numbers are received, print "Data stream complete"
//
// Notes:
// * A simulated data stream is already configured in the code
// * See the stdlib docs for the "chunks" method on "slice" for more info

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

fn process_chunk(some_slice: &[u64]){
    match some_slice {
        [a,b] => {
            println!("pair of numbers: a: {:?}, b: {:?}, sum: {:?}",a,b,a+b);
        },
        [a] => println!("Unpaired value: {:?}",a),
        [] => println!("Data stream complete"),
        [..] => println!("chunk size should be atmost 2"),
    }
}
fn main() {
    // `stream` is an iterator of Option<&[u64]>
    let mut stream = data().chunks(2);
    for chunk in stream{
        process_chunk(chunk);
    }
}
