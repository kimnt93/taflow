# RollingMaximumIndex benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.85M | 0.007 | 152.06M | 0.036 | 4.79× | 5.53× |
| 10,000 | 0.058 | 172.18M | 0.052 | 192.43M | 0.094 | 1.61× | 1.80× |
| 100,000 | 0.512 | 195.38M | 0.524 | 190.83M | 0.658 | 1.29× | 1.26× |
| 1,000,000 | 5.647 | 177.09M | 5.086 | 196.62M | 6.356 | 1.13× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.110 | 1.55× |
| 1 | 5 | 0.408 | 0.515 | 1.26× |
| 1 | 10 | 0.453 | 0.959 | 2.12× |
| 10 | 1 | 0.048 | 0.094 | 1.97× |
| 10 | 5 | 0.224 | 0.427 | 1.91× |
| 10 | 10 | 0.469 | 0.912 | 1.95× |
| 100 | 1 | 0.049 | 0.095 | 1.96× |
| 100 | 5 | 0.212 | 0.436 | 2.06× |
| 100 | 10 | 0.548 | 0.968 | 1.77× |
| 1,000 | 1 | 0.058 | 0.101 | 1.74× |
| 1,000 | 5 | 0.251 | 0.488 | 1.94× |
| 1,000 | 10 | 0.534 | 0.998 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
