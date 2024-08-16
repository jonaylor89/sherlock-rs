use crate::query_result::QueryResult;
use color_eyre::Result;

pub fn save_results(results: Vec<QueryResult>) -> Result<()> {
    for result in results {
        println!("{:?}", result);
    }

    // save results to file
    Ok(())
}
