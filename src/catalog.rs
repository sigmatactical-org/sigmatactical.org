//! Curated copy and grouping for repositories shown on sigmatactical.org.

mod enriched_repo;
mod repo_meta;
mod repo_section;
mod section_meta;
pub use enriched_repo::EnrichedRepo;
pub(crate) use repo_meta::RepoMeta;
pub use repo_section::RepoSection;
pub(crate) use section_meta::SectionMeta;

use crate::repos::RepoView;

const SECTIONS: &[SectionMeta] = &[
    SectionMeta {
        id: "vehicle",
        title: "Vehicle & embedded software",
        intro: "Firmware, specifications, and safety-critical code for Sigma Racer. These \
repositories are the reason we open-source: so you can own, audit, and extend the \
software that runs in your vehicle.",
        order: 10,
    },
    SectionMeta {
        id: "data",
        title: "Data formats & bench tooling",
        intro: "Rust libraries for CAN databases and measurement logs—the same formats we \
use on the bench, in validation, and when bridging vehicle networks to analysis tools.",
        order: 20,
    },
    SectionMeta {
        id: "commerce",
        title: "Commerce services",
        intro: "The transactional backbone behind sigmatactical.store: catalog, cart, \
checkout, and order flow, built as small services you can run and adapt.",
        order: 30,
    },
    SectionMeta {
        id: "platform",
        title: "Shared platform",
        intro: "Authentication, persistence, theming, bot protection, and Kubernetes \
manifests reused across every public-facing Sigma Tactical Group service.",
        order: 40,
    },
    SectionMeta {
        id: "sites",
        title: "Public websites",
        intro: "Themed landing and information sites that share sigma-theme chrome and \
link into the rest of the stack.",
        order: 50,
    },
];

