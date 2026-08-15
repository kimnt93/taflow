# AdaptiveCycle benchmark (`AdaptiveCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.24M | 0.060 | 16.68M | 0.187 | 3.03× | 3.11× |
| 10,000 | 0.603 | 16.59M | 0.571 | 17.52M | 1.032 | 1.71× | 1.81× |
| 100,000 | 5.926 | 16.87M | 5.742 | 17.42M | 9.559 | 1.61× | 1.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.229 | 2.05× |
| 1 | 5 | 0.302 | 1.153 | 3.82× |
| 1 | 10 | 0.395 | 1.829 | 4.63× |
| 10 | 1 | 0.044 | 0.166 | 3.77× |
| 10 | 5 | 0.177 | 0.846 | 4.77× |
| 10 | 10 | 0.404 | 1.861 | 4.60× |
| 100 | 1 | 0.049 | 0.164 | 3.34× |
| 100 | 5 | 0.190 | 0.894 | 4.69× |
| 100 | 10 | 0.417 | 1.939 | 4.65× |
| 1,000 | 1 | 0.109 | 0.249 | 2.29× |
| 1,000 | 5 | 0.230 | 1.271 | 5.53× |
| 1,000 | 10 | 0.481 | 2.597 | 5.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
