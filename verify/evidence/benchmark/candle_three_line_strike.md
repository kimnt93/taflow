# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.90M | 0.010 | 96.06M | 0.035 | 2.45× | 3.32× |
| 10,000 | 0.079 | 126.27M | 0.073 | 136.59M | 0.119 | 1.50× | 1.62× |
| 100,000 | 0.874 | 114.42M | 0.841 | 118.88M | 0.874 | 1.00× | 1.04× |
| 1,000,000 | 9.010 | 110.98M | 8.885 | 112.55M | 8.899 | 0.99× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.128 | 0.85× |
| 1 | 5 | 0.348 | 0.501 | 1.44× |
| 1 | 10 | 0.558 | 0.903 | 1.62× |
| 10 | 1 | 0.055 | 0.092 | 1.66× |
| 10 | 5 | 0.251 | 0.432 | 1.72× |
| 10 | 10 | 0.600 | 1.091 | 1.82× |
| 100 | 1 | 0.061 | 0.093 | 1.51× |
| 100 | 5 | 0.274 | 0.444 | 1.62× |
| 100 | 10 | 0.578 | 1.034 | 1.79× |
| 1,000 | 1 | 0.072 | 0.096 | 1.34× |
| 1,000 | 5 | 0.273 | 0.473 | 1.73× |
| 1,000 | 10 | 0.576 | 1.071 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
