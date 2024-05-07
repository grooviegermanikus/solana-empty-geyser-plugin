use std::cell::RefCell;
use std::ops::Sub;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::{Duration, SystemTime};
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::{GeyserPlugin, ReplicaAccountInfoVersions, SlotStatus};
use solana_sdk::clock::Slot;
use crate::debouncer::Debouncer;


pub mod config;
mod debouncer;

#[derive(Debug)]
pub struct Plugin {
    debouncer: Debouncer,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            debouncer: Debouncer::new(Duration::from_millis(2)),
        }
    }
}

impl GeyserPlugin for Plugin {
    fn name(&self) -> &'static str {
        "geyser_account_timestamp_tagging_plugin"
    }

    fn on_load(
        &mut self,
        _config_file: &str,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        solana_logger::setup_with_default(&"info");

        Ok(())
    }

    fn on_unload(&mut self) {}

    // true is actually the default
    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn update_slot_status(&self, slot: Slot, _parent: Option<u64>, _status: SlotStatus) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");

        info!("account slot: timestamp_us={};slot={}", since_the_epoch.as_micros(), slot);
        Ok(())
    }

    fn update_account(&self, account: ReplicaAccountInfoVersions, slot: Slot, is_startup: bool) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        if is_startup {
            return Ok(());
        }

        // TODO is that good?
        // if !self.debouncer.can_fire() {
        //     return Ok(());
        // }

        let account = match account {
            ReplicaAccountInfoVersions::V0_0_1(_info) => {
                unreachable!("ReplicaAccountInfoVersions::V0_0_1 is not supported")
            }
            ReplicaAccountInfoVersions::V0_0_2(_info) => {
                unreachable!("ReplicaAccountInfoVersions::V0_0_2 is not supported")
            }
            ReplicaAccountInfoVersions::V0_0_3(info) => info,
        };

        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");

        info!("account update: write_version={};timestamp_us={};slot={}", account.write_version, since_the_epoch.as_micros(), slot);

        Ok(())
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Plugin::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
