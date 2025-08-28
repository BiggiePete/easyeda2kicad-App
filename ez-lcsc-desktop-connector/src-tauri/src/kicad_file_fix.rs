use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum KiCadFixerError {
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Walkdir error: {0}")]
    WalkdirError(#[from] walkdir::Error),
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),
}

#[derive(Debug, Clone)]
pub struct SymbolIssue {
    pub issue_type: IssueType,
    pub description: String,
    pub line_number: Option<usize>,
    pub severity: Severity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    MissingHeader,
    UnbalancedParentheses,
    MissingProperties,
    InvalidCoordinates,
    DuplicatePinNumbers,
    MissingPinInfo,
    MissingUnitSpec,
    InvalidSyntax,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

pub struct KiCadSymbolFixer {
    pub verbose: bool,
}

impl KiCadSymbolFixer {
    pub fn new() -> Self {
        Self { verbose: false }
    }

    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Analyze a KiCad symbol file and detect issues
    pub fn analyze_file(&self, content: &str) -> Result<Vec<SymbolIssue>, KiCadFixerError> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Check for missing KiCad symbol library header
        if !content.contains("(kicad_symbol_lib") && !content.contains("(symbol") {
            issues.push(SymbolIssue {
                issue_type: IssueType::MissingHeader,
                description: "Missing KiCad symbol library header".to_string(),
                line_number: Some(1),
                severity: Severity::Error,
            });
        }

        // Check for unbalanced parentheses
        let open_parens = content.matches('(').count();
        let close_parens = content.matches(')').count();
        if open_parens != close_parens {
            issues.push(SymbolIssue {
                issue_type: IssueType::UnbalancedParentheses,
                description: format!(
                    "Unbalanced parentheses: {} opening, {} closing",
                    open_parens, close_parens
                ),
                line_number: None,
                severity: Severity::Error,
            });
        }

        // Check for missing required properties
        if !content.contains(r#"(property "Reference""#) {
            issues.push(SymbolIssue {
                issue_type: IssueType::MissingProperties,
                description: "Missing Reference property".to_string(),
                line_number: None,
                severity: Severity::Warning,
            });
        }

        if !content.contains(r#"(property "Value""#) {
            issues.push(SymbolIssue {
                issue_type: IssueType::MissingProperties,
                description: "Missing Value property".to_string(),
                line_number: None,
                severity: Severity::Warning,
            });
        }

        // Check for duplicate pin numbers
        let pin_regex = Regex::new(r#"\(number\s+"([^"]+)""#)?;
        let mut pin_numbers = HashSet::new();
        for capture in pin_regex.captures_iter(content) {
            let pin_num = &capture[1];
            if !pin_numbers.insert(pin_num.to_string()) {
                issues.push(SymbolIssue {
                    issue_type: IssueType::DuplicatePinNumbers,
                    description: format!("Duplicate pin number: {}", pin_num),
                    line_number: None,
                    severity: Severity::Error,
                });
            }
        }

        // Check for invalid coordinates
        let coord_regex = Regex::new(r"at\s+(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)")?;
        for (line_idx, line) in lines.iter().enumerate() {
            for capture in coord_regex.captures_iter(line) {
                let x: f64 = capture[1].parse().unwrap_or(0.0);
                let y: f64 = capture[2].parse().unwrap_or(0.0);
                if x.abs() > 100.0 || y.abs() > 100.0 {
                    issues.push(SymbolIssue {
                        issue_type: IssueType::InvalidCoordinates,
                        description: format!(
                            "Coordinates may be outside typical range: {} {}",
                            x, y
                        ),
                        line_number: Some(line_idx + 1),
                        severity: Severity::Warning,
                    });
                }
            }
        }

        // Check for missing unit specification
        if content.contains("(symbol ") && !content.contains("(unit ") {
            issues.push(SymbolIssue {
                issue_type: IssueType::MissingUnitSpec,
                description: "Symbol may be missing unit specification".to_string(),
                line_number: None,
                severity: Severity::Warning,
            });
        }

        // Check for missing pin information
        let pin_matches = Regex::new(r"\(pin\s+\w+\s+\w+")?;
        for (idx, mat) in pin_matches.find_iter(content).enumerate() {
            let pin_context = &content[mat.start()..];
            if !pin_context.contains("at ") || !pin_context.contains("length ") {
                issues.push(SymbolIssue {
                    issue_type: IssueType::MissingPinInfo,
                    description: format!(
                        "Pin {} may be missing position or length information",
                        idx + 1
                    ),
                    line_number: None,
                    severity: Severity::Warning,
                });
            }
        }

        Ok(issues)
    }

