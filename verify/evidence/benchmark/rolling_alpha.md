# RollingAlpha benchmark (`Alpha` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.84M | 0.038 | 26.13M | 0.221 | 5.50× | 5.78× |
| 10,000 | 0.374 | 26.73M | 0.370 | 27.04M | 0.894 | 2.39× | 2.42× |
| 100,000 | 4.013 | 24.92M | 3.798 | 26.33M | 7.613 | 1.90× | 2.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.279 | 3.04× |
| 1 | 5 | 0.258 | 1.200 | 4.66× |
| 1 | 10 | 0.418 | 2.536 | 6.07× |
| 10 | 1 | 0.043 | 0.230 | 5.32× |
| 10 | 5 | 0.199 | 1.371 | 6.91× |
| 10 | 10 | 0.402 | 2.481 | 6.16× |
| 100 | 1 | 0.051 | 0.237 | 4.70× |
| 100 | 5 | 0.194 | 1.420 | 7.30× |
| 100 | 10 | 0.418 | 2.577 | 6.16× |
| 1,000 | 1 | 0.083 | 0.298 | 3.57× |
| 1,000 | 5 | 0.212 | 1.801 | 8.51× |
| 1,000 | 10 | 0.465 | 3.275 | 7.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
