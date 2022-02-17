use sp_runtime::DispatchError;

use crate::primitives::*;

/// Trait for providing control over Asylum Item interpretations
pub trait Interpretable<BoundedString> {
	/// Create new interpretation type
	///
	/// # Arguments
	///
	/// * `type_name` - A bounded string that hold humanreadble name of the interpretation type
	/// * `metadata` - A bounded string that hold ifsh hash to metadata
	///
	/// # Return
	///
	/// Id of newly create interpretation type
	fn interpretation_type_create(
		type_name: &BoundedString,
		metadata: BoundedString,
	) -> Result<InterpretationTypeId, DispatchError>;

	/// Create new interpretation
	///
	/// # Arguments
	///
	/// * `interpretation_name` - A bounded string that hold humanreadble name of the interpretation type
	/// * `src` - A bounded string that hold ifsh hash to media(image, model, sound, etc.)
	/// * `metadata` - A bounded string that hold ifsh hash to metadata
	///
	/// # Return
	///
	/// Id of newly create interpretation
	fn interpretation_create(
		type_name: &BoundedString,
		interpretation_name: BoundedString,
		src: BoundedString,
		metadata: BoundedString,
	) -> Result<InterpretationId, DispatchError>;
}
