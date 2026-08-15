# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.45M | 0.005 | 191.15M | 0.043 | 6.78× | 8.29× |
| 10,000 | 0.046 | 217.51M | 0.045 | 224.38M | 0.119 | 2.59× | 2.68× |
| 100,000 | 0.450 | 222.08M | 0.436 | 229.55M | 0.951 | 2.11× | 2.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.150 | 0.124 | 0.83× |
| 1 | 5 | 0.386 | 0.482 | 1.25× |
| 1 | 10 | 0.389 | 0.990 | 2.54× |
| 10 | 1 | 0.044 | 0.093 | 2.12× |
| 10 | 5 | 0.182 | 0.423 | 2.32× |
| 10 | 10 | 0.382 | 0.955 | 2.50× |
| 100 | 1 | 0.043 | 0.103 | 2.41× |
| 100 | 5 | 0.203 | 0.461 | 2.27× |
| 100 | 10 | 0.401 | 0.974 | 2.43× |
| 1,000 | 1 | 0.048 | 0.103 | 2.16× |
| 1,000 | 5 | 0.196 | 0.495 | 2.53× |
| 1,000 | 10 | 0.498 | 1.050 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
