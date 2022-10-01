use csv::Writer;
use quick_xml::{de, reader::Reader};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Item {
    item_status: i32,
    allow_discount: bool,
    unit_of_measure_price: f32,
    item_price: f32,
    qty_in_package: i32,
    unit_of_measure: String,
    #[serde(rename = "bIsWeighted")]
    b_is_weighted: bool,
    quantity: f32,
    unit_qty: String,
    manufacturer_item_description: String,
    manufacture_country: String,
    manufacturer_name: String,
    item_name: String,
    item_type: i32,
    item_code: String,
    price_update_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Items {
    item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Branch {
    chain_id: String,
    sub_chain_id: String,
    store_id: String,
    items: Items,
}

#[derive(Serialize, Deserialize, Debug)]
struct BranchRow {
    chain_id: String,
    sub_chain_id: String,
    store_id: String,
    item_status: i32,
    allow_discount: bool,
    price: f32,
    unit_of_measure: String,
    unit_of_measure_price: f32,
    is_weighted: bool,
    quantity: f32,
    quantity_unit: String,
    quantity_in_package: i32,
    manufacturer_item_description: String,
    manufacture_country: String,
    manufacturer_name: String,
    name: String,
    item_type: i32,
    code: String,
    price_date: String,
}

fn main() {
    let csv_writer = Writer::from_path("./output/output.csv");
    match csv_writer {
        Err(err) => println!("Failed creating CSV writer: {:?}", err),
        Ok(mut csvw) => match Reader::from_file("./data/file.xml") {
            Err(err) => println!("Failed reading from file: {:?}", err),
            Ok(reader) => {
                let branch: Branch = de::from_reader(reader.into_inner()).unwrap();
                for item in branch.items.item {
                    let branch_row = BranchRow {
                        chain_id: branch.chain_id.clone(),
                        sub_chain_id: branch.sub_chain_id.clone(),
                        store_id: branch.store_id.clone(),
                        item_status: item.item_status,
                        allow_discount: item.allow_discount,
                        price: item.item_price,
                        unit_of_measure: item.unit_of_measure.clone(),
                        unit_of_measure_price: item.unit_of_measure_price,
                        is_weighted: item.b_is_weighted,
                        quantity: item.quantity,
                        quantity_unit: item.unit_qty,
                        quantity_in_package: item.qty_in_package,
                        manufacturer_item_description: item.manufacturer_item_description.clone(),
                        manufacture_country: item.manufacture_country,
                        manufacturer_name: item.manufacturer_name,
                        name: item.item_name.clone(),
                        item_type: item.item_type,
                        code: item.item_code.clone(),
                        price_date: item.price_update_date.clone(),
                    };
                    // rows.push(branch_row);
                    match csvw.serialize(branch_row) {
                        Err(err) => println!("Failure writing line {:?}: {:?}", item.item_name, err),
                        Ok(_) => {},
                    }
                }
            }
        },
    }
}
