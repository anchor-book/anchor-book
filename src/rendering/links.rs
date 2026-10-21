use super::RenderError;
use std::path::Path;

pub(super) fn route(path: &Path) -> Result<Vec<String>, RenderError> {
    let text = path
        .to_str()
        .ok_or_else(|| RenderError::InvalidInput("chapter routes must be UTF-8".into()))?;
    if text.is_empty()
        || text.contains('\\')
        || text.contains(':')
        || text.chars().any(char::is_control)
        || text
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
    {
        return Err(RenderError::InvalidInput(format!(
            "expected a relative chapter route: {text}"
        )));
    }
    let output = path.with_extension("html");
    Ok(output
        .to_str()
        .expect("UTF-8 path with ASCII extension")
        .split('/')
        .map(str::to_owned)
        .collect())
}
fn encode(segment: &str) -> String {
    let mut result = String::new();
    for b in segment.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            result.push(char::from(b));
        } else {
            use std::fmt::Write;
            write!(result, "%{b:02X}").expect("writing to String");
        }
    }
    result
}
pub(super) fn relative(from: &[String], to: &[String], id: &str) -> String {
    if from == to {
        return format!("#{id}");
    }
    let directory = &from[..from.len() - 1];
    let common = directory.iter().zip(to).take_while(|(a, b)| a == b).count();
    let mut parts = vec!["..".to_owned(); directory.len() - common];
    parts.extend(to[common..].iter().map(|s| encode(s)));
    format!("{}#{id}", parts.join("/"))
}
