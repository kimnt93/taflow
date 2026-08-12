# Ichimoku benchmark (`causal ichimoku components` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.096 | 10.37M | 0.094 | 10.69M | 0.467 | 4.85× | 5.00× |
| 10,000 | 0.964 | 10.38M | 0.996 | 10.04M | 2.729 | 2.83× | 2.74× |
| 100,000 | 9.289 | 10.77M | 9.092 | 11.00M | 29.133 | 3.14× | 3.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.167 | 1.55× |
| 1 | 5 | 0.320 | 0.664 | 2.08× |
| 1 | 10 | 0.483 | 1.386 | 2.87× |
| 10 | 1 | 0.056 | 0.219 | 3.89× |
| 10 | 5 | 0.249 | 1.022 | 4.10× |
| 10 | 10 | 0.539 | 2.151 | 3.99× |
| 100 | 1 | 0.067 | 0.328 | 4.87× |
| 100 | 5 | 0.246 | 1.871 | 7.62× |
| 100 | 10 | 0.546 | 3.685 | 6.76× |
| 1,000 | 1 | 0.157 | 0.615 | 3.92× |
| 1,000 | 5 | 0.316 | 2.276 | 7.20× |
| 1,000 | 10 | 0.658 | 4.554 | 6.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
