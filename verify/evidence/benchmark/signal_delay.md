# SignalDelay benchmark (`signal delay` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.03M | 0.035 | 28.73M | 0.025 | 0.62× | 0.71× |
| 10,000 | 0.278 | 35.91M | 0.273 | 36.68M | 0.028 | 0.10× | 0.10× |
| 100,000 | 2.671 | 37.44M | 2.637 | 37.92M | 0.064 | 0.02× | 0.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.125 | 0.96× |
| 1 | 5 | 0.396 | 0.440 | 1.11× |
| 1 | 10 | 0.597 | 0.816 | 1.37× |
| 10 | 1 | 0.069 | 0.089 | 1.28× |
| 10 | 5 | 0.292 | 0.405 | 1.39× |
| 10 | 10 | 0.599 | 0.860 | 1.44× |
| 100 | 1 | 0.067 | 0.087 | 1.29× |
| 100 | 5 | 0.310 | 0.413 | 1.33× |
| 100 | 10 | 0.631 | 0.840 | 1.33× |
| 1,000 | 1 | 0.091 | 0.093 | 1.03× |
| 1,000 | 5 | 0.287 | 0.417 | 1.46× |
| 1,000 | 10 | 0.648 | 0.893 | 1.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
