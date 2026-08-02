use markitdown::MarkItDown;
use std::{env, fs};

fn main() {
    // Project path and output path
    let project_path = env::args().nth(1).unwrap_or_else(|| {
        "../sources/ASWE Module 7 Project (April update).pdf".to_string()
    });

    let project_output_path = env::args().nth(2).unwrap_or_else(|| {
        "../sources/project_output.md".to_string()
    });

    // Assessment guide path and output path
    let assesment_guide_path = env::args().nth(3).unwrap_or_else(|| {
        "../sources/ASWE_Module_7.pdf".to_string()
    });

    let assesment_guide_output_path = env::args().nth(4).unwrap_or_else(|| {
        "../sources/assesment_guide_output.md".to_string()
    });

    let md = MarkItDown::new();
    match md.convert(&project_path, None) {
        Some(result) => {
            match fs::write(&project_output_path, &result.text_content) {
                Ok(_) => println!("Successfully wrote markdown to {}", project_output_path),
                Err(e) => eprintln!("Error writing file: {}", e),
            }
        },
        None => eprintln!("Error converting PDF"),
    }

    match md.convert(&assesment_guide_path, None) {
        Some(result) => {
            match fs::write(&assesment_guide_output_path, &result.text_content) {
                Ok(_) => println!("Successfully wrote markdown to {}", assesment_guide_output_path),
                Err(e) => eprintln!("Error writing file: {}", e),
            }
        },
        None => eprintln!("Error converting PDF"),
    }
}
