# RollingQuantile benchmark (`RollingQuantile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.96M | 0.042 | 23.94M | 0.371 | 8.90× | 8.89× |
| 10,000 | 0.433 | 23.12M | 0.431 | 23.18M | 1.785 | 4.13× | 4.14× |
| 100,000 | 4.505 | 22.20M | 4.371 | 22.88M | 18.134 | 4.02× | 4.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.320 | 5.35× |
| 1 | 5 | 0.316 | 1.719 | 5.43× |
| 1 | 10 | 0.426 | 2.539 | 5.96× |
| 10 | 1 | 0.057 | 0.268 | 4.66× |
| 10 | 5 | 0.216 | 1.390 | 6.45× |
| 10 | 10 | 0.408 | 2.787 | 6.82× |
| 100 | 1 | 0.059 | 0.265 | 4.48× |
| 100 | 5 | 0.214 | 1.456 | 6.81× |
| 100 | 10 | 0.494 | 3.123 | 6.32× |
| 1,000 | 1 | 0.103 | 0.410 | 3.99× |
| 1,000 | 5 | 0.227 | 2.377 | 10.47× |
| 1,000 | 10 | 0.488 | 4.710 | 9.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
