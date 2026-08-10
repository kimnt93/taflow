# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.097 | 10.33M | 0.123 | 8.16M | 0.453 | 4.68× | 3.69× |
| 10,000 | 0.987 | 10.13M | 1.166 | 8.58M | 4.451 | 4.51× | 3.82× |
| 100,000 | 13.797 | 7.25M | 10.151 | 9.85M | 41.578 | 3.01× | 4.10× |
| 1,000,000 | 98.662 | 10.14M | 100.793 | 9.92M | 417.280 | 4.23× | 4.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.105 | 0.92× |
| 1 | 5 | 0.343 | 0.472 | 1.38× |
| 1 | 10 | 0.459 | 0.936 | 2.04× |
| 10 | 1 | 0.050 | 0.090 | 1.80× |
| 10 | 5 | 0.257 | 0.474 | 1.85× |
| 10 | 10 | 0.607 | 0.941 | 1.55× |
| 100 | 1 | 0.072 | 0.114 | 1.59× |
| 100 | 5 | 0.243 | 0.546 | 2.25× |
| 100 | 10 | 0.480 | 1.175 | 2.45× |
| 1,000 | 1 | 0.157 | 0.505 | 3.21× |
| 1,000 | 5 | 0.282 | 2.615 | 9.27× |
| 1,000 | 10 | 0.579 | 5.130 | 8.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
