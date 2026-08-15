# TripleTopBottom benchmark (`TripleTopBottom` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.49M | 0.007 | 150.45M | 0.228 | 25.02× | 34.37× |
| 10,000 | 0.093 | 107.74M | 0.089 | 112.66M | 1.357 | 14.62× | 15.29× |
| 100,000 | 0.885 | 113.05M | 0.879 | 113.81M | 12.813 | 14.49× | 14.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.200 | 1.57× |
| 1 | 5 | 0.262 | 0.845 | 3.23× |
| 1 | 10 | 0.387 | 1.620 | 4.19× |
| 10 | 1 | 0.042 | 0.162 | 3.89× |
| 10 | 5 | 0.190 | 1.136 | 5.98× |
| 10 | 10 | 0.380 | 1.707 | 4.50× |
| 100 | 1 | 0.041 | 0.174 | 4.22× |
| 100 | 5 | 0.215 | 1.256 | 5.85× |
| 100 | 10 | 0.431 | 1.778 | 4.12× |
| 1,000 | 1 | 0.051 | 0.295 | 5.73× |
| 1,000 | 5 | 0.240 | 1.817 | 7.55× |
| 1,000 | 10 | 0.427 | 3.050 | 7.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
