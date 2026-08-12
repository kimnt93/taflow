# ZeroLagExponentialMovingAverage benchmark (`ZLEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.33M | 0.007 | 140.01M | 0.156 | 19.22× | 21.83× |
| 10,000 | 0.056 | 179.18M | 0.053 | 190.19M | 0.511 | 9.15× | 9.71× |
| 100,000 | 0.486 | 205.92M | 0.446 | 224.02M | 3.770 | 7.76× | 8.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.244 | 2.07× |
| 1 | 5 | 0.299 | 0.995 | 3.32× |
| 1 | 10 | 0.501 | 2.110 | 4.21× |
| 10 | 1 | 0.049 | 0.195 | 4.01× |
| 10 | 5 | 0.268 | 0.974 | 3.64× |
| 10 | 10 | 0.505 | 2.166 | 4.29× |
| 100 | 1 | 0.056 | 0.194 | 3.44× |
| 100 | 5 | 0.244 | 0.978 | 4.00× |
| 100 | 10 | 0.482 | 2.232 | 4.63× |
| 1,000 | 1 | 0.057 | 0.235 | 4.09× |
| 1,000 | 5 | 0.229 | 1.152 | 5.03× |
| 1,000 | 10 | 0.512 | 2.603 | 5.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
