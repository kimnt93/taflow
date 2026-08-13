# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.91M | 0.023 | 43.33M | 0.029 | 1.00× | 1.24× |
| 10,000 | 0.168 | 59.43M | 0.152 | 65.88M | 0.034 | 0.20× | 0.22× |
| 100,000 | 1.489 | 67.15M | 1.488 | 67.20M | 0.066 | 0.04× | 0.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.169 | 0.154 | 0.91× |
| 1 | 5 | 0.440 | 0.442 | 1.00× |
| 1 | 10 | 0.584 | 0.922 | 1.58× |
| 10 | 1 | 0.063 | 0.095 | 1.49× |
| 10 | 5 | 0.282 | 0.437 | 1.55× |
| 10 | 10 | 0.599 | 0.911 | 1.52× |
| 100 | 1 | 0.067 | 0.091 | 1.37× |
| 100 | 5 | 0.286 | 0.417 | 1.45× |
| 100 | 10 | 0.606 | 0.893 | 1.47× |
| 1,000 | 1 | 0.080 | 0.097 | 1.22× |
| 1,000 | 5 | 0.291 | 0.432 | 1.49× |
| 1,000 | 10 | 0.636 | 0.898 | 1.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
