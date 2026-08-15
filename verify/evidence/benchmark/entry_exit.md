# EntryExit benchmark (`entry-exit position state` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 230.72M | 0.003 | 335.86M | 0.133 | 30.68× | 44.66× |
| 10,000 | 0.023 | 435.14M | 0.020 | 508.23M | 1.279 | 55.68× | 65.03× |
| 100,000 | 0.196 | 510.27M | 0.178 | 561.22M | 12.750 | 65.06× | 71.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.100 | 1.13× |
| 1 | 5 | 0.256 | 0.363 | 1.42× |
| 1 | 10 | 0.398 | 0.608 | 1.53× |
| 10 | 1 | 0.046 | 0.063 | 1.35× |
| 10 | 5 | 0.174 | 0.306 | 1.76× |
| 10 | 10 | 0.401 | 0.657 | 1.64× |
| 100 | 1 | 0.041 | 0.081 | 1.96× |
| 100 | 5 | 0.187 | 0.384 | 2.05× |
| 100 | 10 | 0.419 | 0.875 | 2.09× |
| 1,000 | 1 | 0.053 | 0.201 | 3.83× |
| 1,000 | 5 | 0.215 | 1.039 | 4.83× |
| 1,000 | 10 | 0.580 | 1.960 | 3.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
