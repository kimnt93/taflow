# ADX — benchmark vs TA-Lib 0.7.1

Correctness: **PASS** @100,000 bars — batch vs talib: max abs err 3.55e-14, 0 NaN mismatches; state vs talib: max abs err 3.55e-14, 0 NaN mismatches; chunk replay bitwise-identical

## Bulk compute (mean seconds per call, 20 repeats)

| Bars | TA-Lib ms | TAFlow ms | Speedup | TAFlow ops/s | State-cold ms | Speedup |
|---:|---:|---:|---:|---:|---:|---:|
| 100 | 0.0016 | 0.0018 | 0.90× | 56.8M | 0.0023 | 0.68× |
| 1,000 | 0.0088 | 0.0087 | 1.01× | 115.1M | 0.0145 | 0.60× |
| 10,000 | 0.0900 | 0.0857 | 1.05× | 116.6M | 0.1376 | 0.65× |
| 100,000 | 0.9310 | 0.7936 | 1.17× | 126.0M | 1.3287 | 0.70× |
| 1,000,000 | 9.8809 | 8.1625 | 1.21× | 122.5M | 13.3010 | 0.74× |

## Live continuation (latency per update; TA-Lib = full recompute of base+chunk)

| Base | Chunk | TAFlow µs | TA-Lib µs | Speedup | Tail-window µs | vs tail | TAFlow bars/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100 | 1 | 0.22 | 1.7 | 7.47× | 1.10 | 4.91× | 4.5M |
| 100 | 10 | 0.78 | 1.8 | 2.25× | 1.16 | 1.48× | 12.8M |
| 100 | 100 | 2.48 | 2.6 | 1.06× | 1.98 | 0.80× | 40.3M |
| 100 | 1,000 | 20.27 | 10.2 | 0.51× | 9.90 | 0.49× | 49.3M |
| 1,000 | 1 | 0.21 | 8.9 | 42.36× | 1.25 | 5.95× | 4.8M |
| 1,000 | 10 | 0.79 | 8.5 | 10.81× | 1.14 | 1.45× | 12.7M |
| 1,000 | 100 | 2.54 | 9.9 | 3.91× | 1.87 | 0.74× | 39.3M |
| 1,000 | 1,000 | 20.47 | 16.1 | 0.79× | 9.31 | 0.45× | 48.9M |
| 10,000 | 1 | 0.31 | 90.6 | 294× | 1.04 | 3.37× | 3.3M |
| 10,000 | 10 | 0.73 | 88.8 | 122× | 1.15 | 1.58× | 13.8M |
| 10,000 | 100 | 2.56 | 91.1 | 35.54× | 1.83 | 0.71× | 39.0M |
| 10,000 | 1,000 | 18.36 | 100.3 | 5.46× | 11.02 | 0.60× | 54.5M |
| 100,000 | 1 | 0.23 | 867.0 | 3,812× | 1.38 | 6.08× | 4.4M |
| 100,000 | 10 | 0.72 | 966.8 | 1,334× | 1.14 | 1.57× | 13.8M |
| 100,000 | 100 | 2.53 | 956.2 | 377× | 1.83 | 0.72× | 39.5M |
| 100,000 | 1,000 | 19.04 | 971.5 | 51.01× | 9.04 | 0.47× | 52.5M |
| 1,000,000 | 1 | 0.22 | 9445.0 | 43,885× | 1.04 | 4.84× | 4.6M |
| 1,000,000 | 10 | 0.75 | 9676.7 | 12,858× | 1.14 | 1.51× | 13.3M |
| 1,000,000 | 100 | 2.61 | 9228.8 | 3,534× | 1.84 | 0.70× | 38.3M |
| 1,000,000 | 1,000 | 18.71 | 10056.7 | 537× | 9.46 | 0.51× | 53.4M |

Append latency (base 1,000,000, chunk 1): p50 0.21 µs, p99 0.25 µs.

## Parallel continuation (100,000-bar warmed history per thread, one independent stream per thread)

| Threads | TAFlow agg updates/s | Scaling | TA-Lib agg updates/s | Scaling | Speedup |
|---:|---:|---:|---:|---:|---:|
| 1 | 6.2M | 1.00× | 1.1K | 1.00× | 5,792× |
| 2 | 7.5M | 1.20× | 1.0K | 0.93× | 7,473× |
| 5 | 7.8M | 1.24× | 1.0K | 0.96× | 7,509× |
| 10 | 6.9M | 1.11× | 1.0K | 0.95× | 6,782× |
| 20 | 7.5M | 1.20× | 1.0K | 0.96× | 7,198× |

Each thread owns its own state/stream (N-symbol live feed model). Scaling >1× with threads requires the underlying call to release the GIL.

---
Python-interface measurement: numbers include conversion/boundary overhead by design. Rust-core-only numbers live in criterion benches and are not comparable.
