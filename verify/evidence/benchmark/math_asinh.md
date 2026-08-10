# MathAsinh benchmark (`numpy.arcsinh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.95M | 0.015 | 66.84M | 0.024 | 1.43× | 1.63× |
| 10,000 | 0.189 | 53.00M | 0.148 | 67.64M | 0.157 | 0.83× | 1.06× |
| 100,000 | 1.333 | 75.04M | 1.284 | 77.88M | 1.352 | 1.01× | 1.05× |
| 1,000,000 | 13.138 | 76.12M | 12.314 | 81.21M | 12.248 | 0.93× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.088 | 1.08× |
| 1 | 5 | 0.326 | 0.288 | 0.88× |
| 1 | 10 | 0.469 | 0.566 | 1.21× |
| 10 | 1 | 0.050 | 0.060 | 1.19× |
| 10 | 5 | 0.229 | 0.277 | 1.21× |
| 10 | 10 | 0.458 | 0.577 | 1.26× |
| 100 | 1 | 0.045 | 0.055 | 1.21× |
| 100 | 5 | 0.209 | 0.277 | 1.33× |
| 100 | 10 | 0.471 | 0.582 | 1.24× |
| 1,000 | 1 | 0.058 | 0.068 | 1.17× |
| 1,000 | 5 | 0.214 | 0.311 | 1.45× |
| 1,000 | 10 | 0.527 | 0.764 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
