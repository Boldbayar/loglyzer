use crate::cli::cli_args::OutputType;
use crate::utils::util_file_writer::{AnalysisResult, save_result};

pub async fn handle_output(
    mode: &OutputType, analysis: &AnalysisResult,
) -> Result<(), Box<dyn std::error::Error>> {
    match mode {
        OutputType::Console => {
            println!("{}", analysis.ai_summary);
        }
        OutputType::File => {
            save_result(analysis)?;
        }
        OutputType::Grafana => {
            // TODO: Implement Grafana output
        }
    }
    Ok(())
}
