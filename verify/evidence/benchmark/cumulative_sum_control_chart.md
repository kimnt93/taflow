# CumulativeSumControlChart benchmark (`CUSUM event filter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.30M | 0.005 | 206.73M | 0.530 | 93.96× | 109.56× |
| 10,000 | 0.043 | 232.23M | 0.041 | 241.72M | 5.257 | 122.09× | 127.08× |
| 100,000 | 0.479 | 208.73M | 0.419 | 238.47M | 51.756 | 108.03× | 123.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.131 | 1.35× |
| 1 | 5 | 0.257 | 0.420 | 1.63× |
| 1 | 10 | 0.367 | 0.832 | 2.27× |
| 10 | 1 | 0.043 | 0.096 | 2.22× |
| 10 | 5 | 0.176 | 0.451 | 2.56× |
| 10 | 10 | 0.425 | 0.923 | 2.17× |
| 100 | 1 | 0.047 | 0.135 | 2.86× |
| 100 | 5 | 0.190 | 0.653 | 3.45× |
| 100 | 10 | 0.380 | 1.422 | 3.74× |
| 1,000 | 1 | 0.049 | 0.601 | 12.31× |
| 1,000 | 5 | 0.201 | 3.006 | 14.92× |
| 1,000 | 10 | 0.427 | 6.004 | 14.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
