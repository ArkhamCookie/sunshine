//! Errors and error handling

use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
pub(crate) enum SunshineError {
	ApiError(reqwest::Error),
	ConvertError(num::ParseFloatError),
	JsonError(serde_json::Error),
	MalformedLocationString,
	UnknownLocationName,
}

impl fmt::Display for SunshineError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			SunshineError::ApiError(err) => write!(f, "api connection error: {:?}", err),
			SunshineError::ConvertError(err) => write!(f, "converting type error: {:?}", err),
			SunshineError::JsonError(err) => write!(f, "json deserialization error: {:?}", err),
			SunshineError::MalformedLocationString => write!(f, "malformed location string"),
			SunshineError::UnknownLocationName => write!(f, "requested location can not be found"),
		}
	}
}

impl error::Error for SunshineError {
	fn cause(&self) -> Option<&dyn error::Error> {
		match self {
			SunshineError::JsonError(cause) => Some(cause),
			_ => None,
		}
	}
}

impl From<num::ParseFloatError> for SunshineError {
	fn from(value: num::ParseFloatError) -> Self {
		SunshineError::ConvertError(value)
	}
}

impl From<reqwest::Error> for SunshineError {
	fn from(value: reqwest::Error) -> Self {
		SunshineError::ApiError(value)
	}
}

impl From<serde_json::Error> for SunshineError {
	fn from(value: serde_json::Error) -> Self {
		SunshineError::JsonError(value)
	}
}
