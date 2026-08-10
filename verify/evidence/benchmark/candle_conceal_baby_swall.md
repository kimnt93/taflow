# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.71M | 0.018 | 56.50M | 0.034 | 1.82× | 1.91× |
| 10,000 | 0.121 | 82.85M | 0.135 | 74.02M | 0.088 | 0.73× | 0.65× |
| 100,000 | 1.181 | 84.68M | 1.264 | 79.12M | 0.685 | 0.58× | 0.54× |
| 1,000,000 | 12.215 | 81.87M | 13.140 | 76.10M | 6.270 | 0.51× | 0.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.123 | 0.91× |
| 1 | 5 | 0.366 | 0.466 | 1.28× |
| 1 | 10 | 0.499 | 0.874 | 1.75× |
| 10 | 1 | 0.057 | 0.102 | 1.77× |
| 10 | 5 | 0.240 | 0.443 | 1.84× |
| 10 | 10 | 0.536 | 0.938 | 1.75× |
| 100 | 1 | 0.057 | 0.090 | 1.56× |
| 100 | 5 | 0.258 | 0.427 | 1.66× |
| 100 | 10 | 0.528 | 0.887 | 1.68× |
| 1,000 | 1 | 0.071 | 0.096 | 1.34× |
| 1,000 | 5 | 0.256 | 0.451 | 1.76× |
| 1,000 | 10 | 0.564 | 0.983 | 1.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
