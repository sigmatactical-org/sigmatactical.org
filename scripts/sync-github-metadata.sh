#!/usr/bin/env bash
# Apply curated GitHub repository metadata for sigmatactical-org.
# Descriptions mirror src/catalog.rs; homepages use production URLs from platform overlays.
set -euo pipefail

ORG="${GITHUB_ORG:-sigmatactical-org}"

declare -A DESCRIPTIONS=(
  [sigma-racer-cluster]="Sigma Racer dashboard cluster firmware and in-vehicle display software."
  [sigma-racer-efi]="Sigma Racer engine management (EFI) firmware and calibration tooling."
  [sigma-racer-wingman]="Sigma Racer Wingman companion ECU firmware and integration."
  [sigma-racer-sidearm]="All-Rust M7 safety-core firmware and M7 safety-bus CAN dictionary for the Sigma Racer Wingman cluster."
  [sigma-racer]="Sigma Racer build specifications and engineering source documents."
  [sigma-instrumentation]="Shared instrumentation types and logging utilities for Sigma vehicle software."
  [sigma-racer-telemetry]="VSS vehicle state, M7 CAN bridge, and NDJSON IPC for Sigma Racer cockpit services."
  [sigma-racer-vehicle]="Linux vehicle daemon: CAN/sim to VSS telemetry for the Sigma Racer cockpit."
  [dbc-rs]="Rust library for parsing, editing, and encoding CAN DBC (database) files."
  [mdf4-rs]="Rust library for reading and writing ASAM MDF 4 measurement data files."
  [store]="Sigma Tactical Group storefront service (listings, product pages, reservations)."
  [catalog]="Product catalog and SKU management API for Sigma commerce services."
  [cart]="Shopping cart service for Sigma Tactical Group commerce."
  [orders]="Order management service for Sigma Tactical Group commerce."
  [addresses]="Billing and shipping address service for Sigma Tactical Group identity users."
  [payments]="Payment method service for Sigma Tactical Group identity users."
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

# Production public URLs (platform/services/*/overlays/prod).
declare -A HOMEPAGES=(
  [sigmatactical.org]="https://sigmatactical.org"
  [sigmatacticalgroup.com]="https://sigmatacticalgroup.com"
  [sigma-tactical.com]="https://sigma-tactical.com"
  [store]="https://sigmatactical.store"
  [catalog]="https://catalog.sigmatactical.store"
  [cart]="https://cart.sigmatactical.store"
  [orders]="https://orders.sigmatactical.store"
  [addresses]="https://addresses.sigma-tactical.com"
  [payments]="https://payments.sigma-tactical.com"
  [contact]="https://contact.sigma-tactical.com"
  [identity]="https://identity.sigma-tactical.com"
  [services]="https://services.sigmatacticalgroup.com"
  [info]="https://info.sigmatacticalgroup.com"
  [sigma-racer]="https://info.sigmatacticalgroup.com"
  [dbc-rs]="https://crates.io/crates/dbc-rs"
  [mdf4-rs]="https://docs.rs/mdf4-rs"
  [sigma-racer-cluster]="https://sigmatactical.org"
  [sigma-racer-efi]="https://sigmatactical.org"
  [sigma-racer-wingman]="https://sigmatactical.org"
  [sigma-racer-sidearm]="https://sigmatactical.org"
  [sigma-instrumentation]="https://sigmatactical.org"
  [sigma-racer-telemetry]="https://sigmatactical.org"
  [sigma-racer-vehicle]="https://sigmatactical.org"
)

declare -A TOPICS=(
  [sigma-racer-cluster]="sigma-tactical sigma-racer embedded rust firmware motorcycle instrument-cluster"
  [sigma-racer-efi]="sigma-tactical sigma-racer embedded rust firmware motorcycle efi ecu"
  [sigma-racer-wingman]="sigma-tactical sigma-racer embedded rust firmware yocto linux"
  [sigma-racer-sidearm]="sigma-tactical sigma-racer embedded rust firmware embassy safety can"
  [sigma-racer]="sigma-tactical sigma-racer motorcycle specifications engineering documentation"
  [sigma-instrumentation]="sigma-tactical sigma-racer embedded rust instrumentation logging"
  [sigma-racer-telemetry]="sigma-tactical sigma-racer embedded rust telemetry vss can"
  [sigma-racer-vehicle]="sigma-tactical sigma-racer embedded rust telemetry can linux"
  [dbc-rs]="sigma-tactical rust can dbc automotive embedded"
  [mdf4-rs]="sigma-tactical rust mdf4 automotive measurement logging"
  [store]="sigma-tactical rust web ecommerce storefront"
  [catalog]="sigma-tactical rust web ecommerce api catalog"
  [cart]="sigma-tactical rust web ecommerce api cart"
  [orders]="sigma-tactical rust web ecommerce api order"
  [addresses]="sigma-tactical rust web ecommerce api address"
  [payments]="sigma-tactical rust web ecommerce api payment"
  [contact]="sigma-tactical rust web api contact"
  [accounting]="sigma-tactical rust api accounting internal"
  [identity]="sigma-tactical rust oidc authentication identity"
  [sigma-pg]="sigma-tactical rust postgresql database"
  [theme]="sigma-tactical rust web ui theme"
  [human-check]="sigma-tactical rust altcha security bot-protection"
  [platform]="sigma-tactical kubernetes istio kustomize infrastructure"
  [sentry]="sigma-tactical rust monitoring operations"
  [services]="sigma-tactical rust web services"
  [info]="sigma-tactical rust documentation specifications"
  [sigmatacticalgroup.com]="sigma-tactical rust website corporate"
  [sigma-tactical.com]="sigma-tactical rust website brand"
  [sigmatactical.org]="sigma-tactical rust open-source website"
)

set_topics() {
  local repo="$1"
  local topics="$2"
  local -a names=()
  read -r -a names <<<"$topics"
  local payload
  payload="$(printf '%s\n' "${names[@]}" | jq -R . | jq -s '{names: .}')"
  gh api -X PUT "repos/${ORG}/${repo}/topics" \
    --input - <<<"$payload" \
    -H "Accept: application/vnd.github+json"
}

for repo in "${!DESCRIPTIONS[@]}"; do
  desc="${DESCRIPTIONS[$repo]}"
  echo "==> ${ORG}/${repo}"
  args=(--description "$desc")
  if [[ -n "${HOMEPAGES[$repo]:-}" ]]; then
    args+=(--homepage "${HOMEPAGES[$repo]}")
  fi
  gh repo edit "${ORG}/${repo}" "${args[@]}"
  if [[ -n "${TOPICS[$repo]:-}" ]]; then
    set_topics "$repo" "${TOPICS[$repo]}"
  fi
done

echo "Updated ${#DESCRIPTIONS[@]} repositories (description, homepage, topics)."
echo "Note: GitHub has no public API to hide Releases, Packages, or Deployments on the repo homepage."
echo "      Disable those under About -> Include in the home page if any repo still shows them."
