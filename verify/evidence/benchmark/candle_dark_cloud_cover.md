# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.56M | 0.074 | 13.59M | 0.036 | 0.41× | 0.48× |
| 10,000 | 0.675 | 14.81M | 0.784 | 12.76M | 0.107 | 0.16× | 0.14× |
| 100,000 | 6.636 | 15.07M | 6.610 | 15.13M | 0.769 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.122 | 1.01× |
| 1 | 5 | 0.411 | 0.515 | 1.25× |
| 1 | 10 | 0.666 | 0.962 | 1.45× |
| 10 | 1 | 0.066 | 0.091 | 1.38× |
| 10 | 5 | 0.324 | 0.456 | 1.41× |
| 10 | 10 | 0.642 | 0.944 | 1.47× |
| 100 | 1 | 0.076 | 0.089 | 1.17× |
| 100 | 5 | 0.302 | 0.454 | 1.51× |
| 100 | 10 | 0.649 | 0.935 | 1.44× |
| 1,000 | 1 | 0.136 | 0.107 | 0.79× |
| 1,000 | 5 | 0.359 | 0.627 | 1.75× |
| 1,000 | 10 | 0.722 | 1.068 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
