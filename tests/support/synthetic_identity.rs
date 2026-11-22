//! Test support (synthetic only): declare an identity and a direction basis
//! for every edge of a test field, so it passes the RPPI-V10 identity check
//! by declaration, never by bypassing it. Ids are `SYNTHETIC-e{index}` and
//! the basis is marked SYNTHETIC. A registered record declares each
//! relation's id and basis from its own observing chain instead.

use abr_kernel::operators::DeclaredRelations;
use abr_kernel::topology::RelationIdentity;

pub fn identify(r: DeclaredRelations) -> DeclaredRelations {
    let ids = (0..r.n_edges()).map(|e| RelationIdentity {
        id: format!("SYNTHETIC-e{e}"),
        basis: "SYNTHETIC: direction declared for the test".into(),
    }).collect();
    r.with_identity(ids)
}
