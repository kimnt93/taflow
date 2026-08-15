# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.29M | 0.021 | 47.86M | 0.059 | 2.81× | 2.84× |
| 10,000 | 0.194 | 51.57M | 0.197 | 50.71M | 0.252 | 1.30× | 1.28× |
| 100,000 | 1.967 | 50.84M | 1.911 | 52.33M | 2.198 | 1.12× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.116 | 1.42× |
| 1 | 5 | 0.224 | 0.508 | 2.27× |
| 1 | 10 | 0.391 | 0.967 | 2.47× |
| 10 | 1 | 0.046 | 0.096 | 2.07× |
| 10 | 5 | 0.239 | 0.479 | 2.00× |
| 10 | 10 | 0.399 | 0.942 | 2.36× |
| 100 | 1 | 0.044 | 0.096 | 2.19× |
| 100 | 5 | 0.191 | 0.453 | 2.37× |
| 100 | 10 | 0.467 | 0.980 | 2.10× |
| 1,000 | 1 | 0.063 | 0.113 | 1.79× |
| 1,000 | 5 | 0.224 | 0.538 | 2.41× |
| 1,000 | 10 | 0.449 | 1.197 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
