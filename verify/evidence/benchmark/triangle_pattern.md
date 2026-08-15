# TrianglePattern benchmark (`Triangle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.56M | 0.008 | 131.15M | 0.227 | 21.43× | 29.72× |
| 10,000 | 0.103 | 97.43M | 0.100 | 100.22M | 1.367 | 13.32× | 13.70× |
| 100,000 | 0.965 | 103.64M | 0.978 | 102.22M | 13.009 | 13.48× | 13.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.222 | 3.09× |
| 1 | 5 | 0.257 | 0.800 | 3.11× |
| 1 | 10 | 0.399 | 1.686 | 4.23× |
| 10 | 1 | 0.046 | 0.170 | 3.72× |
| 10 | 5 | 0.198 | 1.072 | 5.42× |
| 10 | 10 | 0.397 | 1.729 | 4.36× |
| 100 | 1 | 0.046 | 0.191 | 4.17× |
| 100 | 5 | 0.205 | 1.156 | 5.63× |
| 100 | 10 | 0.416 | 1.893 | 4.55× |
| 1,000 | 1 | 0.061 | 0.300 | 4.92× |
| 1,000 | 5 | 0.210 | 1.780 | 8.46× |
| 1,000 | 10 | 0.469 | 3.021 | 6.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
