<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Large-Scale Propagator Architecture with Cloudflare

## The Challenge

You have:
- **10,000+ devices** sending sensor data
- **1M+ cells** with interdependencies
- **100K+ concurrent users** viewing dashboards
- **Complex derived metrics** (averages, alerts, predictions)
- **Real-time requirements** (<100ms latency)

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Cloudflare Edge                                   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                         Edge Workers                                 │    │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐   │    │
│  │  │ Ingest  │  │ Ingest  │  │  Query  │  │  Query  │  │   WS    │   │    │
│  │  │ Worker  │  │ Worker  │  │ Worker  │  │ Worker  │  │ Worker  │   │    │
│  │  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘   │    │
│  └───────┼───────────┼───────────┼───────────┼───────────────┼────────┘    │
│          │           │           │           │               │              │
│          ▼           ▼           ▼           ▼               ▼              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                       Router DO (singleton)                          │    │
│  │  ┌─────────────────────────────────────────────────────────────┐    │    │
│  │  │  Cell Registry: which cells live on which Propagator DO     │    │    │
│  │  │  device:abc123:temp → PropagatorDO("shard-device-abc")      │    │    │
│  │  │  zone:west:avg_temp → PropagatorDO("shard-zone-west")       │    │    │
│  │  └─────────────────────────────────────────────────────────────┘    │    │
│  └──────────────────────────────┬──────────────────────────────────────┘    │
│                                 │                                            │
│          ┌──────────────────────┼──────────────────────┐                    │
│          ▼                      ▼                      ▼                    │
│  ┌───────────────┐      ┌───────────────┐      ┌───────────────┐           │
│  │ PropagatorDO  │      │ PropagatorDO  │      │ PropagatorDO  │           │
│  │ (shard-001)   │◀────▶│ (shard-002)   │◀────▶│ (shard-003)   │           │
│  │               │      │               │      │               │           │
│  │ ~10K cells    │      │ ~10K cells    │      │ ~10K cells    │           │
│  │ ~1K propagators      │ ~1K propagators      │ ~1K propagators           │
│  │               │      │               │      │               │           │
│  │ Local compute │      │ Local compute │      │ Local compute │           │
│  │ + cross-shard │      │ + cross-shard │      │ + cross-shard │           │
│  │   edges       │      │   edges       │      │   edges       │           │
│  └───────────────┘      └───────────────┘      └───────────────┘           │
│          │                      │                      │                    │
│          ▼                      ▼                      ▼                    │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    Aggregator DOs (per-zone)                         │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                  │    │
│  │  │ Zone West   │  │ Zone East   │  │ Zone Global │                  │    │
│  │  │ Aggregator  │  │ Aggregator  │  │ Aggregator  │                  │    │
│  │  │             │  │             │  │             │                  │    │
│  │  │ avg, min,   │  │ avg, min,   │  │ global      │                  │    │
│  │  │ max, alerts │  │ max, alerts │  │ dashboard   │                  │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                  │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Sharding Strategy

### Level 1: Device Shards (10K cells each)

Each device's sensors go to a single PropagatorDO:

```typescript
function getDeviceShardChirho(deviceIdChirho: string): DurableObjectId {
  // Consistent hashing - same device always goes to same shard
  const hashChirho = await crypto.subtle.digest(
    "SHA-256",
    new TextEncoder().encode(deviceIdChirho)
  );
  const shardIndexChirho = new Uint32Array(hashChirho)[0] % SHARD_COUNT_CHIRHO;
  return env.PROPAGATOR_DO.idFromName(`shard-${shardIndexChirho}`);
}
```

**Why this works:**
- Device cells are co-located (temp, humidity, pressure all in same DO)
- Intra-device propagation is instant (same DO)
- ~100 devices × ~100 cells = ~10K cells per shard

### Level 2: Zone Aggregators

Aggregator DOs subscribe to device shards for their zone:

```typescript
// In AggregatorDO
async initializeChirho() {
  // Get all device shards for this zone
  const shardsChirho = await this.routerChirho.getShardsForZoneChirho(this.zoneChirho);

  for (const shardIdChirho of shardsChirho) {
    const shardChirho = env.PROPAGATOR_DO.get(shardIdChirho);

    // Subscribe via WebSocket for real-time updates
    const wsChirho = await shardChirho.fetch("/ws", {
      headers: { "Upgrade": "websocket" }
    });

    // Subscribe to all temperature cells in this zone
    wsChirho.send(JSON.stringify({
      type: "SubscribeChirho",
      cells_chirho: [`zone:${this.zoneChirho}:*:temperature`]
    }));
  }
}

// When we receive updates, propagate to zone-level aggregates
handleUpdateChirho(cellChirho: string, contentChirho: NumericInfo) {
  // Update local propagator network
  this.networkChirho.addInfoChirho(cellChirho, contentChirho);

  // Zone propagators automatically compute:
  // - zone:west:avg_temperature
  // - zone:west:max_temperature
  // - zone:west:alert_count
}
```

