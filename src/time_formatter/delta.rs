use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::{TimeFormatter, Accuracy};

pub struct Inner {
    time: Option<TimeSpan>,
    drop_decimals: bool,
    accuracy: Accuracy,
}
pub struct Delta(bool, Accuracy);

impl Delta {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_decimal_dropping() -> Self {
        Delta(true, Accuracy::Tenths)
    }
}

impl Default for Delta {
    fn default() -> Self {
        Delta(true, Accuracy::Tenths)
    }
}

impl<'a> TimeFormatter<'a> for Delta {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
        where T: Into<Option<TimeSpan>>
    {
        Inner {
            time: time.into(),
            drop_decimals: self.0,
            accuracy: self.1,
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "−")?;
            } else {
                write!(f, "+")?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                if self.drop_decimals {
                    write!(f, "{}:{:02}:{:02}", hours, minutes, seconds as u8)
                } else {
                    match self.accuracy {
                        Accuracy::Hundredths => {
                            write!(f, "{}:{:02}:{:05.2}", hours, minutes, seconds)
                        }
                        Accuracy::Tenths => write!(f, "{}:{:02}:{:04.1}", hours, minutes, seconds),
                        Accuracy::Seconds => {
                            write!(f, "{}:{:02}:{:02}", hours, minutes, seconds as u8)
                        }
                    }
                }
            } else if total_minutes > 0 {
                if self.drop_decimals {
                    write!(f, "{}:{:02}", minutes, seconds as u8)
                } else {
                    match self.accuracy {
                        Accuracy::Hundredths => write!(f, "{}:{:05.2}", minutes, seconds),
                        Accuracy::Tenths => write!(f, "{}:{:04.1}", minutes, seconds),
                        Accuracy::Seconds => write!(f, "{}:{:02}", minutes, seconds as u8),
                    }
                }
            } else {
                match self.accuracy {
                    Accuracy::Hundredths => write!(f, "{:.2}", seconds),
                    Accuracy::Tenths => write!(f, "{:.1}", seconds),
                    Accuracy::Seconds => write!(f, "{}", seconds as u8),
                }
            }
        } else {
            write!(f, "—")
        }
    }
}
