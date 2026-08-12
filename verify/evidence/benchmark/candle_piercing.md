# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.23M | 0.014 | 69.39M | 0.036 | 2.08× | 2.52× |
| 10,000 | 0.130 | 76.99M | 0.128 | 78.03M | 0.142 | 1.09× | 1.11× |
| 100,000 | 1.274 | 78.49M | 1.259 | 79.43M | 0.972 | 0.76× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.140 | 1.00× |
| 1 | 5 | 0.420 | 0.486 | 1.16× |
| 1 | 10 | 0.535 | 0.977 | 1.83× |
| 10 | 1 | 0.054 | 0.085 | 1.56× |
| 10 | 5 | 0.261 | 0.447 | 1.71× |
| 10 | 10 | 0.520 | 0.914 | 1.76× |
| 100 | 1 | 0.063 | 0.103 | 1.64× |
| 100 | 5 | 0.264 | 0.441 | 1.67× |
| 100 | 10 | 0.589 | 0.902 | 1.53× |
| 1,000 | 1 | 0.072 | 0.097 | 1.36× |
| 1,000 | 5 | 0.278 | 0.529 | 1.90× |
| 1,000 | 10 | 0.573 | 1.072 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
