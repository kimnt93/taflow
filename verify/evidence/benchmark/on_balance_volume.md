# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 337.35M | 0.002 | 549.66M | 0.030 | 10.17× | 16.57× |
| 10,000 | 0.037 | 268.04M | 0.035 | 288.24M | 0.065 | 1.75× | 1.88× |
| 100,000 | 0.399 | 250.93M | 0.373 | 268.10M | 0.407 | 1.02× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.150 | 1.40× |
| 1 | 5 | 0.244 | 0.502 | 2.06× |
| 1 | 10 | 0.438 | 0.944 | 2.15× |
| 10 | 1 | 0.046 | 0.096 | 2.08× |
| 10 | 5 | 0.195 | 0.449 | 2.30× |
| 10 | 10 | 0.454 | 1.021 | 2.25× |
| 100 | 1 | 0.041 | 0.089 | 2.18× |
| 100 | 5 | 0.200 | 0.481 | 2.40× |
| 100 | 10 | 0.382 | 1.012 | 2.65× |
| 1,000 | 1 | 0.045 | 0.090 | 1.98× |
| 1,000 | 5 | 0.204 | 0.463 | 2.27× |
| 1,000 | 10 | 0.423 | 0.985 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
