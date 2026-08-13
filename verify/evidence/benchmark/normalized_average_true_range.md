# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.20M | 0.065 | 15.37M | 0.040 | 0.56× | 0.61× |
| 10,000 | 0.529 | 18.90M | 0.542 | 18.44M | 0.087 | 0.16× | 0.16× |
| 100,000 | 5.331 | 18.76M | 5.744 | 17.41M | 0.586 | 0.11× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.133 | 1.14× |
| 1 | 5 | 0.432 | 0.484 | 1.12× |
| 1 | 10 | 0.635 | 0.930 | 1.47× |
| 10 | 1 | 0.071 | 0.101 | 1.42× |
| 10 | 5 | 0.298 | 0.447 | 1.50× |
| 10 | 10 | 0.634 | 0.923 | 1.45× |
| 100 | 1 | 0.072 | 0.106 | 1.48× |
| 100 | 5 | 0.313 | 0.455 | 1.45× |
| 100 | 10 | 0.675 | 0.925 | 1.37× |
| 1,000 | 1 | 0.127 | 0.102 | 0.80× |
| 1,000 | 5 | 0.311 | 0.480 | 1.55× |
| 1,000 | 10 | 0.642 | 0.991 | 1.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
