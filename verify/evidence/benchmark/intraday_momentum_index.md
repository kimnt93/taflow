# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.92M | 0.017 | 60.27M | 0.093 | 5.11× | 5.61× |
| 10,000 | 0.137 | 73.21M | 0.130 | 77.10M | 0.672 | 4.92× | 5.18× |
| 100,000 | 1.316 | 75.96M | 1.342 | 74.54M | 6.355 | 4.83× | 4.74× |
| 1,000,000 | 14.094 | 70.95M | 13.379 | 74.75M | 62.113 | 4.41× | 4.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.122 | 1.20× |
| 1 | 5 | 0.370 | 0.526 | 1.42× |
| 1 | 10 | 0.511 | 0.960 | 1.88× |
| 10 | 1 | 0.050 | 0.093 | 1.86× |
| 10 | 5 | 0.233 | 0.468 | 2.01× |
| 10 | 10 | 0.537 | 0.986 | 1.84× |
| 100 | 1 | 0.054 | 0.098 | 1.82× |
| 100 | 5 | 0.227 | 0.492 | 2.16× |
| 100 | 10 | 0.538 | 1.053 | 1.96× |
| 1,000 | 1 | 0.064 | 0.149 | 2.33× |
| 1,000 | 5 | 0.253 | 0.757 | 2.99× |
| 1,000 | 10 | 0.519 | 1.611 | 3.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
