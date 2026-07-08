#!/usr/bin/env bash
# Apply curated GitHub repository descriptions for sigmatactical-org.
set -euo pipefail

ORG="${GITHUB_ORG:-sigmatactical-org}"

declare -A DESCRIPTIONS=(
  [sigma-racer-cluster]="Sigma Racer dashboard cluster firmware and in-vehicle display software."
  [sigma-racer-efi]="Sigma Racer engine management (EFI) firmware and calibration tooling."
  [sigma-racer-wingman]="Sigma Racer Wingman companion ECU firmware and integration."
  [cafe-racer-sidearm]="All-Rust M7 safety-core firmware for the Sigma Racer Wingman cluster (Embassy)."
  [sigma-racer-specs]="Sigma Racer build specifications and engineering source documents."
  [sigma-instrumentation]="Shared instrumentation types and logging utilities for Sigma vehicle software."
  [dbc-rs]="Rust library for parsing, editing, and encoding CAN DBC (database) files."
  [mdf4-rs]="Rust library for reading and writing ASAM MDF 4 measurement data files."
  [store]="Sigma Tactical Group storefront service (listings, product pages, reservations)."
  [catalog]="Product catalog and SKU management API for Sigma commerce services."
  [cart]="Shopping cart service for Sigma Tactical Group commerce."
  [order]="Order management service for Sigma Tactical Group commerce."
  [contact]="Contact form and inquiry service for Sigma Tactical Group."
  [accounting]="Internal accounting and billing integrations for Sigma operations."
  [identity]="Identity and OIDC session service for Sigma Tactical Group applications."
  [sigma-pg]="Shared PostgreSQL helpers and schema migrations for Sigma web services."
  [theme]="Shared UI theme, templates, and static assets for Sigma Tactical Group sites."
  [human-check]="Human verification (ALTCHA) library for Sigma registration and forms."
  [platform]="Kubernetes and Istio deployment manifests for Sigma Tactical Group services."
  [sentry]="Monitoring and operational health service for the Sigma stack."
  [services]="Professional services information site for Sigma Tactical Group."
  [info]="Information and documentation site with Sigma Racer build specifications."
  [sigmatacticalgroup.com]="Sigma Tactical Group corporate website."
  [sigma-tactical.com]="Sigma Tactical customer landing page and brand entry point."
  [sigmatactical.org]="Open-source showcase for Sigma Tactical Group GitHub repositories."
)

declare -A HOMEPAGES=(
  [sigmatactical.org]="https://sigmatactical.org"
  [sigmatacticalgroup.com]="https://sigmatacticalgroup.com"
  [sigma-tactical.com]="https://sigma-tactical.com"
  [store]="https://sigmatactical.store"
  [info]="https://sigmatactical.info"
  [services]="https://services.sigma-tactical.com"
  [dbc-rs]="https://crates.io/crates/dbc-rs"
)

for repo in "${!DESCRIPTIONS[@]}"; do
  desc="${DESCRIPTIONS[$repo]}"
  echo "==> ${ORG}/${repo}"
  if [[ -n "${HOMEPAGES[$repo]:-}" ]]; then
    gh repo edit "${ORG}/${repo}" --description "$desc" --homepage "${HOMEPAGES[$repo]}"
  else
    gh repo edit "${ORG}/${repo}" --description "$desc"
  fi
done

echo "Updated ${#DESCRIPTIONS[@]} repository descriptions."
