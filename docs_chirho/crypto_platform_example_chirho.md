<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Cryptocurrency Trading Platform with Propagators

## The Use Case

A **real-time crypto portfolio tracker and trading platform** where:

- **Price feeds** from 10+ exchanges update every 100ms
- **10K+ users** each with their own portfolios
- **Derived metrics** compute automatically (P&L, NAV, risk metrics)
- **Bidirectional queries**: "What price does ETH need for me to break even?"
- **Alerts trigger** when conditions are met
- **Order execution** based on computed values

## Why Propagators Are Perfect Here

| Traditional Approach | Propagator Approach |
|---------------------|---------------------|
| Poll prices, recalculate everything | Push prices, only affected cells update |
| "What's my P&L?" requires full portfolio scan | P&L cell is always current |
| "Break-even price?" requires solver | Set target P&L=0, read back break-even price |
| Alert checks run on timer | Alerts fire instantly when conditions met |
| Complex dependency tracking | Dependencies are the propagator network |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Crypto Platform Architecture                         │
│                                                                              │
│  External Price Feeds                                                        │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐               │
│  │Binance  │ │Coinbase │ │ Kraken  │ │ FTX     │ │ Uniswap │               │
│  │WebSocket│ │WebSocket│ │WebSocket│ │WebSocket│ │  RPC    │               │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘               │
│       └──────────┬┴──────────┬┴──────────┬┴──────────┘                      │
│                  ▼           ▼           ▼                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                     Price Aggregator DO                              │    │
│  │                                                                      │    │
│  │  Cells:                          Propagators:                        │    │
│  │  ├─ btc:binance:bid  = 42150    ├─ VWAP calculator                  │    │
│  │  ├─ btc:binance:ask  = 42152    ├─ Best bid/ask selector            │    │
│  │  ├─ btc:coinbase:bid = 42148    ├─ Spread calculator                │    │
│  │  ├─ btc:coinbase:ask = 42155    ├─ Cross-exchange arb detector      │    │
│  │  ├─ btc:vwap         = 42151    │                                    │    │
│  │  ├─ btc:best_bid     = 42150    │                                    │    │
│  │  ├─ btc:best_ask     = 42152    │                                    │    │
│  │  ├─ btc:spread       = 0.005%   │                                    │    │
│  │  └─ btc:arb_opportunity = true  │                                    │    │
│  │                                                                      │    │
│  └───────────────────────────────┬─────────────────────────────────────┘    │
│                                  │ WebSocket broadcast                       │
│                                  ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      User Portfolio DOs                              │    │
│  │                                                                      │    │
│  │  User: alice_chirho                    User: bob_chirho              │    │
│  │  ┌─────────────────────────┐          ┌─────────────────────────┐   │    │
│  │  │ Holdings:               │          │ Holdings:               │   │    │
│  │  │ ├─ btc_qty    = 2.5     │          │ ├─ btc_qty    = 0.1     │   │    │
│  │  │ ├─ eth_qty    = 15.0    │          │ ├─ eth_qty    = 100.0   │   │    │
│  │  │ ├─ usdc_qty   = 5000    │          │ ├─ sol_qty    = 500     │   │    │
│  │  │                         │          │                         │   │    │
│  │  │ Computed (auto):        │          │ Computed (auto):        │   │    │
│  │  │ ├─ btc_value  = 105375  │          │ ├─ btc_value  = 4215    │   │    │
│  │  │ ├─ eth_value  = 34500   │          │ ├─ eth_value  = 230000  │   │    │
│  │  │ ├─ total_nav  = 144875  │          │ ├─ total_nav  = 284215  │   │    │
│  │  │ ├─ pnl_24h    = +2340   │          │ ├─ pnl_24h    = -1205   │   │    │
│  │  │ ├─ pnl_pct    = +1.64%  │          │ ├─ pnl_pct    = -0.42%  │   │    │
│  │  │                         │          │                         │   │    │
│  │  │ Bidirectional:          │          │ Alerts:                 │   │    │
│  │  │ ├─ target_nav = 150000  │          │ ├─ eth_alert_price=2500 │   │    │
│  │  │ ├─ btc_needed = 42900◀──│──compute │ ├─ alert_triggered=false│   │    │
│  │  │ └─ (what BTC price for  │          │ └─ (fires when met)     │   │    │
│  │  │     target NAV?)        │          │                         │   │    │
│  │  └─────────────────────────┘          └─────────────────────────┘   │    │
│  │                                                                      │    │
│  └──────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                       Order Execution DO                             │    │
│  │                                                                      │    │
│  │  Pending Orders:                    Propagators:                     │    │
│  │  ├─ order:123:trigger_price=43000   ├─ Price threshold checker      │    │
│  │  ├─ order:123:side = "buy"          ├─ Order validator              │    │
│  │  ├─ order:123:triggered = false     ├─ Execution router             │    │
│  │  └─ order:123:fill_price = nothing  │                                │    │
│  │                                                                      │    │
│  │  When btc:best_ask ≤ order:123:trigger_price                        │    │
│  │  → order:123:triggered = true                                        │    │
│  │  → Execute trade automatically                                       │    │
│  │                                                                      │    │
│  └──────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Rust Implementation: Core Cells and Propagators

