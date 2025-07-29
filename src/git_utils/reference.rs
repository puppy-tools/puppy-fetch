use git2::{Reference, ReferenceType, Repository};

pub fn valid_references(repo: &Repository) -> Result<Vec<Reference<'_>>, git2::Error> {
    Ok(repo.references()?.filter_map(Result::ok).collect())
}

pub fn is_symbolic_reference(reference: &Reference<'_>) -> bool {
    match reference.kind() {
        Some(reference) => reference == ReferenceType::Symbolic,
        None => false,
    }
}
