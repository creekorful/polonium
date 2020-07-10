use std::time::Duration;

pub mod grabbing;
pub mod scanning;

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(1);
const READ_CONNECT_TIMEOUT: Duration = Duration::from_secs(1);
const WRITE_CONNECT_TIMEOUT: Duration = Duration::from_secs(1);

/// Parse a duration from given milliseconds string.
///
/// # Examples
///
/// ```
/// use boron::parse_duration;
/// use std::time::Duration;
/// let duration = parse_duration(&Some("42"));
/// assert_eq!(duration, Some(Duration::from_millis(42)));
/// ```
pub fn parse_duration(value: &Option<&str>) -> Option<Duration> {
    value
        .map(|v| v.parse::<u64>())
        .filter(|v| v.is_ok())
        .map(|v| Duration::from_millis(v.unwrap()))
}
