# SqueezePro benchmark (`squeeze_pro` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.92M | 0.074 | 13.54M | 8.863 | 105.68× | 119.97× |
| 10,000 | 0.423 | 23.64M | 0.404 | 24.73M | 12.796 | 30.25× | 31.65× |
| 100,000 | 4.251 | 23.52M | 3.972 | 25.17M | 60.985 | 14.35× | 15.35× |
| 1,000,000 | 74.055 | 13.50M | 42.580 | 23.49M | 605.494 | 8.18× | 14.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.418 | 4.77× |
| 1 | 5 | 0.345 | 1.954 | 5.66× |
| 1 | 10 | 0.556 | 3.740 | 6.72× |
| 10 | 1 | 0.063 | 0.383 | 6.07× |
| 10 | 5 | 0.255 | 1.872 | 7.35× |
| 10 | 10 | 0.536 | 3.999 | 7.46× |
| 100 | 1 | 0.070 | 8.670 | 123.52× |
| 100 | 5 | 0.370 | 46.367 | 125.48× |
| 100 | 10 | 0.575 | 93.518 | 162.73× |
| 1,000 | 1 | 0.114 | 9.022 | 79.20× |
| 1,000 | 5 | 0.358 | 51.047 | 142.60× |
| 1,000 | 10 | 0.618 | 100.382 | 162.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
