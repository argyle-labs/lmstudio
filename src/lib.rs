//! lmstudio service backend — LM Studio local LLM runner (host desktop app).
//!
//! Implements `ServiceBackend` so the generic `service.*` tools
//! (deploy/backup/restore/configure/status/connect/sync) drive lmstudio. No
//! `#[orca_tool]`s — the only orca dep is `plugin-toolkit`. Modeled on the
//! nfs StorageBackend. See orca/docs/PLUGIN-PROGRAM.md.
#![allow(clippy::disallowed_types)]

use plugin_toolkit::service::{
    BoxFuture, Endpoint, Runtime, ServiceBackend, ServiceCapability, ServiceError, ServiceStatus,
    WorkloadSpec,
};

mod abi_export;

/// lmstudio backend. Holds only the provider name; per-instance endpoint/creds
/// come from the `Endpoint` the generic `service.*` tools hand each op.
#[derive(Debug, Clone)]
pub struct LmstudioBackend {
    provider: &'static str,
}

impl LmstudioBackend {
    pub fn new(provider: &'static str) -> Self {
        Self { provider }
    }
}

impl ServiceBackend for LmstudioBackend {
    fn provider(&self) -> &str { self.provider }

    /// Runtimes lmstudio can be placed on. `service.deploy` hands the
    /// `workload_spec` below to a matching deploy target — this backend never
    /// drives pct/docker itself (that mechanic lives in the deploy-target domain).
    fn runtimes(&self) -> Vec<Runtime> { vec![] }

    fn capabilities(&self) -> Vec<ServiceCapability> { vec![ServiceCapability::Backup, ServiceCapability::Restore, ServiceCapability::Configure, ServiceCapability::Status] }

    fn default_port(&self) -> u16 { 1234 }

    /// In-workload paths holding config/data. This is ALL lmstudio declares for
    /// backup — the generic pluggable backup (tar for containers/LXC, PBS for
    /// Proxmox guests when available) snapshots these. No backup/restore code
    /// here; those are inherited from ServiceBackend's defaults.
    fn data_paths(&self) -> Vec<String> { vec!["/config".to_string()] }

    fn workload_spec<'a>(&'a self, _runtime: Runtime, _ep: &'a Endpoint)
        -> BoxFuture<'a, Result<WorkloadSpec, ServiceError>>
    {
        // TODO: describe the lmstudio workload (image/template, ports, mounts,
        // env) for the chosen runtime. The deploy target turns this into a
        // compose service / LXC config / VM. See deploy-target::WorkloadSpec.
        Box::pin(async move { Err(ServiceError::unimplemented("lmstudio.workload_spec")) })
    }

    fn configure<'a>(&'a self, _ep: &'a Endpoint, _config: &'a str)
        -> BoxFuture<'a, Result<(), ServiceError>>
    {
        // TODO: apply lmstudio-specific config idempotently.
        Box::pin(async move { Err(ServiceError::unimplemented("lmstudio.configure")) })
    }

    fn status<'a>(&'a self, _ep: &'a Endpoint)
        -> BoxFuture<'a, Result<ServiceStatus, ServiceError>>
    {
        // TODO: real health/diagnostics.
        Box::pin(async move { Err(ServiceError::unimplemented("lmstudio.status")) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declares_provider() {
        let b = LmstudioBackend::new("lmstudio");
        assert_eq!(b.provider(), "lmstudio");
    }
}
