# RollingInterquartileRange benchmark (`RollingIqr` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.97M | 0.046 | 21.53M | 0.301 | 5.71× | 6.49× |
| 10,000 | 0.520 | 19.24M | 0.516 | 19.36M | 1.686 | 3.24× | 3.27× |
| 100,000 | 5.168 | 19.35M | 5.071 | 19.72M | 15.704 | 3.04× | 3.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.238 | 2.70× |
| 1 | 5 | 0.350 | 1.093 | 3.12× |
| 1 | 10 | 0.413 | 2.627 | 6.36× |
| 10 | 1 | 0.050 | 0.213 | 4.24× |
| 10 | 5 | 0.195 | 1.057 | 5.43× |
| 10 | 10 | 0.408 | 2.290 | 5.62× |
| 100 | 1 | 0.051 | 0.231 | 4.56× |
| 100 | 5 | 0.207 | 1.442 | 6.98× |
| 100 | 10 | 0.423 | 3.154 | 7.46× |
| 1,000 | 1 | 0.107 | 0.383 | 3.57× |
| 1,000 | 5 | 0.216 | 2.196 | 10.18× |
| 1,000 | 10 | 0.473 | 4.035 | 8.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
