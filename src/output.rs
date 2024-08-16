use crate::query_result::QueryResult;
use color_eyre::Result;

pub fn save_results(results: Vec<QueryResult>) -> Result<()> {
    println!("total of {} results", results.len());

    // save results to file
    Ok(())
}
