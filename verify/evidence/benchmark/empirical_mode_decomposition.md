# EmpiricalModeDecomposition benchmark (`EmpiricalModeDecomposition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.12M | 0.040 | 25.18M | 0.230 | 5.33× | 5.80× |
| 10,000 | 0.413 | 24.23M | 0.406 | 24.66M | 1.340 | 3.25× | 3.30× |
| 100,000 | 4.047 | 24.71M | 3.925 | 25.48M | 6.966 | 1.72× | 1.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.284 | 2.29× |
| 1 | 5 | 0.322 | 1.384 | 4.29× |
| 1 | 10 | 0.542 | 2.726 | 5.03× |
| 10 | 1 | 0.051 | 0.253 | 4.99× |
| 10 | 5 | 0.235 | 1.464 | 6.23× |
| 10 | 10 | 0.481 | 2.658 | 5.52× |
| 100 | 1 | 0.056 | 0.260 | 4.64× |
| 100 | 5 | 0.247 | 1.434 | 5.81× |
| 100 | 10 | 0.517 | 2.871 | 5.55× |
| 1,000 | 1 | 0.096 | 0.319 | 3.34× |
| 1,000 | 5 | 0.245 | 1.835 | 7.50× |
| 1,000 | 10 | 0.538 | 3.312 | 6.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
