<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Crypto Portfolio Durable Object

A **pure Rust** Cloudflare Durable Object using `propagators-chirho` for real-time cryptocurrency portfolio tracking.

## Features

- **Real-time NAV**: Portfolio value updates instantly on price changes
- **Bidirectional queries**: "What BTC price for $200K NAV?"
- **Break-even calculator**: Computed automatically via propagators
- **Price alerts**: Trigger when thresholds crossed
- **Persistent state**: Holdings survive hibernation

## API

### Portfolio State
```bash
GET /user/:user_id/portfolio

# Response:
{
  "positions_chirho": [
    {
      "asset_chirho": "btc",
      "qty_chirho": 2.5,
      "price_chirho": 42150,
      "value_chirho": 105375,
      "pnl_chirho": 5375,
      "pnl_pct_chirho": 5.37
    }
  ],
  "total_nav_chirho": 150000,
  "total_pnl_chirho": 8500
}
```

### Update Price
```bash
PUT /user/:user_id/price
{"asset_chirho": "btc", "price_chirho": 43000}
```

### Update Holding
```bash
PUT /user/:user_id/holding
{
  "asset_chirho": "btc",
  "qty_chirho": 2.5,
  "cost_basis_chirho": 100000
}
```

### Target NAV Query (Bidirectional!)
```bash
POST /user/:user_id/target-nav
{"target_nav_chirho": 200000}

# Response:
{
  "target_nav_chirho": 200000,
  "required_btc_price_chirho": 64200,
  "current_btc_price_chirho": 42150,
  "gap_pct_chirho": 52.3
}
```

### Break-Even Price
```bash
POST /user/:user_id/break-even
{"asset_chirho": "eth"}

# Response:
{
  "asset_chirho": "eth",
  "break_even_price_chirho": 1850,
  "current_price_chirho": 2300,
  "gap_pct_chirho": -19.6
}
```

### Set Alert
```bash
PUT /user/:user_id/alert
{
  "asset_chirho": "btc",
  "threshold_chirho": 50000,
  "direction_chirho": "above"
}
```

## Development

```bash
# Install wrangler
npm install -g wrangler

# Login to Cloudflare
wrangler login

# Run locally
wrangler dev

# Deploy
wrangler deploy
```

## How It Works

Each user gets their own Durable Object containing a propagator network:

```
Price Update ($42,150)
       │
       ▼
   btc_price cell
       │
       ▼ (multiplier propagator)
   btc_value = btc_price × btc_qty
       │
       ▼ (subtractor propagator)
   btc_pnl = btc_value - btc_cost_basis
       │
       ▼ (aggregator)
   total_nav = Σ asset_values
```

All derived values update automatically when any input changes.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  Cloudflare Edge                     │
│                                                      │
│  Request: PUT /user/alice/price {btc: 42150}        │
│                     │                                │
│                     ▼                                │
│  ┌─────────────────────────────────────────────┐    │
│  │         CryptoPortfolioDoChirho             │    │
│  │                                             │    │
│  │  ┌─────────────────────────────────────┐   │    │
│  │  │      Propagator Network (Rust)      │   │    │
│  │  │                                     │   │    │
│  │  │  btc_price ──┬──▶ btc_value ──────▶ │   │    │
│  │  │  btc_qty ────┘         │            │   │    │
│  │  │                        ▼            │   │    │
│  │  │  btc_cost_basis ──▶ btc_pnl        │   │    │
│  │  │                        │            │   │    │
│  │  │                        ▼            │   │    │
│  │  │              total_nav, total_pnl   │   │    │
│  │  └─────────────────────────────────────┘   │    │
│  │                     │                       │    │
│  │                     ▼                       │    │
│  │  ┌─────────────────────────────────────┐   │    │
│  │  │      Durable Storage (K/V)          │   │    │
│  │  │  holdings: {...}                    │   │    │
│  │  │  alerts: [...]                      │   │    │
│  │  │  prices: {...}                      │   │    │
│  │  └─────────────────────────────────────┘   │    │
│  └─────────────────────────────────────────────┘    │
│                                                      │
└─────────────────────────────────────────────────────┘
```
