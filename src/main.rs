use csv::Reader;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::fs::File; // Import File type from std::fs

#[derive(Debug, Deserialize)]
struct CsvRow {
    #[serde(rename = "Display name")]
    display_name: Option<String>,
    #[serde(rename = "Value")]
    value: Option<String>,
    #[serde(rename = "Recommended")]
    recommended: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./snowaudit /path/to/sys_properties.list.csv");
        return;
    }

    let csv_file = &args[1];
    let file = File::open(csv_file).expect("Failed to open file.");
    let mut rdr = Reader::from_reader(file);

    let mut html_output = String::new();
    html_output.push_str("<html><head><title>System Properties Audit</title>");
    html_output.push_str("<style>");
    html_output.push_str("body { font-family: Arial, sans-serif; text-align: center; background-color: #100E16; color: #f2f2f2; }");
    html_output.push_str("table { border-collapse: collapse; width: 80%; margin: auto; }");
    html_output.push_str("th, td { border: 1px solid #f2f2f2; padding: 8px; text-align: left; }");
    html_output.push_str("th { background-color: #000; }");
    html_output.push_str("td { max-width: 300px; white-space: pre-wrap; word-wrap: break-word; }");
    html_output.push_str("img { display: block; margin: 10px auto; width: 500px; }");
    html_output.push_str(".unsupported { display: table-row; }");
    html_output.push_str(".toggle-switch { display: inline-block; width: 40px; height: 20px; background-color: #3498db; border-radius: 10px; position: relative; cursor: pointer; transition: background-color 0.2s; }");
    html_output.push_str(".toggle-switch::before { content: ''; position: absolute; width: 16px; height: 16px; background-color: #fff; border-radius: 50%; top: 2px; left: 2px; transition: 0.2s; }");
    html_output.push_str(".toggle-switch.active { background-color: #4CAF50; }");
    html_output.push_str(".toggle-switch.active::before { transform: translateX(20px); }");
    html_output.push_str("</style>");
    html_output.push_str("<script>");
    html_output.push_str("function toggleUnsupported() {");
    html_output.push_str("  var unsupportedRows = document.getElementsByClassName('unsupported');");
    html_output.push_str("  for (var i = 0; i < unsupportedRows.length; i++) {");
    html_output.push_str("    unsupportedRows[i].style.display = unsupportedRows[i].style.display === 'none' ? 'table-row' : 'none';");
    html_output.push_str("  }");
    html_output.push_str("  var toggleSwitch = document.getElementById('toggle-switch');");
    html_output.push_str("  toggleSwitch.classList.toggle('active');");
    html_output.push_str("}");
    html_output.push_str("</script>");
    html_output.push_str("</head><body>");
    html_output.push_str("<img width=\"600\" src=\"https://i.postimg.cc/6qyYZ4DM/snowy-snowauditv4.png\">");
    html_output.push_str("<div class=\"toggle-switch\" id=\"toggle-switch\" onclick=\"toggleUnsupported()\"></div>");
    html_output.push_str("<p>Hide/Show Unsupported Definitions</p>");
    html_output.push_str("<p></p><p></p>");
    html_output.push_str("<table>");
    html_output.push_str("<tr><th>DEFINITION</th><th>CURRENT</th><th>RECOMMENDED</th><th>DESCRIPTION</th></tr>");

    // best security practice values go here
    // These values should be added in the form of an array of strings, each representing a row
    // in the CSV table. Each array element should contain four elements in the following format:
    // ["Display Name", "Value", "Recommended", "Description"]
    //
    // - "Display Name": The name of the system property or configuration.
    // - "Value": The current value of the system property or configuration.
    // - "Recommended": The recommended value for the system property or configuration based on
    //                  best security practices.
    // - "Description": A brief description or explanation of the system property or configuration.
    let best_security_practice_values = vec![
        ["glide.security.csrf.strict.validation.mode", "false", "true", "Enforces CSRF token strict validation that does not allow resubmit the request if CSRF token does not match."],
        ["glide.security.csrf_previous.allow", "false", "true", "Allow usage of an expired secure token to identify and validate incoming requests.  This token is used to prevent CSRF attacks."]
        // add more here, integrate support for ALL security related service now definitions.
    ];

    for result in rdr.deserialize::<CsvRow>() {
        match result {
            Ok(row) => {
                let display_name = row.display_name.unwrap_or_else(|| "".to_string());
                let value = row.value.as_ref().map_or_else(|| "".to_string(), |v| v.to_string());
                let mut recommended_value = row.recommended.as_ref().map_or_else(|| "UNSUPPORTED".to_string(), |v| v.to_string());
                let description = row.description.unwrap_or_else(|| "".to_string());

                for best_value in &best_security_practice_values {
                    let display_name_best = best_value[0];
                    let recommended_value_best = best_value[2];

                    if display_name == display_name_best {
                        recommended_value = recommended_value_best.to_string();
                        break;
                    }
                }

                // Add a CSS class 'unsupported' to rows with unsupported values
                let css_class = if recommended_value == "UNSUPPORTED" { " class=\"unsupported\"" } else { "" };

                html_output.push_str("<tr");
                html_output.push_str(css_class);
                html_output.push_str(">");
                html_output.push_str(&format!(
                    "<td>{}</td><td>{}</td><td>{}</td><td>{}</td>",
                    encode_text_minimal(&display_name),
                    encode_text_minimal(&value),
                    encode_text_minimal(&recommended_value),
                    encode_text_minimal(&description)
                ));
                html_output.push_str("</tr>");
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    html_output.push_str("</table>");
    html_output.push_str("</body></html>");

    // Save HTML output to a file
    let output_file = Path::new("snowaudit_report.html");
    if let Err(error) = fs::write(output_file, html_output) {
        // Handle the error if writing fails
        eprintln!("Failed to write HTML output to file: {}", error);
    } else {
        // Handle the success case
        println!("Success. Navigate to the snowaudit_report.html export for results.");
    }
}

// Function to encode text to HTML entities to prevent HTML injection.
fn encode_text_minimal(text: &str) -> String {
    html_escape::encode_text_minimal(text).into_owned()
}
