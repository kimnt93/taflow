# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.91M | 0.005 | 184.49M | 0.031 | 4.13× | 5.73× |
| 10,000 | 0.023 | 426.42M | 0.019 | 539.14M | 0.037 | 1.56× | 1.97× |
| 100,000 | 0.200 | 499.27M | 0.189 | 528.96M | 0.103 | 0.52× | 0.55× |
| 1,000,000 | 3.187 | 313.73M | 2.631 | 380.04M | 1.914 | 0.60× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.117 | 1.36× |
| 1 | 5 | 0.322 | 0.475 | 1.48× |
| 1 | 10 | 0.604 | 1.090 | 1.80× |
| 10 | 1 | 0.060 | 0.090 | 1.50× |
| 10 | 5 | 0.275 | 0.500 | 1.82× |
| 10 | 10 | 0.586 | 1.179 | 2.01× |
| 100 | 1 | 0.078 | 0.123 | 1.59× |
| 100 | 5 | 0.319 | 0.666 | 2.09× |
| 100 | 10 | 0.707 | 1.361 | 1.93× |
| 1,000 | 1 | 0.068 | 0.113 | 1.66× |
| 1,000 | 5 | 0.386 | 0.701 | 1.81× |
| 1,000 | 10 | 0.729 | 1.309 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