### Level 3: Global Aggregator

Subscribes to zone aggregators for global metrics:

```typescript
// Global dashboard cells
// - global:total_devices
// - global:avg_temperature
// - global:active_alerts
// - global:data_rate_per_second
```

## Cross-Shard Propagation

When a propagator spans multiple shards (rare but needed):

```rust
// In PropagatorDO
pub struct CrossShardEdgeChirho {
    /// Local cell that triggers the edge
    source_cell_chirho: String,
    /// Remote shard that owns the target cell
    target_shard_chirho: String,
    /// Cell on the remote shard
    target_cell_chirho: String,
    /// The constraint type
    constraint_chirho: ConstraintTypeChirho,
}

impl PropagatorDoChirho {
    async fn handle_local_update_chirho(&mut self, cell_chirho: &str) {
        // First, run local propagation
        self.network_chirho.propagate_chirho();

        // Check for cross-shard edges
        for edge_chirho in self.cross_shard_edges_chirho.iter() {
            if edge_chirho.source_cell_chirho == cell_chirho {
                // Send update to remote shard
                let content_chirho = self.network_chirho.get_content_chirho(cell_chirho);

                self.send_to_shard_chirho(
                    &edge_chirho.target_shard_chirho,
                    CrossShardUpdateChirho {
                        cell_chirho: edge_chirho.target_cell_chirho.clone(),
                        content_chirho,
                        constraint_chirho: edge_chirho.constraint_chirho.clone(),
                    }
                ).await;
            }
        }
    }
}
```

## Data Flow Example

**Scenario:** Device ABC123 reports temperature = 25°C

```
1. Device → Ingest Worker
   POST /ingest { device: "abc123", temp: 25.0 }

2. Ingest Worker → Router DO
   "Which shard owns device:abc123:temperature?"
   Response: "shard-042"

3. Ingest Worker → PropagatorDO(shard-042)
   PUT /cell/device:abc123:temperature { value: 25.0 }

4. PropagatorDO(shard-042) internal propagation:
   - device:abc123:temperature = 25.0
   - device:abc123:fahrenheit = 77.0  (computed)
   - device:abc123:comfort = 0.85     (computed from temp + humidity)
   - device:abc123:alert = false      (computed: temp in safe range)

5. PropagatorDO → Zone Aggregator (via WebSocket)
   { cell: "device:abc123:temperature", content: { lo: 25, hi: 25 } }

6. Zone Aggregator internal propagation:
   - zone:west:temp_sum += 25.0
   - zone:west:temp_count += 1
   - zone:west:avg_temperature = sum/count = 24.8
   - zone:west:max_temperature = max(25.0, prev_max) = 28.3

7. Zone Aggregator → Global Aggregator (via WebSocket)
   { cell: "zone:west:avg_temperature", content: { lo: 24.8, hi: 24.8 } }

8. Global Aggregator → Dashboard WebSockets
   All connected dashboards receive instant update

Total latency: ~20-50ms edge-to-dashboard
```

## Persistence Strategy

### Per-Cell Persistence (Simple, More I/O)

```typescript
async updateCellChirho(cellChirho: string, valueChirho: number) {
  this.networkChirho.setExactChirho(cellChirho, valueChirho);

  // Persist immediately
  await this.state.storage.put(
    `cell:${cellChirho}`,
    this.networkChirho.getCellStateChirho(cellChirho)
  );
}
```

### Batch Persistence (Recommended for High Volume)

```typescript
private pendingWritesChirho: Map<string, CellState> = new Map();
private flushTimerChirho: number | null = null;

async updateCellChirho(cellChirho: string, valueChirho: number) {
  this.networkChirho.setExactChirho(cellChirho, valueChirho);

  // Queue for batch write
  this.pendingWritesChirho.set(cellChirho, this.networkChirho.getCellStateChirho(cellChirho));

  // Debounce flush
  if (!this.flushTimerChirho) {
    this.flushTimerChirho = setTimeout(() => this.flushChirho(), 50);
  }
}

async flushChirho() {
  if (this.pendingWritesChirho.size === 0) return;

  // Batch write all pending cells
  const entriesChirho = Array.from(this.pendingWritesChirho.entries())
    .map(([k, v]) => [`cell:${k}`, v]);

  await this.state.storage.put(Object.fromEntries(entriesChirho));

  this.pendingWritesChirho.clear();
  this.flushTimerChirho = null;
}
```

