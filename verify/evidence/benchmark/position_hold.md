# PositionHold benchmark (`nonzero position hold` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 245.13M | 0.003 | 332.90M | 0.132 | 32.26× | 43.81× |
| 10,000 | 0.025 | 406.84M | 0.021 | 473.45M | 1.188 | 48.33× | 56.25× |
| 100,000 | 0.228 | 439.14M | 0.200 | 498.90M | 12.213 | 53.63× | 60.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.122 | 1.46× |
| 1 | 5 | 0.277 | 0.340 | 1.23× |
| 1 | 10 | 0.455 | 0.713 | 1.57× |
| 10 | 1 | 0.060 | 0.070 | 1.17× |
| 10 | 5 | 0.211 | 0.347 | 1.65× |
| 10 | 10 | 0.410 | 0.682 | 1.66× |
| 100 | 1 | 0.048 | 0.075 | 1.57× |
| 100 | 5 | 0.187 | 0.345 | 1.84× |
| 100 | 10 | 0.388 | 0.710 | 1.83× |
| 1,000 | 1 | 0.046 | 0.176 | 3.87× |
| 1,000 | 5 | 0.191 | 0.927 | 4.85× |
| 1,000 | 10 | 0.439 | 1.818 | 4.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
