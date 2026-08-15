# CumulativeMinimum benchmark (`numpy.minimum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 227.11M | 0.003 | 292.71M | 0.015 | 3.50× | 4.51× |
| 10,000 | 0.031 | 324.71M | 0.027 | 368.79M | 0.039 | 1.28× | 1.45× |
| 100,000 | 0.314 | 318.91M | 0.269 | 371.11M | 0.279 | 0.89× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.091 | 0.62× |
| 1 | 5 | 0.227 | 0.325 | 1.43× |
| 1 | 10 | 0.385 | 0.615 | 1.60× |
| 10 | 1 | 0.043 | 0.063 | 1.46× |
| 10 | 5 | 0.183 | 0.289 | 1.58× |
| 10 | 10 | 0.399 | 0.638 | 1.60× |
| 100 | 1 | 0.044 | 0.064 | 1.45× |
| 100 | 5 | 0.183 | 0.305 | 1.67× |
| 100 | 10 | 0.397 | 0.606 | 1.53× |
| 1,000 | 1 | 0.047 | 0.062 | 1.33× |
| 1,000 | 5 | 0.202 | 0.334 | 1.65× |
| 1,000 | 10 | 0.449 | 0.714 | 1.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
