# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.160 | 6.23M | 0.144 | 6.95M | 0.045 | 0.28× | 0.31× |
| 10,000 | 1.319 | 7.58M | 1.356 | 7.38M | 0.140 | 0.11× | 0.10× |
| 100,000 | 14.136 | 7.07M | 13.472 | 7.42M | 1.019 | 0.07× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.144 | 1.13× |
| 1 | 5 | 0.535 | 0.552 | 1.03× |
| 1 | 10 | 0.840 | 1.048 | 1.25× |
| 10 | 1 | 0.084 | 0.101 | 1.19× |
| 10 | 5 | 0.385 | 0.505 | 1.31× |
| 10 | 10 | 0.790 | 1.043 | 1.32× |
| 100 | 1 | 0.100 | 0.109 | 1.09× |
| 100 | 5 | 0.370 | 0.520 | 1.41× |
| 100 | 10 | 0.824 | 1.059 | 1.29× |
| 1,000 | 1 | 0.238 | 0.118 | 0.50× |
| 1,000 | 5 | 0.487 | 0.574 | 1.18× |
| 1,000 | 10 | 0.864 | 1.160 | 1.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
