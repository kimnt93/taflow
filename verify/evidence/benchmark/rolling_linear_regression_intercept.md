# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.174 | 5.76M | 0.168 | 5.96M | 0.043 | 0.25× | 0.26× |
| 10,000 | 1.602 | 6.24M | 1.663 | 6.01M | 0.147 | 0.09× | 0.09× |
| 100,000 | 16.256 | 6.15M | 16.053 | 6.23M | 1.450 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.127 | 1.19× |
| 1 | 5 | 0.375 | 0.537 | 1.43× |
| 1 | 10 | 0.627 | 0.999 | 1.59× |
| 10 | 1 | 0.066 | 0.096 | 1.45× |
| 10 | 5 | 0.314 | 0.451 | 1.44× |
| 10 | 10 | 0.631 | 1.109 | 1.76× |
| 100 | 1 | 0.092 | 0.100 | 1.09× |
| 100 | 5 | 0.328 | 0.471 | 1.44× |
| 100 | 10 | 0.695 | 1.123 | 1.62× |
| 1,000 | 1 | 0.257 | 0.113 | 0.44× |
| 1,000 | 5 | 0.512 | 0.541 | 1.06× |
| 1,000 | 10 | 0.790 | 1.112 | 1.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