### Snapshot Persistence (For Recovery)

```typescript
// Every N updates or every M minutes
async snapshotChirho() {
  const snapshotChirho = {
    version_chirho: this.versionChirho++,
    timestamp_chirho: Date.now(),
    network_chirho: this.networkChirho.serializeChirho(),
    clock_chirho: this.logicalClockChirho,
  };

  await this.state.storage.put("snapshot:latest", snapshotChirho);

  // Keep last 3 snapshots for rollback
  await this.state.storage.put(`snapshot:${snapshotChirho.version_chirho}`, snapshotChirho);
}
```

## Hibernation Handling

DOs hibernate when idle. Handle gracefully:

```typescript
export class PropagatorDoChirho implements DurableObject {
  private networkChirho: PropagatorNetwork | null = null;
  private loadedCellsChirho: Set<string> = new Set();

  async ensureLoadedChirho() {
    if (this.networkChirho) return;

    // Restore from snapshot
    const snapshotChirho = await this.state.storage.get("snapshot:latest");
    if (snapshotChirho) {
      this.networkChirho = PropagatorNetwork.deserializeChirho(snapshotChirho.network_chirho);
    } else {
      this.networkChirho = new PropagatorNetwork();
    }
  }

  async getCellChirho(cellChirho: string): Promise<NumericInfo> {
    await this.ensureLoadedChirho();

    // Lazy load individual cells if not in snapshot
    if (!this.loadedCellsChirho.has(cellChirho)) {
      const storedChirho = await this.state.storage.get(`cell:${cellChirho}`);
      if (storedChirho) {
        this.networkChirho.restoreCellChirho(cellChirho, storedChirho);
      }
      this.loadedCellsChirho.add(cellChirho);
    }

    return this.networkChirho.getContentChirho(cellChirho);
  }
}
```

## Scaling Numbers

| Component | Count | Cells | Memory | Storage |
|-----------|-------|-------|--------|---------|
| Device Shards | 100 | 10K each | ~50MB | ~10MB |
| Zone Aggregators | 10 | 1K each | ~5MB | ~1MB |
| Global Aggregator | 1 | 100 | ~1MB | ~100KB |
| **Total** | 111 DOs | ~1M cells | ~5GB | ~1GB |

## Cost Optimization

1. **Hibernation**: DOs sleep when inactive (pay only for active time)
2. **Batch writes**: 50ms debounce reduces storage ops by 10-100x
3. **Intervals vs exact**: Use intervals when precision isn't needed (smaller payloads)
4. **WebSocket over polling**: One connection vs repeated requests
5. **Local propagation**: 99% of computation stays within a single DO

## Failure Handling

### DO Crash/Restart

```typescript
// On wake, validate consistency
async validateChirho() {
  const snapshotChirho = await this.state.storage.get("snapshot:latest");
  const cellKeysChirho = await this.state.storage.list({ prefix: "cell:" });

  for (const [keyChirho, storedChirho] of cellKeysChirho) {
    const inSnapshotChirho = this.networkChirho.hasCell(keyChirho);
    if (!inSnapshotChirho) {
      // Cell was added after last snapshot - restore it
      this.networkChirho.restoreCellChirho(keyChirho, storedChirho);
    }
  }

  // Re-run propagation to ensure consistency
  this.networkChirho.propagateChirho();
}
```

### Cross-Shard Partition

```typescript
// If a shard is unreachable, queue updates
private pendingCrossShardChirho: Map<string, CrossShardUpdate[]> = new Map();

async sendToShardChirho(shardChirho: string, updateChirho: CrossShardUpdate) {
  try {
    await this.doStubChirho.fetch("/update", { body: JSON.stringify(updateChirho) });
  } catch (e) {
    // Queue for retry
    const queueChirho = this.pendingCrossShardChirho.get(shardChirho) || [];
    queueChirho.push(updateChirho);
    this.pendingCrossShardChirho.set(shardChirho, queueChirho);

    // Schedule retry
    this.scheduleRetryChirho(shardChirho);
  }
}
```

## When NOT to Use This

- **Write-heavy with low read**: Traditional database might be cheaper
- **No derived computations**: If you just store/retrieve, skip propagators
- **Strict transactions**: Propagators are eventually consistent
- **Very large individual cells**: Propagators work best with many small cells
