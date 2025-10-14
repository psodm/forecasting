use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationRow {
    #[serde(alias="Resource ID")]
    resource_id: String,
    #[serde(alias="Resource Full Name")]
    resource_name: String,
    #[serde(alias="Employment Type")]
    employment_type: String,
    #[serde(alias="Resource Manager")]
    resource_manager: String,
    #[serde(alias="Investment ID")]
    investment_id: String,
    #[serde(alias="Investment Name")]
    investment_name: String,
    #[serde(alias="Investment Type")]
    investment_type: String,
    #[serde(alias="Investment Role")]
    investment_role: String,
    #[serde(alias="Investment Manager")]
    investment_manager: String,
    a0: Option<f64>,
    a1: Option<f64>,
    a2: Option<f64>,
    a3: Option<f64>,
    a4: Option<f64>,
    a5: Option<f64>,
    a6: Option<f64>,
    a7: Option<f64>,
    a8: Option<f64>,
    a9: Option<f64>,
    a10: Option<f64>,
    a11: Option<f64>,
    a12: Option<f64>,
    a13: Option<f64>,
}

#[derive(Debug)]
pub struct Allocation {
    resource_id: String,
    resource_name: String,
    resource_manager: String,
    employment_type: String,
    investment_id: String,
    investment_name: String,
    investment_type: String,
    investment_role: String,
    investment_manager: String,
    a0: f64,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    a7: f64,
    a8: f64,
    a9: f64,
    a10: f64,
    a11: f64,
    a12: f64,
    a13: f64,
}

pub fn convert_row_to_allocation(row: AllocationRow) -> Allocation {
    let a = Allocation{
        resource_id: row.resource_id,
        resource_name: row.resource_name,
        resource_manager: row.resource_manager,
        employment_type: row.employment_type,
        investment_id: row.investment_id,
        investment_name: row.investment_name,
        investment_type: row.investment_type,
        investment_role: row.investment_role,
        investment_manager: row.investment_manager,
        a0: some_allocation_to_float(row.a0),
        a1: some_allocation_to_float(row.a1),
        a2: some_allocation_to_float(row.a2),
        a3: some_allocation_to_float(row.a3),
        a4: some_allocation_to_float(row.a4),
        a5: some_allocation_to_float(row.a5),
        a6: some_allocation_to_float(row.a6),
        a7: some_allocation_to_float(row.a7),
        a8: some_allocation_to_float(row.a8),
        a9: some_allocation_to_float(row.a9),
        a10: some_allocation_to_float(row.a10),
        a11: some_allocation_to_float(row.a11),
        a12: some_allocation_to_float(row.a12),
        a13: some_allocation_to_float(row.a13),
    };
    return a;
}

pub fn some_allocation_to_float(value: Option<f64>) -> f64 {
    match value {
        Some(value) => return value,
        None => return 0.0,
    }
}