# RollingKurtosis benchmark (`Kurtosis` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.61M | 0.020 | 50.93M | 0.186 | 8.66× | 9.46× |
| 10,000 | 0.178 | 56.10M | 0.166 | 60.41M | 0.588 | 3.30× | 3.55× |
| 100,000 | 1.689 | 59.21M | 1.747 | 57.24M | 4.567 | 2.70× | 2.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.262 | 3.01× |
| 1 | 5 | 0.299 | 1.605 | 5.37× |
| 1 | 10 | 0.632 | 2.695 | 4.26× |
| 10 | 1 | 0.062 | 0.227 | 3.63× |
| 10 | 5 | 0.275 | 1.526 | 5.55× |
| 10 | 10 | 0.478 | 2.418 | 5.06× |
| 100 | 1 | 0.057 | 0.223 | 3.90× |
| 100 | 5 | 0.272 | 1.295 | 4.77× |
| 100 | 10 | 0.492 | 2.419 | 4.92× |
| 1,000 | 1 | 0.071 | 0.254 | 3.57× |
| 1,000 | 5 | 0.247 | 1.474 | 5.98× |
| 1,000 | 10 | 0.535 | 2.723 | 5.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
