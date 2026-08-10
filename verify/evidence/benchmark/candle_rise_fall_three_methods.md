# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 29.85M | 0.022 | 45.66M | 0.038 | 1.15× | 1.75× |
| 10,000 | 0.195 | 51.27M | 0.184 | 54.32M | 0.128 | 0.66× | 0.70× |
| 100,000 | 1.925 | 51.94M | 2.085 | 47.96M | 1.021 | 0.53× | 0.49× |
| 1,000,000 | 19.153 | 52.21M | 19.151 | 52.22M | 10.486 | 0.55× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.117 | 0.74× |
| 1 | 5 | 0.293 | 0.541 | 1.85× |
| 1 | 10 | 0.602 | 0.961 | 1.60× |
| 10 | 1 | 0.066 | 0.093 | 1.42× |
| 10 | 5 | 0.258 | 0.455 | 1.77× |
| 10 | 10 | 0.577 | 0.949 | 1.64× |
| 100 | 1 | 0.062 | 0.090 | 1.47× |
| 100 | 5 | 0.266 | 0.466 | 1.75× |
| 100 | 10 | 0.599 | 1.104 | 1.84× |
| 1,000 | 1 | 0.088 | 0.113 | 1.29× |
| 1,000 | 5 | 0.289 | 0.553 | 1.91× |
| 1,000 | 10 | 0.597 | 1.111 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
