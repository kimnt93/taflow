# TimeSegmentedVolume benchmark (`TSV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.44M | 0.050 | 19.95M | 0.187 | 3.44× | 3.72× |
| 10,000 | 0.388 | 25.74M | 0.389 | 25.73M | 0.714 | 1.84× | 1.84× |
| 100,000 | 4.037 | 24.77M | 3.697 | 27.05M | 5.810 | 1.44× | 1.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.308 | 2.66× |
| 1 | 5 | 0.535 | 1.102 | 2.06× |
| 1 | 10 | 0.669 | 2.281 | 3.41× |
| 10 | 1 | 0.079 | 0.212 | 2.69× |
| 10 | 5 | 0.306 | 1.240 | 4.06× |
| 10 | 10 | 0.638 | 2.193 | 3.44× |
| 100 | 1 | 0.073 | 0.218 | 3.01× |
| 100 | 5 | 0.303 | 1.233 | 4.07× |
| 100 | 10 | 0.613 | 2.305 | 3.76× |
| 1,000 | 1 | 0.113 | 0.277 | 2.46× |
| 1,000 | 5 | 0.305 | 1.534 | 5.04× |
| 1,000 | 10 | 0.686 | 2.848 | 4.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
