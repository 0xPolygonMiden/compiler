use std::{collections::BTreeMap, sync::Arc};

use miden_core::crypto::hash::RpoDigest;
use miden_processor::{
    AdviceExtractor, AdviceInjector, AdviceProvider, ExecutionError, Host, HostResponse,
    MastForest, MastForestStore, MemAdviceProvider, MemMastForestStore, ProcessState, RowIndex,
};

use super::{TraceEvent, TraceHandler};

#[derive(Default)]
pub struct TestHost {
    adv_provider: MemAdviceProvider,
    store: MemMastForestStore,
    tracing_callbacks: BTreeMap<u32, Vec<Box<TraceHandler>>>,
    on_assert_failed: Option<Box<TraceHandler>>,
}
impl TestHost {
    pub fn new(adv_provider: MemAdviceProvider) -> Self {
        Self {
            adv_provider,
            store: Default::default(),
            tracing_callbacks: Default::default(),
            on_assert_failed: None,
        }
    }

    pub fn register_trace_handler<F>(&mut self, event: TraceEvent, callback: F)
    where
        F: FnMut(RowIndex, TraceEvent) + 'static,
    {
        let key = match event {
            TraceEvent::AssertionFailed(None) => u32::MAX,
            ev => ev.into(),
        };
        self.tracing_callbacks.entry(key).or_default().push(Box::new(callback));
    }

    pub fn register_assert_failed_tracer<F>(&mut self, callback: F)
    where
        F: FnMut(RowIndex, TraceEvent) + 'static,
    {
        self.on_assert_failed = Some(Box::new(callback));
    }

    pub fn load_mast_forest(&mut self, forest: MastForest) {
        self.store.insert(forest);
    }
}

impl Host for TestHost {
    fn get_advice<P: ProcessState>(
        &mut self,
        process: &P,
        extractor: AdviceExtractor,
    ) -> Result<HostResponse, ExecutionError> {
        self.adv_provider.get_advice(process, &extractor)
    }

    fn set_advice<P: ProcessState>(
        &mut self,
        process: &P,
        injector: AdviceInjector,
    ) -> Result<HostResponse, ExecutionError> {
        self.adv_provider.set_advice(process, &injector)
    }

    fn get_mast_forest(&self, node_digest: &RpoDigest) -> Option<Arc<MastForest>> {
        self.store.get(node_digest)
    }

    fn on_trace<S: ProcessState>(
        &mut self,
        process: &S,
        trace_id: u32,
    ) -> Result<HostResponse, ExecutionError> {
        let event = TraceEvent::from(trace_id);
        let clk = process.clk();
        if let Some(handlers) = self.tracing_callbacks.get_mut(&trace_id) {
            for handler in handlers.iter_mut() {
                handler(clk, event);
            }
        }
        Ok(HostResponse::None)
    }

    fn on_assert_failed<S: ProcessState>(&mut self, process: &S, err_code: u32) -> ExecutionError {
        let clk = process.clk();
        if let Some(handler) = self.on_assert_failed.as_mut() {
            handler(clk, TraceEvent::AssertionFailed(core::num::NonZeroU32::new(err_code)));
        }
        let err_msg = match err_code {
            midenc_hir::ASSERT_FAILED_ALIGNMENT => Some(
                "failed alignment: use of memory address violates minimum alignment requirements \
                 for that use"
                    .to_string(),
            ),
            _ => None,
        };
        ExecutionError::FailedAssertion {
            clk,
            err_code,
            err_msg,
        }
    }
}
