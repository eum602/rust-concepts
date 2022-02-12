#![allow(dead_code, unused_variables)]

use compoundTypes::{on_off,print_difference,print_array,ding, print_distance};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    print_difference(coords.0, coords.1);


    let arr:[f32; 2] = [coords.0,coords.1];
    print_array(arr);


    let series = [1, 1, 2, 3, 5, 8, 13];

    ding(series[6]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    on_off(mess.2[1].0);

    // `cargo run` should produce the same output, only now the code is more organized. 🎉

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}