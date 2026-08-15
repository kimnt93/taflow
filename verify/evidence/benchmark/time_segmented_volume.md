# TimeSegmentedVolume benchmark (`TSV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.83M | 0.007 | 153.65M | 0.217 | 28.55× | 33.27× |
| 10,000 | 0.053 | 189.49M | 0.050 | 201.30M | 0.789 | 14.96× | 15.89× |
| 100,000 | 0.490 | 203.92M | 0.463 | 216.14M | 6.687 | 13.64× | 14.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.257 | 3.03× |
| 1 | 5 | 0.276 | 1.128 | 4.08× |
| 1 | 10 | 0.415 | 2.225 | 5.36× |
| 10 | 1 | 0.044 | 0.212 | 4.84× |
| 10 | 5 | 0.196 | 1.360 | 6.92× |
| 10 | 10 | 0.422 | 2.287 | 5.42× |
| 100 | 1 | 0.055 | 0.243 | 4.41× |
| 100 | 5 | 0.208 | 1.296 | 6.23× |
| 100 | 10 | 0.419 | 2.391 | 5.71× |
| 1,000 | 1 | 0.053 | 0.295 | 5.59× |
| 1,000 | 5 | 0.231 | 1.575 | 6.83× |
| 1,000 | 10 | 0.418 | 3.042 | 7.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
