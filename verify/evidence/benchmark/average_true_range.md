# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.12M | 0.011 | 91.15M | 0.044 | 3.50× | 3.99× |
| 10,000 | 0.077 | 130.53M | 0.074 | 134.24M | 0.099 | 1.30× | 1.33× |
| 100,000 | 0.729 | 137.20M | 0.689 | 145.04M | 0.645 | 0.88× | 0.94× |
| 1,000,000 | 7.694 | 129.97M | 7.042 | 142.00M | 6.624 | 0.86× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.105 | 1.62× |
| 1 | 5 | 0.347 | 0.533 | 1.53× |
| 1 | 10 | 0.498 | 0.962 | 1.93× |
| 10 | 1 | 0.048 | 0.091 | 1.88× |
| 10 | 5 | 0.227 | 0.457 | 2.01× |
| 10 | 10 | 0.531 | 0.987 | 1.86× |
| 100 | 1 | 0.051 | 0.095 | 1.85× |
| 100 | 5 | 0.259 | 0.466 | 1.80× |
| 100 | 10 | 0.494 | 1.147 | 2.32× |
| 1,000 | 1 | 0.060 | 0.108 | 1.79× |
| 1,000 | 5 | 0.327 | 0.561 | 1.71× |
| 1,000 | 10 | 0.544 | 1.203 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
