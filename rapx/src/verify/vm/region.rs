//! Region (lifetime) helpers shared by the VM and the property checker.

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{EarlyParamRegion, GenericParamDefKind, Region, RegionKind, TyCtxt};

/// Resolve a lifetime name from a contract (e.g. `"a"`, `"static"`) to a
/// concrete `Region`. `name` is the raw ident, without the leading `'`.
pub(crate) fn resolve_region_name<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    name: &str,
) -> Option<Region<'tcx>> {
    if name == "static" || name == "static_lifetime" {
        return Some(tcx.lifetimes.re_static);
    }
    let ticked = format!("'{name}");
    let generics = tcx.generics_of(def_id);
    for param in &generics.own_params {
        if matches!(param.kind, GenericParamDefKind::Lifetime)
            && (param.name.as_str() == name || param.name.as_str() == ticked.as_str())
        {
            return Some(Region::new_early_param(
                tcx,
                EarlyParamRegion {
                    index: param.index,
                    name: param.name,
                },
            ));
        }
    }
    None
}

/// Whether `src` outlives `ret` (`src: ret`).
///
/// `'static` and reflexivity are handled structurally; declared where-clauses
/// (`'a: 'b`) are resolved via rustc's `FreeRegionMap` (which also computes the
/// transitive closure). Anything else is treated as *not* outlives.
pub(crate) fn region_outlives<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    src: Region<'tcx>,
    ret: Region<'tcx>,
) -> bool {
    match (src.kind(), ret.kind()) {
        (RegionKind::ReStatic, _) => true,
        (_, RegionKind::ReStatic) => false,
        _ => {
            if src == ret {
                return true;
            }
            if !src.is_free() || !ret.is_free() {
                return false;
            }
            free_region_outlives(tcx, def_id, src, ret)
        }
    }
}

/// Consult the function's declared outlives constraints (where-clauses) via
/// rustc's `FreeRegionMap`, which records `'a: 'b` bounds and their transitive
/// closure. `sub_free_regions(r_a, r_b)` tests `r_a <= r_b` (i.e. `r_b: r_a`),
/// so `src: ret` is `sub_free_regions(ret, src)`.
fn free_region_outlives<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    src: Region<'tcx>,
    ret: Region<'tcx>,
) -> bool {
    use rustc_data_structures::fx::FxHashSet;
    use rustc_infer::infer::outlives::env::OutlivesEnvironment;

    let param_env = tcx.param_env(def_id);
    let env = OutlivesEnvironment::from_normalized_bounds(
        param_env,
        Vec::new(),
        std::iter::empty(),
        FxHashSet::default(),
    );
    env.free_region_map().sub_free_regions(tcx, ret, src)
}
