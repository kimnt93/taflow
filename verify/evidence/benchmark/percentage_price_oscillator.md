# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.46M | 0.036 | 28.14M | 0.039 | 1.00× | 1.10× |
| 10,000 | 0.254 | 39.33M | 0.269 | 37.20M | 0.084 | 0.33× | 0.31× |
| 100,000 | 2.401 | 41.66M | 2.325 | 43.00M | 0.500 | 0.21× | 0.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.141 | 0.94× |
| 1 | 5 | 0.456 | 0.599 | 1.31× |
| 1 | 10 | 0.811 | 1.006 | 1.24× |
| 10 | 1 | 0.074 | 0.099 | 1.34× |
| 10 | 5 | 0.349 | 0.470 | 1.35× |
| 10 | 10 | 0.732 | 0.997 | 1.36× |
| 100 | 1 | 0.076 | 0.101 | 1.32× |
| 100 | 5 | 0.372 | 0.460 | 1.24× |
| 100 | 10 | 0.730 | 0.983 | 1.35× |
| 1,000 | 1 | 0.098 | 0.104 | 1.05× |
| 1,000 | 5 | 0.347 | 0.507 | 1.46× |
| 1,000 | 10 | 0.829 | 1.035 | 1.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
