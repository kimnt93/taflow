# WilliamsAccumulationDistribution benchmark (`Wad` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.38M | 0.008 | 120.12M | 0.194 | 17.91× | 23.29× |
| 10,000 | 0.071 | 141.34M | 0.067 | 148.96M | 1.059 | 14.97× | 15.77× |
| 100,000 | 0.661 | 151.24M | 0.648 | 154.41M | 10.265 | 15.52× | 15.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.221 | 3.18× |
| 1 | 5 | 0.311 | 1.146 | 3.69× |
| 1 | 10 | 0.534 | 1.994 | 3.73× |
| 10 | 1 | 0.059 | 0.168 | 2.84× |
| 10 | 5 | 4.069 | 3.409 | 0.84× |
| 10 | 10 | 0.653 | 2.684 | 4.11× |
| 100 | 1 | 0.066 | 0.214 | 3.27× |
| 100 | 5 | 0.355 | 1.169 | 3.29× |
| 100 | 10 | 0.668 | 2.482 | 3.71× |
| 1,000 | 1 | 0.100 | 0.341 | 3.43× |
| 1,000 | 5 | 0.352 | 1.655 | 4.70× |
| 1,000 | 10 | 0.591 | 3.304 | 5.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
