use std::{ffi::{CString, NulError}, str::FromStr};

use serde::{Deserialize, Serialize};

#[cfg(feature = "abi")]
pub mod abi;

#[derive(Debug, Serialize, Deserialize)]
/// A partial implementation of `AHQStoreApplication`
/// 
/// Instead of importing the structure from `ahqstore-types`, we've partially constructed the object
pub struct PartialAHQStoreApplication {
  /// Version provided by app developer during registration
  #[serde(rename = "usrVersion")]
  usr_version: Option<String>,
  /// Version set by the AHQ Store for Version Management
  /// Generally a function of date
  version: u32
}

/// This internally stores an AppId for the SDK Application
pub struct SDKApp(pub(crate) CString);

impl SDKApp {
  pub fn create<T: AsRef<str>>(app_id: T) -> Result<Self, NulError> {
    Ok(Self(
          CString::from_str(app_id.as_ref())?
    ))
  }

  pub fn create_from_cstring<T: Into<CString>>(app_id: T) -> Self {
    Self(
      app_id.into()
    )
  }

  pub fn get_appid<'a>(&'a self) -> &'a str {
    &self.0.to_str().expect("Guaranteed to be valid string")
  }
}