    /// Fix issues in a KiCad symbol file
    pub fn fix_content(
        &self,
        content: &str,
        issues: &[SymbolIssue],
    ) -> Result<String, KiCadFixerError> {
        let mut fixed_content = content.to_string();

        for issue in issues {
            match issue.issue_type {
                IssueType::MissingHeader => {
                    if !fixed_content.starts_with("(kicad_symbol_lib") {
                        fixed_content = format!(
                            "(kicad_symbol_lib (version 20220914) (generator kicad_symbol_editor)\n{}\n)",
                            fixed_content
                        );
                        if self.verbose {
                            println!("Added KiCad symbol library header");
                        }
                    }
                }
                IssueType::UnbalancedParentheses => {
                    let open_count = fixed_content.matches('(').count();
                    let close_count = fixed_content.matches(')').count();
                    if open_count > close_count {
                        fixed_content.push_str(&")".repeat(open_count - close_count));
                        if self.verbose {
                            println!(
                                "Balanced parentheses by adding {} closing parentheses",
                                open_count - close_count
                            );
                        }
                    }
                }
                IssueType::MissingProperties => {
                    if issue.description.contains("Reference")
                        && !fixed_content.contains(r#"(property "Reference""#)
                    {
                        // Find a good place to insert the reference property
                        if let Some(symbol_pos) = fixed_content.find("(symbol ") {
                            if let Some(end_pos) = fixed_content[symbol_pos..].find(')') {
                                let insert_pos = symbol_pos + end_pos;
                                let reference_prop = r#"
    (property "Reference" "U" (at 0.0 0.0 0) 
      (effects (font (size 1.27 1.27)))
    )"#;
                                fixed_content.insert_str(insert_pos, reference_prop);
                                if self.verbose {
                                    println!("Added Reference property");
                                }
                            }
                        }
                    }

                    if issue.description.contains("Value")
                        && !fixed_content.contains(r#"(property "Value""#)
                    {
                        if let Some(ref_pos) = fixed_content.find(r#"(property "Reference""#) {
                            if let Some(end_pos) = fixed_content[ref_pos..].find(')') {
                                let insert_pos = ref_pos + end_pos + 1;
                                let value_prop = r#"
    (property "Value" "VALUE" (at 0.0 -2.54 0) 
      (effects (font (size 1.27 1.27)))
    )"#;
                                fixed_content.insert_str(insert_pos, value_prop);
                                if self.verbose {
                                    println!("Added Value property");
                                }
                            }
                        }
                    }
                }
                IssueType::MissingUnitSpec => {
                    if !fixed_content.contains("(unit ") && fixed_content.contains("(symbol ") {
                        // This is a complex fix that would require more sophisticated parsing
                        // For now, just log it
                        if self.verbose {
                            println!(
                                "Warning: Missing unit specification detected but not auto-fixed"
                            );
                        }
                    }
                }
                _ => {
                    // For other issues, just log them
                    if self.verbose {
                        println!("Issue detected but not auto-fixed: {}", issue.description);
                    }
                }
            }
        }

        // Format the content
        fixed_content = self.format_content(&fixed_content);

        Ok(fixed_content)
    }

    /// Format KiCad file content with proper indentation
    fn format_content(&self, content: &str) -> String {
        let mut formatted_lines = Vec::new();
        let mut indent_level: usize = 0;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted_lines.push(String::new());
                continue;
            }

            // Decrease indent for closing parentheses
            if trimmed.starts_with(')') {
                indent_level = indent_level.saturating_sub(1);
            }

            // Add indented line
            let indented = format!("{}{}", "  ".repeat(indent_level), trimmed);
            formatted_lines.push(indented);

            // Increase indent for opening parentheses (if line doesn't end with closing)
            if trimmed.starts_with('(') && !trimmed.ends_with(')') {
                indent_level += 1;
            }
        }

        // Remove excessive blank lines
        let formatted = formatted_lines.join("\n");
        let excessive_newlines = Regex::new(r"\n\s*\n\s*\n").unwrap();
        excessive_newlines
            .replace_all(&formatted, "\n\n")
            .to_string()
    }

    /// Process a single file
    pub fn fix_file<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<(Vec<SymbolIssue>, bool), KiCadFixerError> {
        let path = file_path.as_ref();
        let content = fs::read_to_string(path)?;

        if self.verbose {
            println!("Processing file: {}", path.display());
        }

        let issues = self.analyze_file(&content)?;

        if issues.is_empty() {
            if self.verbose {
                println!("No issues found in {}", path.display());
            }
            return Ok((issues, false));
        }

        let fixed_content = self.fix_content(&content, &issues)?;

        // Only write if content actually changed
        if fixed_content != content {
            fs::write(path, fixed_content)?;
            if self.verbose {
                println!("Fixed {} issues in {}", issues.len(), path.display());
            }
            Ok((issues, true))
        } else {
            if self.verbose {
                println!("No changes needed for {}", path.display());
            }
            Ok((issues, false))
        }
    }

    /// Process all .kicad_sym files in a directory
    pub fn fix_directory<P: AsRef<Path>>(
        &self,
        dir_path: P,
    ) -> Result<(usize, usize), KiCadFixerError> {
        let mut files_processed = 0;
        let mut files_fixed = 0;

        for entry in WalkDir::new(dir_path) {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("kicad_sym") {
                match self.fix_file(path) {
                    Ok((issues, was_fixed)) => {
                        files_processed += 1;
                        if was_fixed {
                            files_fixed += 1;
                        }
                        if self.verbose && !issues.is_empty() {
                            for issue in issues {
                                println!(
                                    "  - {}: {}",
                                    match issue.severity {
                                        Severity::Error => "ERROR",
                                        Severity::Warning => "WARN",
                                        Severity::Info => "INFO",
                                    },
                                    issue.description
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error processing {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok((files_processed, files_fixed))
    }
}
