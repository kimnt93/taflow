# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.40M | 0.009 | 115.87M | 0.038 | 3.02× | 4.35× |
| 10,000 | 0.074 | 135.47M | 0.068 | 146.43M | 0.090 | 1.21× | 1.31× |
| 100,000 | 0.739 | 135.39M | 0.662 | 151.12M | 0.563 | 0.76× | 0.85× |
| 1,000,000 | 6.902 | 144.88M | 6.433 | 155.45M | 5.621 | 0.81× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.122 | 1.44× |
| 1 | 5 | 0.276 | 0.552 | 2.00× |
| 1 | 10 | 0.638 | 1.065 | 1.67× |
| 10 | 1 | 0.062 | 0.094 | 1.53× |
| 10 | 5 | 0.272 | 0.441 | 1.62× |
| 10 | 10 | 0.584 | 1.029 | 1.76× |
| 100 | 1 | 0.080 | 0.106 | 1.34× |
| 100 | 5 | 0.281 | 0.475 | 1.69× |
| 100 | 10 | 0.569 | 1.139 | 2.00× |
| 1,000 | 1 | 0.092 | 0.126 | 1.37× |
| 1,000 | 5 | 0.328 | 0.519 | 1.58× |
| 1,000 | 10 | 0.599 | 1.068 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
