# TripleExponentialAverage benchmark (`T3` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.10M | 0.006 | 157.70M | 0.040 | 5.68× | 6.25× |
| 10,000 | 0.039 | 253.91M | 0.040 | 252.99M | 0.074 | 1.89× | 1.88× |
| 100,000 | 0.359 | 278.68M | 0.343 | 291.58M | 0.426 | 1.19× | 1.24× |
| 1,000,000 | 4.152 | 240.84M | 3.607 | 277.25M | 3.898 | 0.94× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.104 | 1.15× |
| 1 | 5 | 0.325 | 0.542 | 1.67× |
| 1 | 10 | 0.507 | 1.052 | 2.07× |
| 10 | 1 | 0.049 | 0.099 | 2.03× |
| 10 | 5 | 0.235 | 0.456 | 1.94× |
| 10 | 10 | 0.490 | 0.965 | 1.97× |
| 100 | 1 | 0.050 | 0.095 | 1.92× |
| 100 | 5 | 0.225 | 0.454 | 2.01× |
| 100 | 10 | 0.493 | 1.003 | 2.03× |
| 1,000 | 1 | 0.057 | 0.098 | 1.72× |
| 1,000 | 5 | 0.233 | 0.474 | 2.03× |
| 1,000 | 10 | 0.492 | 1.016 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
