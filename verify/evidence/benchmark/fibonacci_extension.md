# FibonacciExtension benchmark (`FibExtension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.116 | 8.64M | 0.106 | 9.40M | 0.590 | 5.10× | 5.54× |
| 10,000 | 0.979 | 10.22M | 0.955 | 10.47M | 4.399 | 4.49× | 4.61× |
| 100,000 | 10.183 | 9.82M | 9.286 | 10.77M | 57.180 | 5.62× | 6.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.221 | 2.13× |
| 1 | 5 | 0.341 | 0.815 | 2.39× |
| 1 | 10 | 0.622 | 1.848 | 2.97× |
| 10 | 1 | 0.070 | 0.166 | 2.38× |
| 10 | 5 | 0.293 | 0.824 | 2.82× |
| 10 | 10 | 0.618 | 1.881 | 3.04× |
| 100 | 1 | 0.083 | 0.224 | 2.69× |
| 100 | 5 | 0.296 | 1.062 | 3.58× |
| 100 | 10 | 0.670 | 2.359 | 3.52× |
| 1,000 | 1 | 0.191 | 0.869 | 4.54× |
| 1,000 | 5 | 0.353 | 3.599 | 10.19× |
| 1,000 | 10 | 0.684 | 7.218 | 10.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
