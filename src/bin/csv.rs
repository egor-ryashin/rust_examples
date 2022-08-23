use std::error::Error;
use csv::Reader;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("math-tests.csv")?;

    let mut headers : Vec<&str> = Vec::new();
    for h in rdr.headers()?.iter() {
        headers.push(h);
    }
    let mut table = Table::new();
    table
        .set_header(headers)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .column_mut(2).expect("Our table has three columns")
        .set_cell_alignment(CellAlignment::Right);

    let mut k = 0;
    for result in rdr.records() {
        k+=1;
        if k > 10 {
            break;
        }
        let record = result?;
        let mut r = Row::new();
        for v in record.iter() {
            r.add_cell(Cell::new(v));
        }
        table.add_row(r);
    }
    println!("{table}");
    Ok(())
}

fn main() {
    example();
}
