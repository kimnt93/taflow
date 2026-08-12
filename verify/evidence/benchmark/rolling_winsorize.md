# RollingWinsorize benchmark (`rolling winsorize` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.61M | 0.055 | 18.17M | 0.627 | 11.03× | 11.38× |
| 10,000 | 0.609 | 16.42M | 0.645 | 15.51M | 3.518 | 5.78× | 5.46× |
| 100,000 | 6.456 | 15.49M | 6.721 | 14.88M | 33.051 | 5.12× | 4.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.494 | 6.06× |
| 1 | 5 | 0.411 | 1.747 | 4.25× |
| 1 | 10 | 0.505 | 3.516 | 6.96× |
| 10 | 1 | 0.054 | 0.329 | 6.15× |
| 10 | 5 | 0.231 | 1.610 | 6.97× |
| 10 | 10 | 0.487 | 3.451 | 7.09× |
| 100 | 1 | 0.063 | 0.374 | 5.97× |
| 100 | 5 | 0.241 | 2.161 | 8.96× |
| 100 | 10 | 0.526 | 4.146 | 7.88× |
| 1,000 | 1 | 0.114 | 0.689 | 6.02× |
| 1,000 | 5 | 0.268 | 2.376 | 8.88× |
| 1,000 | 10 | 0.566 | 5.065 | 8.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