```rust
// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

use propagators_chirho::prelude_chirho::*;

/// A user's cryptocurrency portfolio as a propagator network.
pub struct CryptoPortfolioChirho {
    system_chirho: ConstraintSystemChirho,
}

impl CryptoPortfolioChirho {
    pub fn new_chirho() -> Self {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        // =========================================================
        // PRICE CELLS (fed from Price Aggregator DO)
        // =========================================================
        system_chirho.make_cell_chirho("btc_price");
        system_chirho.make_cell_chirho("eth_price");
        system_chirho.make_cell_chirho("sol_price");
        system_chirho.make_cell_chirho("usdc_price");
        system_chirho.set_exact_chirho("usdc_price", 1.0); // Stablecoin

        // =========================================================
        // HOLDING QUANTITIES (user input)
        // =========================================================
        system_chirho.make_cell_chirho("btc_qty");
        system_chirho.make_cell_chirho("eth_qty");
        system_chirho.make_cell_chirho("sol_qty");
        system_chirho.make_cell_chirho("usdc_qty");

        // Cost basis (what user paid)
        system_chirho.make_cell_chirho("btc_cost_basis");
        system_chirho.make_cell_chirho("eth_cost_basis");
        system_chirho.make_cell_chirho("sol_cost_basis");

        // =========================================================
        // COMPUTED VALUES (propagators calculate automatically)
        // =========================================================

        // Position values: value = price × quantity
        system_chirho.make_cell_chirho("btc_value");
        system_chirho.make_cell_chirho("eth_value");
        system_chirho.make_cell_chirho("sol_value");
        system_chirho.make_cell_chirho("usdc_value");

        system_chirho.add_multiplier_chirho("btc_price", "btc_qty", "btc_value");
        system_chirho.add_multiplier_chirho("eth_price", "eth_qty", "eth_value");
        system_chirho.add_multiplier_chirho("sol_price", "sol_qty", "sol_value");
        system_chirho.add_multiplier_chirho("usdc_price", "usdc_qty", "usdc_value");

        // Total NAV = sum of all position values
        system_chirho.make_cell_chirho("total_nav");
        system_chirho.make_cell_chirho("crypto_subtotal"); // BTC + ETH + SOL

        // Using linear combination: crypto_subtotal = 1*btc + 1*eth + 1*sol
        // (This would need a sum propagator - simplified here)

        // =========================================================
        // P&L CALCULATIONS
        // =========================================================

        // Unrealized P&L per asset: pnl = value - cost_basis
        system_chirho.make_cell_chirho("btc_pnl");
        system_chirho.make_cell_chirho("eth_pnl");
        system_chirho.make_cell_chirho("sol_pnl");

        system_chirho.add_subtractor_chirho("btc_value", "btc_cost_basis", "btc_pnl");
        system_chirho.add_subtractor_chirho("eth_value", "eth_cost_basis", "eth_pnl");
        system_chirho.add_subtractor_chirho("sol_value", "sol_cost_basis", "sol_pnl");

        // =========================================================
        // BIDIRECTIONAL: TARGET NAV CALCULATION
        // =========================================================
        //
        // User can set a target NAV and ask:
        // "What does BTC need to be for me to reach $200,000?"
        //
        // This is where propagators shine - we can work BACKWARDS!

        system_chirho.make_cell_chirho("target_nav");
        system_chirho.make_cell_chirho("btc_price_for_target");

        // target_nav = btc_price_for_target * btc_qty + (other_assets_value)
        // Solving for btc_price_for_target:
        // btc_price_for_target = (target_nav - other_assets_value) / btc_qty

        // =========================================================
        // ALERTS (threshold propagators)
        // =========================================================

        system_chirho.make_cell_chirho("btc_alert_threshold");
        system_chirho.make_cell_chirho("btc_alert_triggered");

        // When btc_price crosses btc_alert_threshold, btc_alert_triggered becomes 1

        Self { system_chirho }
    }

    /// Update a price from the feed.
    pub fn update_price_chirho(&self, asset_chirho: &str, price_chirho: f64) {
        let cell_name_chirho = format!("{}_price", asset_chirho);
        self.system_chirho.set_exact_chirho(&cell_name_chirho, price_chirho);
        self.system_chirho.run_chirho();
    }

    /// Set the user's holdings.
    pub fn set_holding_chirho(&self, asset_chirho: &str, qty_chirho: f64, cost_basis_chirho: f64) {
        let qty_cell_chirho = format!("{}_qty", asset_chirho);
        let cost_cell_chirho = format!("{}_cost_basis", asset_chirho);

        self.system_chirho.set_exact_chirho(&qty_cell_chirho, qty_chirho);
        self.system_chirho.set_exact_chirho(&cost_cell_chirho, cost_basis_chirho);
        self.system_chirho.run_chirho();
    }

    /// Get the current NAV.
    pub fn get_nav_chirho(&self) -> f64 {
        self.system_chirho.get_exact_chirho("total_nav").unwrap_or(0.0)
    }

    /// Get unrealized P&L for an asset.
    pub fn get_pnl_chirho(&self, asset_chirho: &str) -> f64 {
        let cell_name_chirho = format!("{}_pnl", asset_chirho);
        self.system_chirho.get_exact_chirho(&cell_name_chirho).unwrap_or(0.0)
    }

    /// BIDIRECTIONAL: Set target NAV and get required BTC price.
    pub fn price_for_target_nav_chirho(&self, target_nav_chirho: f64) -> f64 {
        self.system_chirho.set_exact_chirho("target_nav", target_nav_chirho);
        self.system_chirho.run_chirho();
        self.system_chirho.get_exact_chirho("btc_price_for_target").unwrap_or(0.0)
    }
}
```

