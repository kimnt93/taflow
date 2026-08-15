# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.02M | 0.005 | 217.79M | 0.042 | 7.40× | 9.10× |
| 10,000 | 0.039 | 253.87M | 0.035 | 287.80M | 0.125 | 3.19× | 3.61× |
| 100,000 | 0.358 | 279.71M | 0.346 | 288.80M | 0.930 | 2.60× | 2.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.185 | 1.87× |
| 1 | 5 | 0.299 | 0.486 | 1.63× |
| 1 | 10 | 0.422 | 0.926 | 2.19× |
| 10 | 1 | 0.044 | 0.092 | 2.06× |
| 10 | 5 | 0.183 | 0.463 | 2.54× |
| 10 | 10 | 0.431 | 0.945 | 2.19× |
| 100 | 1 | 0.049 | 0.089 | 1.82× |
| 100 | 5 | 0.184 | 0.436 | 2.37× |
| 100 | 10 | 0.387 | 0.963 | 2.49× |
| 1,000 | 1 | 0.047 | 0.104 | 2.19× |
| 1,000 | 5 | 0.187 | 0.478 | 2.55× |
| 1,000 | 10 | 0.417 | 0.988 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
