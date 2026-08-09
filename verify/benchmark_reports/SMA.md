# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.30M | 0.005 | 205.61M | 0.037 | 6.13× | 7.63× |
| 10,000 | 0.026 | 385.46M | 0.024 | 408.66M | 0.057 | 2.20× | 2.33× |
| 100,000 | 0.229 | 436.49M | 0.204 | 489.29M | 0.237 | 1.03× | 1.16× |
| 1,000,000 | 2.675 | 373.83M | 2.208 | 452.89M | 2.019 | 0.75× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.106 | 1.36× |
| 1 | 5 | 0.315 | 0.538 | 1.71× |
| 1 | 10 | 0.504 | 0.989 | 1.96× |
| 10 | 1 | 0.050 | 0.096 | 1.93× |
| 10 | 5 | 0.229 | 0.452 | 1.98× |
| 10 | 10 | 0.538 | 0.991 | 1.84× |
| 100 | 1 | 0.050 | 0.097 | 1.94× |
| 100 | 5 | 0.235 | 0.460 | 1.96× |
| 100 | 10 | 0.514 | 1.059 | 2.06× |
| 1,000 | 1 | 0.053 | 0.098 | 1.84× |
| 1,000 | 5 | 0.245 | 0.470 | 1.91× |
| 1,000 | 10 | 0.529 | 1.044 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
