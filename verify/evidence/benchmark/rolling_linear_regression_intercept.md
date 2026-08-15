# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.35M | 0.013 | 75.88M | 0.043 | 3.12× | 3.27× |
| 10,000 | 0.120 | 83.15M | 0.118 | 84.55M | 0.154 | 1.28× | 1.30× |
| 100,000 | 1.249 | 80.04M | 1.212 | 82.50M | 1.205 | 0.96× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.154 | 1.57× |
| 1 | 5 | 0.317 | 0.486 | 1.53× |
| 1 | 10 | 0.392 | 0.972 | 2.48× |
| 10 | 1 | 0.041 | 0.092 | 2.25× |
| 10 | 5 | 0.200 | 0.512 | 2.56× |
| 10 | 10 | 0.438 | 1.021 | 2.33× |
| 100 | 1 | 0.041 | 0.096 | 2.31× |
| 100 | 5 | 0.190 | 0.450 | 2.37× |
| 100 | 10 | 0.387 | 0.967 | 2.50× |
| 1,000 | 1 | 0.058 | 0.113 | 1.94× |
| 1,000 | 5 | 0.194 | 0.499 | 2.57× |
| 1,000 | 10 | 0.417 | 1.053 | 2.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
