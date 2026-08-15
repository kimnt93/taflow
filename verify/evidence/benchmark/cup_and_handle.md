# CupAndHandle benchmark (`CupAndHandle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.58M | 0.007 | 145.38M | 0.226 | 24.36× | 32.91× |
| 10,000 | 0.087 | 114.95M | 0.080 | 124.40M | 1.384 | 15.90× | 17.21× |
| 100,000 | 0.849 | 117.81M | 0.798 | 125.37M | 12.447 | 14.66× | 15.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.246 | 4.07× |
| 1 | 5 | 0.210 | 0.850 | 4.06× |
| 1 | 10 | 0.410 | 1.653 | 4.03× |
| 10 | 1 | 0.051 | 0.170 | 3.33× |
| 10 | 5 | 0.190 | 1.167 | 6.15× |
| 10 | 10 | 0.391 | 1.678 | 4.29× |
| 100 | 1 | 0.051 | 0.183 | 3.57× |
| 100 | 5 | 0.185 | 1.149 | 6.22× |
| 100 | 10 | 0.434 | 1.782 | 4.11× |
| 1,000 | 1 | 0.053 | 0.297 | 5.58× |
| 1,000 | 5 | 0.197 | 1.753 | 8.88× |
| 1,000 | 10 | 0.413 | 3.136 | 7.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
