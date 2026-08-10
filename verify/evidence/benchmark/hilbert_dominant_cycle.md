# HilbertDominantCycle benchmark (`HilbertDominantCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.05M | 0.067 | 14.98M | 0.192 | 2.89× | 2.87× |
| 10,000 | 0.703 | 14.22M | 0.712 | 14.05M | 1.226 | 1.74× | 1.72× |
| 100,000 | 8.211 | 12.18M | 7.118 | 14.05M | 9.987 | 1.22× | 1.40× |
| 1,000,000 | 61.472 | 16.27M | 63.206 | 15.82M | 91.784 | 1.49× | 1.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.199 | 2.29× |
| 1 | 5 | 0.307 | 0.828 | 2.70× |
| 1 | 10 | 0.482 | 1.813 | 3.76× |
| 10 | 1 | 0.052 | 0.157 | 3.00× |
| 10 | 5 | 0.225 | 0.782 | 3.47× |
| 10 | 10 | 0.488 | 1.869 | 3.83× |
| 100 | 1 | 0.058 | 0.179 | 3.05× |
| 100 | 5 | 0.246 | 0.854 | 3.47× |
| 100 | 10 | 0.490 | 1.738 | 3.55× |
| 1,000 | 1 | 0.117 | 0.278 | 2.38× |
| 1,000 | 5 | 0.282 | 1.553 | 5.52× |
| 1,000 | 10 | 0.579 | 2.606 | 4.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
