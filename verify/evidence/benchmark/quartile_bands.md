# QuartileBands benchmark (`QuartileBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.401 | 2.49M | 0.391 | 2.56M | 0.684 | 1.71× | 1.75× |
| 10,000 | 3.912 | 2.56M | 3.889 | 2.57M | 5.412 | 1.38× | 1.39× |
| 100,000 | 39.808 | 2.51M | 39.535 | 2.53M | 54.996 | 1.38× | 1.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.272 | 2.51× |
| 1 | 5 | 0.453 | 1.129 | 2.49× |
| 1 | 10 | 0.630 | 2.274 | 3.61× |
| 10 | 1 | 0.070 | 0.217 | 3.08× |
| 10 | 5 | 0.293 | 1.295 | 4.42× |
| 10 | 10 | 0.653 | 2.351 | 3.60× |
| 100 | 1 | 0.110 | 0.271 | 2.47× |
| 100 | 5 | 0.304 | 1.512 | 4.98× |
| 100 | 10 | 0.676 | 3.133 | 4.63× |
| 1,000 | 1 | 0.504 | 0.963 | 1.91× |
| 1,000 | 5 | 0.834 | 4.340 | 5.20× |
| 1,000 | 10 | 1.157 | 8.676 | 7.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
