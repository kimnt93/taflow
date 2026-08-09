# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.68M | 0.006 | 174.02M | 0.040 | 7.04× | 6.97× |
| 10,000 | 0.033 | 301.66M | 0.033 | 301.05M | 0.045 | 1.36× | 1.35× |
| 100,000 | 0.324 | 308.75M | 0.284 | 352.24M | 0.170 | 0.53× | 0.60× |
| 1,000,000 | 4.057 | 246.48M | 3.337 | 299.67M | 1.405 | 0.35× | 0.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.140 | 1.23× |
| 1 | 5 | 0.271 | 0.449 | 1.66× |
| 1 | 10 | 0.461 | 0.874 | 1.89× |
| 10 | 1 | 0.051 | 0.088 | 1.72× |
| 10 | 5 | 0.220 | 0.404 | 1.84× |
| 10 | 10 | 0.472 | 0.917 | 1.94× |
| 100 | 1 | 0.051 | 0.087 | 1.71× |
| 100 | 5 | 0.219 | 0.418 | 1.91× |
| 100 | 10 | 0.479 | 0.877 | 1.83× |
| 1,000 | 1 | 0.056 | 0.092 | 1.63× |
| 1,000 | 5 | 0.244 | 0.430 | 1.77× |
| 1,000 | 10 | 0.486 | 0.887 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
