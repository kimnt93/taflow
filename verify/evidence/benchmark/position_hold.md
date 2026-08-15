# PositionHold benchmark (`nonzero position hold` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 248.68M | 0.003 | 343.42M | 0.123 | 30.69× | 42.39× |
| 10,000 | 0.024 | 415.62M | 0.021 | 475.10M | 1.118 | 46.48× | 53.13× |
| 100,000 | 0.213 | 470.57M | 0.205 | 488.70M | 11.348 | 53.40× | 55.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.111 | 1.49× |
| 1 | 5 | 0.224 | 0.290 | 1.30× |
| 1 | 10 | 0.380 | 0.627 | 1.65× |
| 10 | 1 | 0.043 | 0.072 | 1.66× |
| 10 | 5 | 0.172 | 0.313 | 1.82× |
| 10 | 10 | 0.366 | 0.622 | 1.70× |
| 100 | 1 | 0.041 | 0.069 | 1.70× |
| 100 | 5 | 0.194 | 0.392 | 2.02× |
| 100 | 10 | 0.400 | 0.695 | 1.73× |
| 1,000 | 1 | 0.044 | 0.188 | 4.24× |
| 1,000 | 5 | 0.189 | 0.887 | 4.69× |
| 1,000 | 10 | 0.422 | 1.793 | 4.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
