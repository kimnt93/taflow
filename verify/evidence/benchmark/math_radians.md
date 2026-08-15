# MathRadians benchmark (`numpy.radians` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 567.68M | 0.001 | 1.14G | 0.013 | 7.35× | 14.81× |
| 10,000 | 0.005 | 1.94G | 0.003 | 3.21G | 0.024 | 4.72× | 7.83× |
| 100,000 | 0.055 | 1.82G | 0.030 | 3.29G | 0.130 | 2.37× | 4.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.163 | 0.124 | 0.76× |
| 1 | 5 | 0.245 | 0.275 | 1.12× |
| 1 | 10 | 0.368 | 0.556 | 1.51× |
| 10 | 1 | 0.038 | 0.053 | 1.40× |
| 10 | 5 | 0.168 | 0.261 | 1.55× |
| 10 | 10 | 0.384 | 0.606 | 1.58× |
| 100 | 1 | 0.038 | 0.053 | 1.39× |
| 100 | 5 | 0.179 | 0.259 | 1.45× |
| 100 | 10 | 0.368 | 0.578 | 1.57× |
| 1,000 | 1 | 0.039 | 0.058 | 1.47× |
| 1,000 | 5 | 0.179 | 0.312 | 1.74× |
| 1,000 | 10 | 0.397 | 0.606 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
