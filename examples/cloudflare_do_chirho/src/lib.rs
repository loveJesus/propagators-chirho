// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Rust Durable Object for a cryptocurrency portfolio using propagators.
//!
//! This is a complete Cloudflare Worker + Durable Object in pure Rust.

use propagators_chirho::{
    CellChirho, ConstraintSystemChirho, IntervalChirho, NumericInfoChirho, SchedulerChirho,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use worker::*;

// ============================================================================
// TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HoldingChirho {
    asset_chirho: String,
    qty_chirho: f64,
    cost_basis_chirho: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortfolioStateChirho {
    positions_chirho: Vec<PositionChirho>,
    total_nav_chirho: f64,
    total_pnl_chirho: f64,
    updated_at_chirho: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PositionChirho {
    asset_chirho: String,
    qty_chirho: f64,
    price_chirho: f64,
    value_chirho: f64,
    cost_basis_chirho: f64,
    pnl_chirho: f64,
    pnl_pct_chirho: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PriceUpdateChirho {
    asset_chirho: String,
    price_chirho: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TargetNavQueryChirho {
    target_nav_chirho: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TargetNavResultChirho {
    target_nav_chirho: f64,
    required_btc_price_chirho: f64,
    current_btc_price_chirho: f64,
    gap_pct_chirho: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AlertConfigChirho {
    asset_chirho: String,
    threshold_chirho: f64,
    direction_chirho: String, // "above" or "below"
}

// ============================================================================
// DURABLE OBJECT STATE
// ============================================================================

#[durable_object]
pub struct CryptoPortfolioDoChirho {
    state_chirho: State,
    env_chirho: Env,
    // Propagator network components
    scheduler_chirho: Rc<SchedulerChirho>,
    cells_chirho: HashMap<String, Rc<CellChirho<NumericInfoChirho>>>,
    // Cached state
    holdings_chirho: HashMap<String, HoldingChirho>,
    alerts_chirho: Vec<AlertConfigChirho>,
}

#[durable_object]
impl DurableObject for CryptoPortfolioDoChirho {
    fn new(state_chirho: State, env_chirho: Env) -> Self {
        let scheduler_chirho = Rc::new(SchedulerChirho::new_chirho());

        Self {
            state_chirho,
            env_chirho,
            scheduler_chirho,
            cells_chirho: HashMap::new(),
            holdings_chirho: HashMap::new(),
            alerts_chirho: Vec::new(),
        }
    }

    async fn fetch(&mut self, req_chirho: Request) -> Result<Response> {
        // Restore state on first request
        self.restore_state_chirho().await?;

        let path_chirho = req_chirho.path();
        let method_chirho = req_chirho.method();

        match (method_chirho, path_chirho.as_str()) {
            // GET /portfolio - full portfolio state
            (Method::Get, "/portfolio") => {
                let state_chirho = self.get_portfolio_state_chirho();
                Response::from_json(&state_chirho)
            }

            // PUT /price - update asset price
            (Method::Put, "/price") => {
                let update_chirho: PriceUpdateChirho = req_chirho.json().await?;
                self.update_price_chirho(&update_chirho.asset_chirho, update_chirho.price_chirho);
                self.check_alerts_chirho(&update_chirho.asset_chirho).await?;
                Response::from_json(&serde_json::json!({"ok": true}))
            }

            // PUT /holding - update holding
            (Method::Put, "/holding") => {
                let holding_chirho: HoldingChirho = req_chirho.json().await?;
                self.update_holding_chirho(holding_chirho.clone()).await?;
                Response::from_json(&serde_json::json!({"ok": true}))
            }

            // POST /target-nav - bidirectional query
            (Method::Post, "/target-nav") => {
                let query_chirho: TargetNavQueryChirho = req_chirho.json().await?;
                let result_chirho = self.calculate_price_for_target_chirho(query_chirho.target_nav_chirho);
                Response::from_json(&result_chirho)
            }

            // POST /break-even - what price to break even on an asset
            (Method::Post, "/break-even") => {
                let body_chirho: serde_json::Value = req_chirho.json().await?;
                let asset_chirho = body_chirho["asset_chirho"].as_str().unwrap_or("btc");
                let result_chirho = self.calculate_break_even_chirho(asset_chirho);
                Response::from_json(&result_chirho)
            }

            // PUT /alert - set price alert
            (Method::Put, "/alert") => {
                let alert_chirho: AlertConfigChirho = req_chirho.json().await?;
                self.add_alert_chirho(alert_chirho).await?;
                Response::from_json(&serde_json::json!({"ok": true}))
            }

            // GET /cell/:name - raw cell value (for debugging)
            (Method::Get, path_chirho) if path_chirho.starts_with("/cell/") => {
                let cell_name_chirho = path_chirho.strip_prefix("/cell/").unwrap();
                let content_chirho = self.get_cell_content_chirho(cell_name_chirho);
                Response::from_json(&content_chirho)
            }

            _ => Response::error("Not found", 404),
        }
    }
}

impl CryptoPortfolioDoChirho {
    // ========================================================================
    // INITIALIZATION
    // ========================================================================

    async fn restore_state_chirho(&mut self) -> Result<()> {
        if !self.cells_chirho.is_empty() {
            return Ok(()); // Already initialized
        }

        // Initialize propagator network
        self.setup_propagator_network_chirho();

        // Restore holdings from storage
        let storage_chirho = self.state_chirho.storage();

        if let Some(holdings_json_chirho) = storage_chirho.get::<String>("holdings").await? {
            let holdings_chirho: HashMap<String, HoldingChirho> =
                serde_json::from_str(&holdings_json_chirho).unwrap_or_default();

            for (_, holding_chirho) in holdings_chirho.iter() {
                self.apply_holding_chirho(holding_chirho);
            }
            self.holdings_chirho = holdings_chirho;
        }

        // Restore alerts
        if let Some(alerts_json_chirho) = storage_chirho.get::<String>("alerts").await? {
            self.alerts_chirho = serde_json::from_str(&alerts_json_chirho).unwrap_or_default();
        }

        // Restore latest prices
        if let Some(prices_json_chirho) = storage_chirho.get::<String>("prices").await? {
            let prices_chirho: HashMap<String, f64> =
                serde_json::from_str(&prices_json_chirho).unwrap_or_default();

            for (asset_chirho, price_chirho) in prices_chirho {
                self.update_price_chirho(&asset_chirho, price_chirho);
            }
        }

        Ok(())
    }

    fn setup_propagator_network_chirho(&mut self) {
        let assets_chirho = ["btc", "eth", "sol", "usdc"];

        for asset_chirho in assets_chirho {
            // Price cell (fed from external source)
            let price_cell_chirho = self.make_cell_chirho(&format!("{}_price", asset_chirho));

            // Quantity cell (user's holding)
            let qty_cell_chirho = self.make_cell_chirho(&format!("{}_qty", asset_chirho));

            // Cost basis cell (what user paid)
            let cost_basis_cell_chirho = self.make_cell_chirho(&format!("{}_cost_basis", asset_chirho));

            // Value cell (computed: price × qty)
            let value_cell_chirho = self.make_cell_chirho(&format!("{}_value", asset_chirho));

            // P&L cell (computed: value - cost_basis)
            let pnl_cell_chirho = self.make_cell_chirho(&format!("{}_pnl", asset_chirho));

            // Install multiplier: price × qty = value
            propagators_chirho::IntervalMultiplierChirho::install_chirho(
                price_cell_chirho.clone(),
                qty_cell_chirho.clone(),
                value_cell_chirho.clone(),
                &self.scheduler_chirho,
            );

            // Install subtractor: value - cost_basis = pnl
            propagators_chirho::IntervalSubtractorChirho::install_chirho(
                value_cell_chirho,
                cost_basis_cell_chirho,
                pnl_cell_chirho,
                &self.scheduler_chirho,
            );
        }

        // USDC is always $1
        self.set_cell_exact_chirho("usdc_price", 1.0);

        // Total NAV cell
        let _nav_cell_chirho = self.make_cell_chirho("total_nav");

        // Total P&L cell
        let _total_pnl_cell_chirho = self.make_cell_chirho("total_pnl");

        // Target NAV for bidirectional queries
        let _target_nav_cell_chirho = self.make_cell_chirho("target_nav");
        let _required_btc_price_cell_chirho = self.make_cell_chirho("required_btc_price");

        // Run initial propagation
        self.scheduler_chirho.run_chirho();
    }

    fn make_cell_chirho(&mut self, name_chirho: &str) -> Rc<CellChirho<NumericInfoChirho>> {
        let cell_chirho = Rc::new(CellChirho::new_chirho(name_chirho));
        self.cells_chirho.insert(name_chirho.to_string(), cell_chirho.clone());
        cell_chirho
    }

    // ========================================================================
    // CELL OPERATIONS
    // ========================================================================

    fn set_cell_exact_chirho(&self, name_chirho: &str, value_chirho: f64) {
        if let Some(cell_chirho) = self.cells_chirho.get(name_chirho) {
            cell_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(value_chirho));
        }
    }

    fn set_cell_interval_chirho(&self, name_chirho: &str, lo_chirho: f64, hi_chirho: f64) {
        if let Some(cell_chirho) = self.cells_chirho.get(name_chirho) {
            cell_chirho.add_content_chirho(NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho));
        }
    }

    fn get_cell_exact_chirho(&self, name_chirho: &str) -> Option<f64> {
        self.cells_chirho.get(name_chirho).and_then(|cell_chirho| {
            let content_chirho = cell_chirho.content_chirho();
            content_chirho.as_interval_chirho().map(|iv_chirho| iv_chirho.lo_chirho)
        })
    }

    fn get_cell_content_chirho(&self, name_chirho: &str) -> serde_json::Value {
        match self.cells_chirho.get(name_chirho) {
            Some(cell_chirho) => {
                let content_chirho = cell_chirho.content_chirho();
                match content_chirho {
                    NumericInfoChirho::NothingChirho => serde_json::json!({"type": "nothing"}),
                    NumericInfoChirho::IntervalChirho(iv_chirho) => serde_json::json!({
                        "type": "interval",
                        "lo": iv_chirho.lo_chirho,
                        "hi": iv_chirho.hi_chirho
                    }),
                    NumericInfoChirho::ContradictionChirho => serde_json::json!({"type": "contradiction"}),
                }
            }
            None => serde_json::json!({"error": "cell not found"}),
        }
    }

    // ========================================================================
    // PRICE UPDATES
    // ========================================================================

    fn update_price_chirho(&mut self, asset_chirho: &str, price_chirho: f64) {
        let cell_name_chirho = format!("{}_price", asset_chirho);
        self.set_cell_exact_chirho(&cell_name_chirho, price_chirho);

        // Run propagation - all derived values update automatically
        self.scheduler_chirho.run_chirho();

        // Recompute total NAV
        self.recompute_totals_chirho();
    }

    fn recompute_totals_chirho(&mut self) {
        let assets_chirho = ["btc", "eth", "sol", "usdc"];
        let mut total_value_chirho = 0.0;
        let mut total_pnl_chirho = 0.0;

        for asset_chirho in assets_chirho {
            if let Some(value_chirho) = self.get_cell_exact_chirho(&format!("{}_value", asset_chirho)) {
                total_value_chirho += value_chirho;
            }
            if let Some(pnl_chirho) = self.get_cell_exact_chirho(&format!("{}_pnl", asset_chirho)) {
                total_pnl_chirho += pnl_chirho;
            }
        }

        self.set_cell_exact_chirho("total_nav", total_value_chirho);
        self.set_cell_exact_chirho("total_pnl", total_pnl_chirho);
    }

    // ========================================================================
    // HOLDINGS
    // ========================================================================

    async fn update_holding_chirho(&mut self, holding_chirho: HoldingChirho) -> Result<()> {
        self.apply_holding_chirho(&holding_chirho);
        self.holdings_chirho.insert(holding_chirho.asset_chirho.clone(), holding_chirho);

        // Persist
        let storage_chirho = self.state_chirho.storage();
        let holdings_json_chirho = serde_json::to_string(&self.holdings_chirho)?;
        storage_chirho.put("holdings", holdings_json_chirho).await?;

        Ok(())
    }

    fn apply_holding_chirho(&mut self, holding_chirho: &HoldingChirho) {
        let qty_cell_chirho = format!("{}_qty", holding_chirho.asset_chirho);
        let cost_cell_chirho = format!("{}_cost_basis", holding_chirho.asset_chirho);

        self.set_cell_exact_chirho(&qty_cell_chirho, holding_chirho.qty_chirho);
        self.set_cell_exact_chirho(&cost_cell_chirho, holding_chirho.cost_basis_chirho);

        self.scheduler_chirho.run_chirho();
        self.recompute_totals_chirho();
    }

    // ========================================================================
    // PORTFOLIO STATE
    // ========================================================================

    fn get_portfolio_state_chirho(&self) -> PortfolioStateChirho {
        let assets_chirho = ["btc", "eth", "sol", "usdc"];
        let mut positions_chirho = Vec::new();

        for asset_chirho in assets_chirho {
            let qty_chirho = self.get_cell_exact_chirho(&format!("{}_qty", asset_chirho)).unwrap_or(0.0);
            let price_chirho = self.get_cell_exact_chirho(&format!("{}_price", asset_chirho)).unwrap_or(0.0);
            let value_chirho = self.get_cell_exact_chirho(&format!("{}_value", asset_chirho)).unwrap_or(0.0);
            let cost_basis_chirho = self.get_cell_exact_chirho(&format!("{}_cost_basis", asset_chirho)).unwrap_or(0.0);
            let pnl_chirho = self.get_cell_exact_chirho(&format!("{}_pnl", asset_chirho)).unwrap_or(0.0);

            let pnl_pct_chirho = if cost_basis_chirho > 0.0 {
                (pnl_chirho / cost_basis_chirho) * 100.0
            } else {
                0.0
            };

            if qty_chirho > 0.0 || cost_basis_chirho > 0.0 {
                positions_chirho.push(PositionChirho {
                    asset_chirho: asset_chirho.to_string(),
                    qty_chirho,
                    price_chirho,
                    value_chirho,
                    cost_basis_chirho,
                    pnl_chirho,
                    pnl_pct_chirho,
                });
            }
        }

        PortfolioStateChirho {
            positions_chirho,
            total_nav_chirho: self.get_cell_exact_chirho("total_nav").unwrap_or(0.0),
            total_pnl_chirho: self.get_cell_exact_chirho("total_pnl").unwrap_or(0.0),
            updated_at_chirho: js_sys::Date::now() as u64,
        }
    }

    // ========================================================================
    // BIDIRECTIONAL QUERIES
    // ========================================================================

    fn calculate_price_for_target_chirho(&mut self, target_nav_chirho: f64) -> TargetNavResultChirho {
        // Get current state
        let current_nav_chirho = self.get_cell_exact_chirho("total_nav").unwrap_or(0.0);
        let current_btc_price_chirho = self.get_cell_exact_chirho("btc_price").unwrap_or(0.0);
        let btc_qty_chirho = self.get_cell_exact_chirho("btc_qty").unwrap_or(0.0);

        // Calculate other assets' value (non-BTC)
        let other_value_chirho = current_nav_chirho
            - self.get_cell_exact_chirho("btc_value").unwrap_or(0.0);

        // Solve: target_nav = btc_price * btc_qty + other_value
        // btc_price = (target_nav - other_value) / btc_qty
        let required_btc_price_chirho = if btc_qty_chirho > 0.0 {
            (target_nav_chirho - other_value_chirho) / btc_qty_chirho
        } else {
            0.0
        };

        let gap_pct_chirho = if current_btc_price_chirho > 0.0 {
            ((required_btc_price_chirho - current_btc_price_chirho) / current_btc_price_chirho) * 100.0
        } else {
            0.0
        };

        TargetNavResultChirho {
            target_nav_chirho,
            required_btc_price_chirho,
            current_btc_price_chirho,
            gap_pct_chirho,
        }
    }

    fn calculate_break_even_chirho(&self, asset_chirho: &str) -> serde_json::Value {
        let qty_chirho = self.get_cell_exact_chirho(&format!("{}_qty", asset_chirho)).unwrap_or(0.0);
        let cost_basis_chirho = self.get_cell_exact_chirho(&format!("{}_cost_basis", asset_chirho)).unwrap_or(0.0);
        let current_price_chirho = self.get_cell_exact_chirho(&format!("{}_price", asset_chirho)).unwrap_or(0.0);

        // Break-even price = cost_basis / qty
        let break_even_price_chirho = if qty_chirho > 0.0 {
            cost_basis_chirho / qty_chirho
        } else {
            0.0
        };

        let gap_pct_chirho = if current_price_chirho > 0.0 {
            ((break_even_price_chirho - current_price_chirho) / current_price_chirho) * 100.0
        } else {
            0.0
        };

        serde_json::json!({
            "asset_chirho": asset_chirho,
            "break_even_price_chirho": break_even_price_chirho,
            "current_price_chirho": current_price_chirho,
            "gap_pct_chirho": gap_pct_chirho,
            "qty_chirho": qty_chirho,
            "cost_basis_chirho": cost_basis_chirho
        })
    }

    // ========================================================================
    // ALERTS
    // ========================================================================

    async fn add_alert_chirho(&mut self, alert_chirho: AlertConfigChirho) -> Result<()> {
        self.alerts_chirho.push(alert_chirho);

        // Persist
        let storage_chirho = self.state_chirho.storage();
        let alerts_json_chirho = serde_json::to_string(&self.alerts_chirho)?;
        storage_chirho.put("alerts", alerts_json_chirho).await?;

        Ok(())
    }

    async fn check_alerts_chirho(&self, asset_chirho: &str) -> Result<()> {
        let current_price_chirho = self.get_cell_exact_chirho(&format!("{}_price", asset_chirho));

        if let Some(price_chirho) = current_price_chirho {
            for alert_chirho in &self.alerts_chirho {
                if alert_chirho.asset_chirho != asset_chirho {
                    continue;
                }

                let triggered_chirho = match alert_chirho.direction_chirho.as_str() {
                    "above" => price_chirho >= alert_chirho.threshold_chirho,
                    "below" => price_chirho <= alert_chirho.threshold_chirho,
                    _ => false,
                };

                if triggered_chirho {
                    // In production: send push notification, webhook, etc.
                    console_log!(
                        "ALERT: {} {} ${} (current: ${})",
                        asset_chirho,
                        alert_chirho.direction_chirho,
                        alert_chirho.threshold_chirho,
                        price_chirho
                    );
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// WORKER ENTRY POINT
// ============================================================================

#[event(fetch)]
async fn main(req_chirho: Request, env_chirho: Env, _ctx_chirho: Context) -> Result<Response> {
    let router_chirho = Router::new();

    router_chirho
        // Route to Durable Object by user ID
        .on_async("/user/:user_id/*path", |req_chirho, ctx_chirho| async move {
            let user_id_chirho = ctx_chirho.param("user_id").unwrap();

            let namespace_chirho = ctx_chirho.durable_object("CRYPTO_PORTFOLIO")?;
            let stub_chirho = namespace_chirho.id_from_name(user_id_chirho)?.get_stub()?;

            // Forward request to the DO
            stub_chirho.fetch_with_request(req_chirho).await
        })
        // Health check
        .get("/health", |_, _| Response::ok("OK"))
        .run(req_chirho, env_chirho)
        .await
}