const REPO_META: &[(&str, RepoMeta)] = &[
    (
        "sigma-racer-cluster",
        RepoMeta {
            section_id: "vehicle",
            relevance: "Cockpit cluster firmware and display logic for Sigma Racer.",
            description: "Sigma Racer dashboard cluster firmware and in-vehicle display software.",
            order: 10,
        },
    ),
    (
        "sigma-racer-efi",
        RepoMeta {
            section_id: "vehicle",
            relevance: "Engine management for the platform—tune, log, and evolve with your build.",
            description: "Sigma Racer engine management (EFI) firmware and calibration tooling.",
            order: 20,
        },
    ),
    (
        "sigma-racer-wingman",
        RepoMeta {
            section_id: "vehicle",
            relevance: "Companion ECU node on the vehicle network alongside the main stack.",
            description: "Sigma Racer Wingman companion ECU firmware and integration.",
            order: 30,
        },
    ),
    (
        "sigma-racer-sidearm",
        RepoMeta {
            section_id: "vehicle",
            relevance: "M7 safety-core firmware (Embassy) and shared CAN contract for the Wingman cluster.",
            description: "All-Rust M7 safety-core firmware and M7 safety-bus CAN dictionary for the Sigma Racer Wingman cluster.",
            order: 40,
        },
    ),
    (
        "sigma-racer",
        RepoMeta {
            section_id: "vehicle",
            relevance: "Authoritative build specs—mechanical, electrical, and system documents.",
            description: "Sigma Racer build specifications and engineering source documents.",
            order: 50,
        },
    ),
    (
        "sigma-instrumentation",
        RepoMeta {
            section_id: "vehicle",
            relevance: "Shared types and instrumentation glue across vehicle software.",
            description: "Shared instrumentation types and logging utilities for Sigma vehicle software.",
            order: 60,
        },
    ),
    (
        "sigma-racer-telemetry",
        RepoMeta {
            section_id: "vehicle",
            relevance: "VSS state, M7 CAN bridge, and NDJSON IPC shared by cluster and vehicle daemons.",
            description: "VSS vehicle state, M7 CAN bridge, and NDJSON IPC for Sigma Racer cockpit services.",
            order: 70,
        },
    ),
    (
        "sigma-racer-vehicle",
        RepoMeta {
            section_id: "vehicle",
            relevance: "Linux daemon that normalizes CAN/sim into VSS telemetry for the cockpit UI.",
            description: "Linux vehicle daemon: CAN/sim to VSS telemetry for the Sigma Racer cockpit.",
            order: 80,
        },
    ),
    (
        "dbc-rs",
        RepoMeta {
            section_id: "data",
            relevance: "Decode and author CAN networks from DBC—the lingua franca of vehicle buses.",
            description: "Rust library for parsing, editing, and encoding CAN DBC (database) files.",
            order: 10,
        },
    ),
    (
        "mdf4-rs",
        RepoMeta {
            section_id: "data",
            relevance: "Read and write ASAM MDF4 logs from dyno, track, and field capture.",
            description: "Rust library for reading and writing ASAM MDF 4 measurement data files.",
            order: 20,
        },
    ),
    (
        "diagnostics",
        RepoMeta {
            section_id: "data",
            relevance: "Desktop viewer for CAN logs and DBC files—inspect captures off the bench.",
            description: "Desktop CAN bus log and DBC inspection tool built on dbc-rs and mdf4-rs.",
            order: 30,
        },
    ),
    (
        "store",
        RepoMeta {
            section_id: "commerce",
            relevance: "Public storefront listings and product pages for sigmatactical.store.",
            description: "Sigma Tactical Group storefront service (listings, product pages, reservations).",
            order: 10,
        },
    ),
    (
        "catalog",
        RepoMeta {
            section_id: "commerce",
            relevance: "SKU and component catalog—the product model other services consume.",
            description: "Product catalog and SKU management API for Sigma commerce services.",
            order: 20,
        },
    ),
    (
        "cart",
        RepoMeta {
            section_id: "commerce",
            relevance: "Session cart and line items before checkout.",
            description: "Shopping cart service for Sigma Tactical Group commerce.",
            order: 30,
        },
    ),
    (
        "orders",
        RepoMeta {
            section_id: "commerce",
            relevance: "Order capture, deposits, and fulfillment state after cart submit.",
            description: "Order management service for Sigma Tactical Group commerce.",
            order: 40,
        },
    ),
    (
        "addresses",
        RepoMeta {
            section_id: "commerce",
            relevance: "Billing and shipping addresses shared across checkout and payments.",
            description: "Billing and shipping address service for Sigma Tactical Group identity users.",
            order: 41,
        },
    ),
    (
        "payments",
        RepoMeta {
            section_id: "commerce",
            relevance: "Saved payment methods tied to a user's billing address.",
            description: "Payment method service for Sigma Tactical Group identity users.",
            order: 42,
        },
    ),
    (
        "contact",
        RepoMeta {
            section_id: "commerce",
            relevance: "Authenticated contact submissions tied to identity sessions.",
            description: "Contact form and inquiry service for Sigma Tactical Group.",
            order: 50,
        },
    ),
    (
        "accounting",
        RepoMeta {
            section_id: "commerce",
            relevance: "Internal bills and accounting integrations (admin-facing).",
            description: "Internal accounting and billing integrations for Sigma operations.",
            order: 60,
        },
    ),
    (
        "identity",
        RepoMeta {
            section_id: "platform",
            relevance: "OIDC login, sessions, and Keycloak integration for every public app.",
            description: "Identity and OIDC session service for Sigma Tactical Group applications.",
            order: 10,
        },
    ),
    (
        "sigma-pg",
        RepoMeta {
            section_id: "platform",
            relevance: "Shared PostgreSQL migrations, health checks, and internal API auth.",
            description: "Shared PostgreSQL helpers and schema migrations for Sigma web services.",
            order: 20,
        },
    ),
    (
        "theme",
        RepoMeta {
            section_id: "platform",
            relevance: "Shared HTML chrome, static assets, and nav widgets for all sites.",
            description: "Shared UI theme, templates, and static assets for Sigma Tactical Group sites.",
            order: 30,
        },
    ),
    (
        "human-check",
        RepoMeta {
            section_id: "platform",
            relevance: "ALTCHA-backed bot protection for registration and sensitive forms.",
            description: "Human verification (ALTCHA) library for Sigma registration and forms.",
            order: 40,
        },
    ),
    (
        "platform",
        RepoMeta {
            section_id: "platform",
            relevance: "Kubernetes + Istio manifests for dev, staging, and production.",
            description: "Kubernetes and Istio deployment manifests for Sigma Tactical Group services.",
            order: 50,
        },
    ),
    (
        "sentry",
        RepoMeta {
            section_id: "platform",
            relevance: "Operational monitoring hooks and health aggregation.",
            description: "Monitoring and operational health service for the Sigma stack.",
            order: 60,
        },
    ),
    (
        "services",
        RepoMeta {
            section_id: "platform",
            relevance: "Professional services landing and intake pages.",
            description: "Professional services information site for Sigma Tactical Group.",
            order: 70,
        },
    ),
    (
        "info",
        RepoMeta {
            section_id: "platform",
            relevance: "Markdown knowledge base and live Sigma Racer specification viewer.",
            description: "Information and documentation site with Sigma Racer build specifications.",
            order: 80,
        },
    ),
    (
        "updates",
        RepoMeta {
            section_id: "platform",
            relevance: "Debian package index and RAUC OTA catalog serving the Wingman fleet.",
            description: "Debian package index and OTA catalog for Sigma Racer Wingman.",
            order: 90,
        },
    ),
    (
        "sigmatacticalgroup.com",
        RepoMeta {
            section_id: "sites",
            relevance: "Corporate affairs and company presence (cart hidden by design).",
            description: "Sigma Tactical Group corporate website.",
            order: 10,
        },
    ),
    (
        "sigma-tactical.com",
        RepoMeta {
            section_id: "sites",
            relevance: "Customer-facing landing for equipment and services.",
            description: "Sigma Tactical customer landing page and brand entry point.",
            order: 20,
        },
    ),
    (
        "sigmatactical.org",
        RepoMeta {
            section_id: "sites",
            relevance: "This open-source directory—you are here.",
            description: "Open-source showcase for Sigma Tactical Group GitHub repositories.",
            order: 30,
        },
    ),
];

