# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.84M | 0.077 | 13.04M | 0.035 | 0.41× | 0.45× |
| 10,000 | 0.679 | 14.73M | 0.671 | 14.89M | 0.189 | 0.28× | 0.28× |
| 100,000 | 6.476 | 15.44M | 6.628 | 15.09M | 1.654 | 0.26× | 0.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.199 | 0.130 | 0.65× |
| 1 | 5 | 0.478 | 0.471 | 0.99× |
| 1 | 10 | 0.637 | 0.916 | 1.44× |
| 10 | 1 | 0.069 | 0.091 | 1.33× |
| 10 | 5 | 0.311 | 0.422 | 1.36× |
| 10 | 10 | 0.637 | 0.925 | 1.45× |
| 100 | 1 | 0.072 | 0.091 | 1.25× |
| 100 | 5 | 0.314 | 0.434 | 1.38× |
| 100 | 10 | 0.668 | 0.925 | 1.38× |
| 1,000 | 1 | 0.139 | 0.111 | 0.80× |
| 1,000 | 5 | 0.318 | 0.505 | 1.59× |
| 1,000 | 10 | 0.675 | 1.064 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
