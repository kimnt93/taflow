# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.70M | 0.009 | 116.71M | 0.034 | 3.91× | 4.01× |
| 10,000 | 0.061 | 162.76M | 0.059 | 168.24M | 0.090 | 1.46× | 1.51× |
| 100,000 | 0.598 | 167.17M | 0.590 | 169.45M | 0.624 | 1.04× | 1.06× |
| 1,000,000 | 6.351 | 157.46M | 5.763 | 173.52M | 5.831 | 0.92× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.105 | 0.94× |
| 1 | 5 | 0.267 | 0.493 | 1.84× |
| 1 | 10 | 0.484 | 0.869 | 1.80× |
| 10 | 1 | 0.047 | 0.093 | 1.95× |
| 10 | 5 | 0.231 | 0.461 | 1.99× |
| 10 | 10 | 0.497 | 0.901 | 1.81× |
| 100 | 1 | 0.050 | 0.088 | 1.76× |
| 100 | 5 | 0.228 | 0.417 | 1.82× |
| 100 | 10 | 0.451 | 0.977 | 2.17× |
| 1,000 | 1 | 0.059 | 0.101 | 1.71× |
| 1,000 | 5 | 0.244 | 0.444 | 1.82× |
| 1,000 | 10 | 0.488 | 0.975 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