fn meta_for(name: &str) -> Option<&'static RepoMeta> {
    REPO_META
        .iter()
        .find(|(repo, _)| *repo == name)
        .map(|(_, meta)| meta)
}

/// Primary CI workflow file for a repo, used to fetch/display build status.
///
/// Returns `None` for repos we don't publish CI for. Curated repos default to
/// `ci.yml`; the handful with a differently named primary workflow are listed
/// explicitly. Repos absent from the catalog get no badge.
#[must_use]
pub fn primary_workflow(name: &str) -> Option<&'static str> {
    match name {
        "dbc-rs" => Some("dbc-rs.yml"),
        "mdf4-rs" => Some("mdf4-rs.yml"),
        "sigma-racer-wingman" => Some("yocto-virt.yml"),
        // Docs/spec repo with no CI pipeline.
        "sigma-racer" => None,
        other => meta_for(other).map(|_| "ci.yml"),
    }
}

fn enrich(repo: RepoView) -> (EnrichedRepo, &'static str, u16) {
    let meta = meta_for(&repo.name);
    let section_id = meta.map(|m| m.section_id).unwrap_or("platform");
    let order = meta.map(|m| m.order).unwrap_or(900);
    let relevance = meta
        .map(|m| m.relevance)
        .unwrap_or("Part of the Sigma Tactical Group open-source stack.");
    let description = if repo.description.is_empty() {
        meta.map(|m| m.description)
            .unwrap_or("Sigma Tactical Group open-source project.")
            .to_string()
    } else {
        repo.description
    };
    (
        EnrichedRepo {
            name: repo.name,
            url: repo.url,
            description,
            relevance: relevance.to_string(),
            language: repo.language,
            stars: repo.stars,
            build: repo.build,
        },
        section_id,
        order,
    )
}

