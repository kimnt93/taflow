# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.93M | 0.013 | 78.11M | 0.040 | 2.37× | 3.09× |
| 10,000 | 0.106 | 94.00M | 0.100 | 99.56M | 0.128 | 1.20× | 1.27× |
| 100,000 | 1.016 | 98.44M | 1.007 | 99.30M | 1.004 | 0.99× | 1.00× |
| 1,000,000 | 10.450 | 95.70M | 10.094 | 99.07M | 11.959 | 1.14× | 1.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.120 | 0.82× |
| 1 | 5 | 0.319 | 0.466 | 1.46× |
| 1 | 10 | 0.473 | 0.955 | 2.02× |
| 10 | 1 | 0.053 | 0.100 | 1.89× |
| 10 | 5 | 0.249 | 0.455 | 1.82× |
| 10 | 10 | 0.505 | 0.938 | 1.86× |
| 100 | 1 | 0.054 | 0.094 | 1.73× |
| 100 | 5 | 0.229 | 0.445 | 1.94× |
| 100 | 10 | 0.485 | 0.914 | 1.89× |
| 1,000 | 1 | 0.063 | 0.102 | 1.61× |
| 1,000 | 5 | 0.232 | 0.460 | 1.98× |
| 1,000 | 10 | 0.500 | 1.043 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