## TypeScript Durable Object Implementation

```typescript
// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

import { PropagatorNetwork } from "propagators-chirho"; // WASM binding

interface HoldingChirho {
  asset_chirho: string;
  qty_chirho: number;
  cost_basis_chirho: number;
}

interface PriceUpdateChirho {
  asset_chirho: string;
  price_chirho: number;
  exchange_chirho: string;
  timestamp_chirho: number;
}

export class UserPortfolioDoChirho implements DurableObject {
  private state_chirho: DurableObjectState;
  private network_chirho: PropagatorNetwork | null = null;
  private subscribers_chirho: Set<WebSocket> = new Set();

  constructor(state_chirho: DurableObjectState, env_chirho: Env) {
    this.state_chirho = state_chirho;

    // Subscribe to price feed on initialization
    this.state_chirho.blockConcurrencyWhile(async () => {
      await this.initializeChirho();
      await this.subscribeToPriceFeedChirho(env_chirho);
    });
  }

  private async initializeChirho() {
    // Restore network from storage or create new
    const stored_chirho = await this.state_chirho.storage.get("network");
    if (stored_chirho) {
      this.network_chirho = PropagatorNetwork.deserialize(stored_chirho as string);
    } else {
      this.network_chirho = new PropagatorNetwork();
      this.setupPortfolioCellsChirho();
    }

    // Restore holdings
    const holdings_chirho = await this.state_chirho.storage.get("holdings") as HoldingChirho[] || [];
    for (const h_chirho of holdings_chirho) {
      this.network_chirho.setExact(`${h_chirho.asset_chirho}_qty`, h_chirho.qty_chirho);
      this.network_chirho.setExact(`${h_chirho.asset_chirho}_cost_basis`, h_chirho.cost_basis_chirho);
    }
  }

  private setupPortfolioCellsChirho() {
    const assets_chirho = ["btc", "eth", "sol", "usdc"];

    for (const asset_chirho of assets_chirho) {
      // Price cells (fed externally)
      this.network_chirho!.makeCell(`${asset_chirho}_price`);

      // Quantity cells (user input)
      this.network_chirho!.makeCell(`${asset_chirho}_qty`);
      this.network_chirho!.makeCell(`${asset_chirho}_cost_basis`);

      // Value cells (computed)
      this.network_chirho!.makeCell(`${asset_chirho}_value`);
      this.network_chirho!.addMultiplier(
        `${asset_chirho}_price`,
        `${asset_chirho}_qty`,
        `${asset_chirho}_value`
      );

      // P&L cells (computed)
      this.network_chirho!.makeCell(`${asset_chirho}_pnl`);
      this.network_chirho!.addSubtractor(
        `${asset_chirho}_value`,
        `${asset_chirho}_cost_basis`,
        `${asset_chirho}_pnl`
      );
    }

    // USDC is always $1
    this.network_chirho!.setExact("usdc_price", 1.0);

    // Total NAV
    this.network_chirho!.makeCell("total_nav");
    // (Would add a sum propagator here)

    // Alerts
    this.network_chirho!.makeCell("btc_alert_threshold");
    this.network_chirho!.makeCell("btc_alert_triggered");
  }

  private async subscribeToPriceFeedChirho(env_chirho: Env) {
    // Connect to the Price Aggregator DO
    const priceAggregatorChirho = env_chirho.PRICE_AGGREGATOR_DO.get(
      env_chirho.PRICE_AGGREGATOR_DO.idFromName("main")
    );

    const ws_chirho = await priceAggregatorChirho.fetch("wss://internal/subscribe", {
      headers: { "Upgrade": "websocket" }
    });

    // Handle price updates
    ws_chirho.addEventListener("message", (event_chirho) => {
      const update_chirho: PriceUpdateChirho = JSON.parse(event_chirho.data);
      this.handlePriceUpdateChirho(update_chirho);
    });
  }

  private async handlePriceUpdateChirho(update_chirho: PriceUpdateChirho) {
    const cell_chirho = `${update_chirho.asset_chirho}_price`;

    // Update the price cell
    this.network_chirho!.setExact(cell_chirho, update_chirho.price_chirho);

    // Run propagation - all derived values update automatically!
    this.network_chirho!.propagate();

    // Check alerts
    await this.checkAlertsChirho(update_chirho.asset_chirho);

    // Notify connected clients
    this.broadcastUpdateChirho();

    // Persist periodically (debounced)
    this.schedulePersistChirho();
  }

  private async checkAlertsChirho(asset_chirho: string) {
    const threshold_chirho = this.network_chirho!.getContent(`${asset_chirho}_alert_threshold`);
    const price_chirho = this.network_chirho!.getContent(`${asset_chirho}_price`);

    if (threshold_chirho && price_chirho) {
      const thresholdVal_chirho = threshold_chirho.lo; // Exact value
      const priceVal_chirho = price_chirho.lo;

      if (priceVal_chirho >= thresholdVal_chirho) {
        // Alert triggered!
        this.network_chirho!.setExact(`${asset_chirho}_alert_triggered`, 1.0);

        // Send push notification, email, etc.
        await this.sendAlertNotificationChirho(asset_chirho, priceVal_chirho, thresholdVal_chirho);
      }
    }
  }

  async fetch(request_chirho: Request): Promise<Response> {
    const url_chirho = new URL(request_chirho.url);

    // WebSocket for real-time updates
    if (request_chirho.headers.get("Upgrade") === "websocket") {
      const pair_chirho = new WebSocketPair();
      this.handleWebSocketChirho(pair_chirho[1]);
      return new Response(null, { status: 101, webSocket: pair_chirho[0] });
    }

    // GET /portfolio - returns full portfolio state
    if (url_chirho.pathname === "/portfolio" && request_chirho.method === "GET") {
      return Response.json(this.getPortfolioStateChirho());
    }

    // PUT /holding - update a holding
    if (url_chirho.pathname === "/holding" && request_chirho.method === "PUT") {
      const body_chirho = await request_chirho.json() as HoldingChirho;
      await this.updateHoldingChirho(body_chirho);
      return Response.json({ ok: true });
    }

    // POST /target-nav - bidirectional query
    if (url_chirho.pathname === "/target-nav" && request_chirho.method === "POST") {
      const { target_nav_chirho } = await request_chirho.json();
      const result_chirho = this.calculatePriceForTargetChirho(target_nav_chirho);
      return Response.json(result_chirho);
    }

    // PUT /alert - set price alert
    if (url_chirho.pathname === "/alert" && request_chirho.method === "PUT") {
      const { asset_chirho, threshold_chirho } = await request_chirho.json();
      this.network_chirho!.setExact(`${asset_chirho}_alert_threshold`, threshold_chirho);
      return Response.json({ ok: true });
    }

    return new Response("Not found", { status: 404 });
  }

  private getPortfolioStateChirho() {
    const assets_chirho = ["btc", "eth", "sol", "usdc"];
    const positions_chirho = [];

    for (const asset_chirho of assets_chirho) {
      positions_chirho.push({
        asset_chirho,
        qty_chirho: this.network_chirho!.getExact(`${asset_chirho}_qty`),
        price_chirho: this.network_chirho!.getExact(`${asset_chirho}_price`),
        value_chirho: this.network_chirho!.getExact(`${asset_chirho}_value`),
        cost_basis_chirho: this.network_chirho!.getExact(`${asset_chirho}_cost_basis`),
        pnl_chirho: this.network_chirho!.getExact(`${asset_chirho}_pnl`),
      });
    }

    return {
      positions_chirho,
      total_nav_chirho: this.network_chirho!.getExact("total_nav"),
      updated_at_chirho: Date.now(),
    };
  }

  private calculatePriceForTargetChirho(targetNav_chirho: number) {
    // This is the magic of propagators - we work BACKWARDS!
    //
    // User asks: "What does BTC need to be for my portfolio to hit $200K?"
    //
    // We set the target and let the propagator network solve for BTC price.

    this.network_chirho!.setExact("target_nav", targetNav_chirho);
    this.network_chirho!.propagate();

    return {
      target_nav_chirho: targetNav_chirho,
      required_btc_price_chirho: this.network_chirho!.getExact("btc_price_for_target"),
      current_btc_price_chirho: this.network_chirho!.getExact("btc_price"),
      gap_percentage_chirho: this.calculateGapChirho(),
    };
  }
}
```

