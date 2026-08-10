# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.57M | 0.010 | 104.32M | 0.042 | 3.23× | 4.34× |
| 10,000 | 0.123 | 81.45M | 0.122 | 81.96M | 0.167 | 1.36× | 1.37× |
| 100,000 | 1.207 | 82.83M | 1.190 | 84.00M | 1.468 | 1.22× | 1.23× |
| 1,000,000 | 12.391 | 80.70M | 12.201 | 81.96M | 14.872 | 1.20× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.103 | 1.09× |
| 1 | 5 | 0.338 | 0.438 | 1.30× |
| 1 | 10 | 0.526 | 0.924 | 1.76× |
| 10 | 1 | 0.053 | 0.087 | 1.64× |
| 10 | 5 | 0.252 | 0.436 | 1.73× |
| 10 | 10 | 0.557 | 0.922 | 1.66× |
| 100 | 1 | 0.061 | 0.088 | 1.45× |
| 100 | 5 | 0.265 | 0.426 | 1.61× |
| 100 | 10 | 0.545 | 0.904 | 1.66× |
| 1,000 | 1 | 0.069 | 0.104 | 1.51× |
| 1,000 | 5 | 0.248 | 0.517 | 2.08× |
| 1,000 | 10 | 0.565 | 1.044 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
