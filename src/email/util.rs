pub mod body_parsing;
pub mod header_parsing;
pub mod mapping_to_extern_crates;

pub fn gen_csv_from_lower(lower_bound: i32, upper_bound: i32) -> String {
    let mut returned_string = "".to_string();

    for i in lower_bound..upper_bound {
        returned_string = returned_string + &i.to_string();
    }
    returned_string
}
