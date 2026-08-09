# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.33M | 0.026 | 38.54M | 0.057 | 2.19× | 2.20× |
| 10,000 | 0.213 | 47.02M | 0.228 | 43.93M | 0.266 | 1.25× | 1.17× |
| 100,000 | 2.282 | 43.82M | 2.373 | 42.15M | 2.262 | 0.99× | 0.95× |
| 1,000,000 | 21.188 | 47.20M | 20.629 | 48.48M | 22.629 | 1.07× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.135 | 1.89× |
| 1 | 5 | 0.272 | 0.530 | 1.95× |
| 1 | 10 | 0.496 | 0.950 | 1.92× |
| 10 | 1 | 0.053 | 0.094 | 1.80× |
| 10 | 5 | 0.261 | 0.504 | 1.93× |
| 10 | 10 | 0.537 | 0.956 | 1.78× |
| 100 | 1 | 0.053 | 0.092 | 1.74× |
| 100 | 5 | 0.248 | 0.467 | 1.89× |
| 100 | 10 | 0.601 | 1.137 | 1.89× |
| 1,000 | 1 | 0.074 | 0.118 | 1.60× |
| 1,000 | 5 | 0.288 | 0.643 | 2.23× |
| 1,000 | 10 | 0.615 | 1.333 | 2.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
