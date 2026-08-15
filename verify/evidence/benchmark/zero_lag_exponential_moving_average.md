# ZeroLagExponentialMovingAverage benchmark (`ZLEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.77M | 0.006 | 165.79M | 0.153 | 22.13× | 25.35× |
| 10,000 | 0.052 | 190.83M | 0.050 | 201.09M | 0.496 | 9.46× | 9.97× |
| 100,000 | 0.491 | 203.84M | 0.458 | 218.28M | 3.974 | 8.10× | 8.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.205 | 2.31× |
| 1 | 5 | 0.283 | 0.942 | 3.33× |
| 1 | 10 | 0.379 | 2.158 | 5.69× |
| 10 | 1 | 0.053 | 0.183 | 3.47× |
| 10 | 5 | 0.178 | 0.961 | 5.39× |
| 10 | 10 | 0.425 | 2.128 | 5.01× |
| 100 | 1 | 0.045 | 0.196 | 4.30× |
| 100 | 5 | 0.203 | 0.968 | 4.77× |
| 100 | 10 | 0.454 | 2.151 | 4.74× |
| 1,000 | 1 | 0.048 | 0.233 | 4.88× |
| 1,000 | 5 | 0.207 | 1.185 | 5.72× |
| 1,000 | 10 | 0.434 | 2.486 | 5.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
