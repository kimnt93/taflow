# GapDown benchmark (`gap down relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.87M | 0.006 | 158.41M | 0.024 | 3.12× | 3.84× |
| 10,000 | 0.033 | 298.98M | 0.030 | 328.40M | 0.045 | 1.34× | 1.47× |
| 100,000 | 0.290 | 344.90M | 0.266 | 375.84M | 0.250 | 0.86× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.103 | 0.75× |
| 1 | 5 | 0.251 | 0.366 | 1.46× |
| 1 | 10 | 0.501 | 0.736 | 1.47× |
| 10 | 1 | 0.048 | 0.075 | 1.57× |
| 10 | 5 | 0.252 | 0.394 | 1.56× |
| 10 | 10 | 0.513 | 0.768 | 1.50× |
| 100 | 1 | 0.048 | 0.074 | 1.52× |
| 100 | 5 | 0.230 | 0.348 | 1.51× |
| 100 | 10 | 0.500 | 0.760 | 1.52× |
| 1,000 | 1 | 0.053 | 0.076 | 1.43× |
| 1,000 | 5 | 0.223 | 0.507 | 2.27× |
| 1,000 | 10 | 0.530 | 14.656 | 27.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