## Real-Time Data Flow

```
┌──────────────────────────────────────────────────────────────────────────┐
│                            PRICE UPDATE FLOW                              │
│                                                                           │
│  T+0ms: Binance BTC = $42,150                                            │
│         │                                                                 │
│         ▼                                                                 │
│  T+5ms: Price Aggregator DO receives update                              │
│         │                                                                 │
│         ├─ btc:binance:bid = 42150                                       │
│         ├─ Propagate → btc:best_bid = 42150 (if best)                    │
│         ├─ Propagate → btc:vwap = 42148.5                                │
│         │                                                                 │
│         ▼                                                                 │
│  T+10ms: Broadcast to all subscribed User Portfolio DOs                  │
│         │                                                                 │
│         ├─────────────────────┬─────────────────────┐                    │
│         ▼                     ▼                     ▼                    │
│  T+15ms: Alice's Portfolio   Bob's Portfolio      Carol's Portfolio     │
│         │                     │                     │                    │
│         ├─ btc_price=42150    ├─ btc_price=42150    ├─ btc_price=42150   │
│         ├─ Propagate:         ├─ Propagate:         ├─ Propagate:        │
│         │  btc_value=105375   │  btc_value=4215     │  btc_value=84300   │
│         │  total_nav=144875   │  total_nav=284215   │  total_nav=92400   │
│         │  pnl=+2340          │  pnl=-1205          │  pnl=+150          │
│         │                     │                     │                    │
│         │                     │                     │ Alert check:       │
│         │                     │                     │ 42150 ≥ 42000? ✓   │
│         │                     │                     │ → ALERT TRIGGERED! │
│         │                     │                     │                    │
│         ▼                     ▼                     ▼                    │
│  T+20ms: Push to connected clients via WebSocket                         │
│                                                                           │
│  Total latency: ~20ms from exchange to user's browser                    │
│                                                                           │
└──────────────────────────────────────────────────────────────────────────┘
```

