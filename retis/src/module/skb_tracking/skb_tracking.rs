use std::sync::Arc;

use anyhow::Result;

use super::tracking_hook;
use crate::{
    cli::{dynamic::DynamicCommand, CliConfig},
    collect::Collector,
    core::{
        events::*,
        probe::{manager::ProbeBuilderManager, Hook},
    },
    event_section_factory,
    events::*,
    module::Module,
};

#[derive(Default)]
pub(crate) struct SkbTrackingModule {}

impl Collector for SkbTrackingModule {
    fn new() -> Result<Self> {
        Ok(Self::default())
    }

    fn known_kernel_types(&self) -> Option<Vec<&'static str>> {
        Some(vec!["struct sk_buff *"])
    }

    fn register_cli(&self, cmd: &mut DynamicCommand) -> Result<()> {
        cmd.register_module_noargs(SectionId::SkbTracking)
    }

    fn init(
        &mut self,
        _: &CliConfig,
        probes: &mut ProbeBuilderManager,
        _: Arc<RetisEventsFactory>,
    ) -> Result<()> {
        probes.register_kernel_hook(Hook::from(tracking_hook::DATA))
    }
}

impl Module for SkbTrackingModule {
    fn collector(&mut self) -> &mut dyn Collector {
        self
    }
    fn section_factory(&self) -> Result<Option<Box<dyn EventSectionFactory>>> {
        Ok(Some(Box::new(SkbTrackingEventFactory {})))
    }
}

#[event_section_factory(FactoryId::SkbTracking)]
#[derive(Default)]
pub(crate) struct SkbTrackingEventFactory {}

impl RawEventSectionFactory for SkbTrackingEventFactory {
    fn create(&mut self, raw_sections: Vec<BpfRawSection>) -> Result<Box<dyn EventSection>> {
        let event = parse_single_raw_section::<SkbTrackingEvent>(&raw_sections)?;

        Ok(Box::new(*event))
    }
}

#[cfg(feature = "benchmark")]
pub(crate) mod benchmark {
    use anyhow::Result;

    use crate::{benchmark::helpers::*, core::events::FactoryId, events::SkbTrackingEvent};

    impl RawSectionBuilder for SkbTrackingEvent {
        fn build_raw(out: &mut Vec<u8>) -> Result<()> {
            let data = SkbTrackingEvent::default();
            build_raw_section(out, FactoryId::SkbTracking as u8, 0, &mut as_u8_vec(&data));
            Ok(())
        }
    }
}
