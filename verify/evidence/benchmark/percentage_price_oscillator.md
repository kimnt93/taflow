# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.70M | 0.009 | 113.94M | 0.049 | 4.85× | 5.60× |
| 10,000 | 0.056 | 177.85M | 0.061 | 165.17M | 0.099 | 1.77× | 1.64× |
| 100,000 | 0.496 | 201.68M | 0.456 | 219.15M | 0.648 | 1.31× | 1.42× |
| 1,000,000 | 5.374 | 186.07M | 4.744 | 210.81M | 6.256 | 1.16× | 1.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.125 | 1.12× |
| 1 | 5 | 0.291 | 0.577 | 1.98× |
| 1 | 10 | 0.552 | 1.051 | 1.91× |
| 10 | 1 | 0.048 | 0.100 | 2.07× |
| 10 | 5 | 0.275 | 0.605 | 2.20× |
| 10 | 10 | 0.629 | 1.100 | 1.75× |
| 100 | 1 | 0.055 | 0.113 | 2.05× |
| 100 | 5 | 0.279 | 0.590 | 2.11× |
| 100 | 10 | 0.610 | 1.132 | 1.85× |
| 1,000 | 1 | 0.055 | 0.107 | 1.94× |
| 1,000 | 5 | 0.256 | 0.556 | 2.17× |
| 1,000 | 10 | 0.608 | 1.117 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
