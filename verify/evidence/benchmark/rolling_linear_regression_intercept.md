# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.29M | 0.014 | 73.56M | 0.045 | 3.08× | 3.32× |
| 10,000 | 0.133 | 75.36M | 0.125 | 80.31M | 0.153 | 1.15× | 1.22× |
| 100,000 | 1.283 | 77.96M | 1.192 | 83.91M | 1.262 | 0.98× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.117 | 1.21× |
| 1 | 5 | 0.200 | 0.454 | 2.27× |
| 1 | 10 | 0.397 | 0.942 | 2.37× |
| 10 | 1 | 0.040 | 0.090 | 2.25× |
| 10 | 5 | 0.178 | 0.475 | 2.67× |
| 10 | 10 | 0.403 | 0.991 | 2.46× |
| 100 | 1 | 0.044 | 0.101 | 2.30× |
| 100 | 5 | 0.201 | 0.460 | 2.28× |
| 100 | 10 | 0.412 | 1.014 | 2.46× |
| 1,000 | 1 | 0.058 | 0.115 | 1.99× |
| 1,000 | 5 | 0.224 | 0.521 | 2.32× |
| 1,000 | 10 | 0.435 | 1.107 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
