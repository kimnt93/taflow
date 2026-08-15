# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.88M | 0.005 | 192.39M | 0.036 | 4.26× | 6.89× |
| 10,000 | 0.074 | 135.72M | 0.068 | 147.34M | 0.105 | 1.43× | 1.55× |
| 100,000 | 0.749 | 133.43M | 0.719 | 139.09M | 0.749 | 1.00× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.116 | 1.42× |
| 1 | 5 | 0.245 | 0.488 | 1.99× |
| 1 | 10 | 0.415 | 0.982 | 2.37× |
| 10 | 1 | 0.041 | 0.085 | 2.08× |
| 10 | 5 | 0.181 | 0.425 | 2.35× |
| 10 | 10 | 0.390 | 1.007 | 2.58× |
| 100 | 1 | 0.049 | 0.102 | 2.07× |
| 100 | 5 | 0.191 | 0.425 | 2.23× |
| 100 | 10 | 0.403 | 0.910 | 2.26× |
| 1,000 | 1 | 0.053 | 0.109 | 2.05× |
| 1,000 | 5 | 0.235 | 0.515 | 2.19× |
| 1,000 | 10 | 0.440 | 0.980 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
