# MathDegrees benchmark (`numpy.degrees` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 534.02M | 0.001 | 856.50M | 0.014 | 7.28× | 11.68× |
| 10,000 | 0.006 | 1.78G | 0.003 | 3.37G | 0.025 | 4.43× | 8.39× |
| 100,000 | 0.052 | 1.91G | 0.031 | 3.25G | 0.137 | 2.61× | 4.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.070 | 0.92× |
| 1 | 5 | 0.252 | 0.315 | 1.25× |
| 1 | 10 | 0.465 | 0.641 | 1.38× |
| 10 | 1 | 0.045 | 0.061 | 1.34× |
| 10 | 5 | 0.205 | 0.331 | 1.62× |
| 10 | 10 | 0.403 | 0.635 | 1.58× |
| 100 | 1 | 0.043 | 0.056 | 1.29× |
| 100 | 5 | 0.190 | 0.289 | 1.52× |
| 100 | 10 | 0.400 | 0.685 | 1.71× |
| 1,000 | 1 | 0.044 | 0.066 | 1.49× |
| 1,000 | 5 | 0.262 | 0.323 | 1.23× |
| 1,000 | 10 | 0.410 | 0.644 | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