## Bidirectional Query Examples

### 1. "What BTC price for target NAV?"

```typescript
// User: "I want my portfolio to be worth $200,000. What does BTC need to hit?"

const result_chirho = await portfolioDoChirho.fetch("/target-nav", {
  method: "POST",
  body: JSON.stringify({ target_nav_chirho: 200000 })
});

// Response:
{
  "target_nav_chirho": 200000,
  "required_btc_price_chirho": 64200,
  "current_btc_price_chirho": 42150,
  "gap_percentage_chirho": 52.3
}

// The propagator network solved this BACKWARDS automatically!
```

### 2. "Break-even price?"

```typescript
// User: "What price does ETH need to be for me to break even?"

// Set target P&L to 0
network_chirho.setExact("eth_pnl", 0);
network_chirho.propagate();

// Read back the required price
const breakEvenPrice_chirho = network_chirho.getExact("eth_price_for_breakeven");
// → $1,850 (computed backwards from current holdings and cost basis)
```

### 3. "How much to buy for allocation?"

```typescript
// User: "I want BTC to be 40% of my portfolio. How much should I buy?"

network_chirho.setExact("btc_allocation_pct", 0.40);
network_chirho.propagate();

const additionalBtc_chirho = network_chirho.getExact("btc_qty_delta_for_target");
// → 0.85 BTC (computed from current holdings and prices)
```

