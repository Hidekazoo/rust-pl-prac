use polars::prelude::*;

fn main() {
   let df = df![
       "names" => ["a","b","c"],
       "valus" => [1,2,3],
       "values_null"=> [Some(1), None, None]
   ];
    println!("{:?}", df.ok());
}
