# TomDeMarkSequential benchmark (`TDSequential` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.56M | 0.056 | 17.80M | 0.593 | 9.22× | 10.55× |
| 10,000 | 0.505 | 19.79M | 0.495 | 20.18M | 4.043 | 8.00× | 8.16× |
| 100,000 | 5.323 | 18.79M | 4.965 | 20.14M | 44.103 | 8.29× | 8.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.383 | 3.57× |
| 1 | 5 | 0.399 | 13.532 | 33.95× |
| 1 | 10 | 0.679 | 2.859 | 4.21× |
| 10 | 1 | 0.071 | 0.267 | 3.77× |
| 10 | 5 | 0.276 | 1.446 | 5.23× |
| 10 | 10 | 0.564 | 2.995 | 5.31× |
| 100 | 1 | 0.074 | 0.299 | 4.03× |
| 100 | 5 | 0.303 | 1.642 | 5.42× |
| 100 | 10 | 0.624 | 3.522 | 5.64× |
| 1,000 | 1 | 0.129 | 0.831 | 6.45× |
| 1,000 | 5 | 0.568 | 3.986 | 7.02× |
| 1,000 | 10 | 1.098 | 8.201 | 7.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
