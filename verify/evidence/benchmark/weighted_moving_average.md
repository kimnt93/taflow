# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 261.68M | 0.003 | 326.96M | 0.032 | 8.31× | 10.38× |
| 10,000 | 0.024 | 414.19M | 0.022 | 462.14M | 0.049 | 2.03× | 2.26× |
| 100,000 | 0.230 | 434.03M | 0.201 | 496.90M | 0.225 | 0.98× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.110 | 1.25× |
| 1 | 5 | 0.338 | 0.490 | 1.45× |
| 1 | 10 | 0.397 | 0.924 | 2.33× |
| 10 | 1 | 0.047 | 0.092 | 1.94× |
| 10 | 5 | 0.183 | 0.438 | 2.40× |
| 10 | 10 | 0.382 | 0.932 | 2.44× |
| 100 | 1 | 0.047 | 0.095 | 2.04× |
| 100 | 5 | 0.193 | 0.451 | 2.34× |
| 100 | 10 | 0.406 | 0.910 | 2.24× |
| 1,000 | 1 | 0.050 | 0.091 | 1.80× |
| 1,000 | 5 | 0.194 | 0.455 | 2.34× |
| 1,000 | 10 | 0.410 | 0.953 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
