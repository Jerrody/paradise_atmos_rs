mod gas_mixture;

use gas_mixture::*;

use auxtools::*;

#[hook("/datum/gas_mixture/proc/change_turf")]
fn change_turf(self_datum: Value, turf: Value) {
    let carbon_dioxide_name = StringRef::new("carbon_dioxide").unwrap();
    let oxygen_name = StringRef::new("oxygen").unwrap();
    let carbon_dioxide = self_datum.get_number(carbon_dioxide_name.clone()).unwrap();
    turf.set(oxygen_name, carbon_dioxide + 228.0).unwrap();
    self_datum.set(carbon_dioxide_name, 1337).unwrap();

    Ok(Value::null())
}