/// Group API results into editorial sections (known repos first, unknown in platform).
pub fn build_sections(repos: Vec<RepoView>) -> Vec<RepoSection> {
    let mut buckets: Vec<(EnrichedRepo, &'static str, u16)> =
        repos.into_iter().map(enrich).collect();

    buckets.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.name.cmp(&b.0.name)));

    let mut sections: Vec<RepoSection> = SECTIONS
        .iter()
        .map(|section| RepoSection {
            id: section.id,
            title: section.title,
            intro: section.intro,
            repos: Vec::new(),
        })
        .collect();
    sections.sort_by_key(|section| {
        SECTIONS
            .iter()
            .find(|meta| meta.id == section.id)
            .map(|meta| meta.order)
            .unwrap_or(999)
    });

    for (repo, section_id, _) in buckets {
        if let Some(section) = sections.iter_mut().find(|s| s.id == section_id) {
            section.repos.push(repo);
        }
    }

    sections.retain(|s| !s.repos.is_empty());
    sections
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repos::BuildStatus;

    fn sample_repo(name: &str) -> RepoView {
        RepoView {
            name: name.to_string(),
            url: format!("https://github.com/sigmatactical-org/{name}"),
            description: String::new(),
            language: "Rust".to_string(),
            stars: 0,
            default_branch: "main".to_string(),
            build: None,
        }
    }

    #[test]
    fn primary_workflow_maps_known_repos() {
        assert_eq!(primary_workflow("sigma-racer-efi"), Some("ci.yml"));
        assert_eq!(primary_workflow("diagnostics"), Some("ci.yml"));
        assert_eq!(primary_workflow("dbc-rs"), Some("dbc-rs.yml"));
        assert_eq!(primary_workflow("mdf4-rs"), Some("mdf4-rs.yml"));
        assert_eq!(
            primary_workflow("sigma-racer-wingman"),
            Some("yocto-virt.yml")
        );
        assert_eq!(primary_workflow("sigma-racer"), None);
        assert_eq!(primary_workflow("sigma-racer-vehicle"), Some("ci.yml"));
        assert_eq!(primary_workflow("updates"), Some("ci.yml"));
        assert_eq!(primary_workflow("not-a-repo"), None);
    }

    #[test]
    fn build_status_passes_through_enrichment() {
        let mut repo = sample_repo("dbc-rs");
        repo.build = Some(BuildStatus {
            state: crate::repos::BuildState::Passing,
            url: "https://github.com/sigmatactical-org/dbc-rs/actions/workflows/dbc-rs.yml"
                .to_string(),
        });
        let sections = build_sections(vec![repo]);
        let enriched = &sections
            .iter()
            .find(|s| s.id == "data")
            .expect("data")
            .repos[0];
        assert_eq!(
            enriched.build.as_ref().map(|b| b.state),
            Some(crate::repos::BuildState::Passing)
        );
    }

    #[test]
    fn groups_vehicle_repos_into_first_section() {
        let sections = build_sections(vec![
            sample_repo("store"),
            sample_repo("dbc-rs"),
            sample_repo("sigma-racer-efi"),
        ]);
        assert_eq!(sections[0].id, "vehicle");
        assert_eq!(sections[0].repos[0].name, "sigma-racer-efi");
        assert!(sections[0].repos[0].relevance.contains("Engine management"));
        assert!(
            sections
                .iter()
                .any(|s| s.id == "data" && s.repos[0].name == "dbc-rs")
        );
        assert!(
            sections
                .iter()
                .any(|s| s.id == "commerce" && s.repos[0].name == "store")
        );
    }

    #[test]
    fn uses_catalog_description_when_github_description_empty() {
        let sections = build_sections(vec![sample_repo("mdf4-rs")]);
        let repo = &sections
            .iter()
            .find(|s| s.id == "data")
            .expect("data section")
            .repos[0];
        assert!(repo.description.contains("MDF 4"));
    }
}
