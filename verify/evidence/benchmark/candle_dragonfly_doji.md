# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.62M | 0.010 | 97.13M | 0.035 | 2.59× | 3.38× |
| 10,000 | 0.073 | 136.99M | 0.066 | 151.75M | 0.102 | 1.40× | 1.55× |
| 100,000 | 0.760 | 131.56M | 0.747 | 133.79M | 0.724 | 0.95× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.134 | 1.30× |
| 1 | 5 | 0.352 | 0.440 | 1.25× |
| 1 | 10 | 0.531 | 0.893 | 1.68× |
| 10 | 1 | 0.056 | 0.088 | 1.57× |
| 10 | 5 | 0.270 | 0.478 | 1.77× |
| 10 | 10 | 0.559 | 0.896 | 1.60× |
| 100 | 1 | 0.053 | 0.092 | 1.73× |
| 100 | 5 | 0.263 | 0.454 | 1.73× |
| 100 | 10 | 0.557 | 0.907 | 1.63× |
| 1,000 | 1 | 0.068 | 0.103 | 1.53× |
| 1,000 | 5 | 0.256 | 0.459 | 1.79× |
| 1,000 | 10 | 0.575 | 1.046 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
