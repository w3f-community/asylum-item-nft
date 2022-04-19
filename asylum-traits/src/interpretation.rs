use sp_runtime::DispatchResult;

/// Trait for providing control over Asylum Item interpretations
pub trait Interpretable<BoundedString, BoundedTag> {
	/// Create new interpretation tag
	///
	/// # Arguments
	///
	/// * `tag` - A bounded string that hold tag
	/// * `metadata` - A bounded string that hold ipfs hash to metadata
	///
	/// # Return
	///
	/// Id of newly create interpretation type
	fn interpretation_tag_create(tag: &BoundedTag, metadata: BoundedString) -> DispatchResult;
}
