use crate::{
    ctxt::AnalysisCtxt,
    hir_lints::not_using_prelude::{NOT_USING_PRELUDE, NotUsingPrelude},
};
use rustc_driver::Callbacks;

pub struct Driver;

impl Callbacks for Driver {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.register_lints = Some(Box::new(move |_, lint_store| {
            lint_store.register_lints(&[NOT_USING_PRELUDE]);

            lint_store.register_late_pass(|tcx| {
                Box::new(NotUsingPrelude {
                    cx: AnalysisCtxt::new(tcx),
                })
            });
        }));
    }
}
