/*!
Contains all the model structs used in the Shyft Rust SDK.

<div class="warning">
This does not necessarily cover all response types from the Shyft API.
</div>
*/

pub mod parsed_transaction_details;

use serde::{Deserialize, Serialize};

/// A generic response struct used for API responses.
///
/// # Type Parameters
///
/// * `T` - The type of the result field.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    /// Indicates if the API request was successful.
    pub success: bool,
    /// A message providing additional information about the API response.
    pub message: String,
    /// The result of the API request.
    pub result: T,
}
