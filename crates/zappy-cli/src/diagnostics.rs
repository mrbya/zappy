use std::fmt;

use zappy_fs::{DiscoveredTemplate, TemplateSearchPath};

/// User-facing diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Error diagnostic.
    Error,

    /// Warning diagnostic.
    Warning,

    /// Informational diagnostic.
    Info,
}

impl Severity {
    /// Returns display label for severity.
    #[must_use]
    const fn label(self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Info => "Info",
        }
    }
}

/// Small user-facing diagnostic report.
///
/// This is intentionally light for now. Will be replaced or
/// backed by `miette`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticReport {
    /// Diagnostic severity.
    severity: Severity,

    /// Main diagnostic title.
    title: String,

    /// Additional detail lines.
    details: Vec<String>,

    /// Suggested user actions.
    hints: Vec<String>,
}

impl DiagnosticReport {
    /// Constructs an error diagnostic.
    #[must_use]
    pub fn error(title: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            title: title.into(),
            details: Vec::new(),
            hints: Vec::new(),
        }
    }

    /// Constructs a warning diagnostic.
    #[must_use]
    pub fn warning(title: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            title: title.into(),
            details: Vec::new(),
            hints: Vec::new(),
        }
    }

    /// Constructs an info diagnostic.
    #[must_use]
    pub fn info(title: impl Into<String>) -> Self {
        Self {
            severity: Severity::Info,
            title: title.into(),
            details: Vec::new(),
            hints: Vec::new(),
        }
    }

    /// Adds a detail line.
    #[must_use]
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.details.push(detail.into());
        self
    }

    /// Adds a hint line.
    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }

    /// Prints diagnostic to stderr.
    pub fn print(&self) {
        match self.severity {
            Severity::Info => println!("{self}"),
            _ => eprintln!("{self}"),
        }
    }
}

impl fmt::Display for DiagnosticReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {}", self.severity.label(), self.title)?;

        if !self.details.is_empty() {
            writeln!(f)?;

            for detail in &self.details {
                writeln!(f, "{detail}")?;
            }
        }

        if !self.hints.is_empty() {
            writeln!(f)?;
            writeln!(f, "Hint:")?;

            for hint in &self.hints {
                writeln!(f, "  {hint}")?;
            }
        }

        Ok(())
    }
}

/// Prints a generic error.
pub fn print_error(title: impl Into<String>) {
    DiagnosticReport::error(title).print();
}

/// Prints an error with source detail.
pub fn print_error_with_source(title: impl Into<String>, source: impl fmt::Display) {
    DiagnosticReport::error(title)
        .detail(format!("Cause: {source}"))
        .print();
}

/// Prints a generic warning.
pub fn print_warning(title: impl Into<String>) {
    DiagnosticReport::warning(title).print();
}

/// Prints a warning with source detail.
pub fn print_warning_with_source(title: impl Into<String>, source: impl fmt::Display) {
    DiagnosticReport::warning(title)
        .detail(format!("Cause: {source}"))
        .print();
}

/// Prints an info message.
pub fn print_info(title: impl Into<String>) {
    DiagnosticReport::info(title).print();
}

/// Prints an info message with details.
pub fn print_info_with_details(title: impl Into<String>, details: impl fmt::Display) {
    DiagnosticReport::info(title)
        .detail("Details:")
        .detail(format!("  {details}"))
        .print();
}

/// Prints a template-not-found diagnostic.
pub fn print_template_not_found(
    id: &str,
    search_paths: &[TemplateSearchPath],
    available_templates: &[DiscoveredTemplate],
) {
    let mut report = DiagnosticReport::error(format!("template `{id}` was not found"));

    if !search_paths.is_empty() {
        report = report.detail(String::from("Searched:"));

        for search_path in search_paths {
            report = report.detail(format!("  {search_path}"));
        }
    }

    if !available_templates.is_empty() {
        report = report.detail(String::new());
        report = report.detail(String::from("Available templates:"));
        report = report.detail(format!("  {:<24} {:<16} Name", "ID", "Language"));
        report = report.detail(format!("  {:-<24} {:-<16} {:-<24}", "", "", ""));

        for template in available_templates {
            let metadata = &template.manifest.template;
            let language = metadata.language.as_deref().unwrap_or("-");
            report = report.detail(format!(
                "  {:<24} {:<16} {}",
                metadata.id.as_str(),
                language,
                metadata.name,
            ));
        }
    }

    report
        .hint("run `zappy list` to see available templates")
        .hint("pass in `--templates-dir <PATH>` to use a specific template directory")
        .print();
}