## Scaling to 100K Users

```
┌─────────────────────────────────────────────────────────────────────┐
│                     100K User Architecture                           │
│                                                                      │
│  Price Aggregator DO (singleton)                                     │
│  └─ Handles all exchange connections                                 │
│  └─ Computes VWAP, best bid/ask                                     │
│  └─ ~100 cells, ~50 propagators                                     │
│                                                                      │
│  User Portfolio DOs (100K instances)                                 │
│  └─ Each user has their own DO                                      │
│  └─ ~50 cells per user, ~30 propagators                             │
│  └─ Hibernates when user inactive                                   │
│  └─ Wakes on price update OR user request                           │
│                                                                      │
│  Order Execution DOs (10 shards)                                    │
│  └─ Pending orders sharded by user ID                               │
│  └─ Triggers orders when conditions met                             │
│  └─ ~1K orders per shard                                            │
│                                                                      │
│  Cost Model:                                                         │
│  ├─ Price Aggregator: Always active (~$50/month)                    │
│  ├─ User Portfolios: Pay per active user                            │
│  │   └─ 10K active users × $0.001/hour = ~$250/month               │
│  └─ Order Execution: Scales with order volume                       │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Why This Beats Traditional Approaches

| Aspect | Traditional | Propagators |
|--------|-------------|-------------|
| **Price update** | Recalculate all positions | Only affected cells update |
| **NAV query** | Fetch all positions, sum | Read single cell (always current) |
| **"What-if" analysis** | Custom solver code | Set target, read result |
| **Alerts** | Poll on timer | Instant (propagator triggers) |
| **Code complexity** | Manual dependency tracking | Declarative constraints |
| **Latency** | 100-500ms | 10-30ms |
| **Cost at scale** | Linear with positions | Sublinear (shared propagation) |
