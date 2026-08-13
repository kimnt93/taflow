# MovingAverageEnvelope benchmark (`MaEnvelope` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.233 | 4.28M | 0.218 | 4.58M | 0.530 | 2.27× | 2.43× |
| 10,000 | 2.147 | 4.66M | 2.156 | 4.64M | 3.504 | 1.63× | 1.63× |
| 100,000 | 21.618 | 4.63M | 21.761 | 4.60M | 39.888 | 1.85× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.312 | 3.03× |
| 1 | 5 | 0.388 | 1.407 | 3.63× |
| 1 | 10 | 0.621 | 2.579 | 4.15× |
| 10 | 1 | 0.071 | 0.252 | 3.56× |
| 10 | 5 | 0.310 | 1.426 | 4.60× |
| 10 | 10 | 0.634 | 2.814 | 4.44× |
| 100 | 1 | 0.110 | 0.318 | 2.88× |
| 100 | 5 | 0.318 | 1.617 | 5.08× |
| 100 | 10 | 0.672 | 2.993 | 4.46× |
| 1,000 | 1 | 0.296 | 0.808 | 2.73× |
| 1,000 | 5 | 0.507 | 3.452 | 6.81× |
| 1,000 | 10 | 0.873 | 13.060 | 14.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
