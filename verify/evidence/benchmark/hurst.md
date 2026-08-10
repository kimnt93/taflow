# Hurst benchmark (`HurstExponent` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.384 | 2.61M | 0.393 | 2.55M | 0.468 | 1.22× | 1.19× |
| 10,000 | 3.848 | 2.60M | 3.821 | 2.62M | 3.327 | 0.86× | 0.87× |
| 100,000 | 40.215 | 2.49M | 41.364 | 2.42M | 31.765 | 0.79× | 0.77× |
| 1,000,000 | 398.868 | 2.51M | 399.077 | 2.51M | 319.098 | 0.80× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.556 | 5.64× |
| 1 | 5 | 0.327 | 1.257 | 3.84× |
| 1 | 10 | 0.504 | 2.561 | 5.08× |
| 10 | 1 | 0.046 | 0.242 | 5.24× |
| 10 | 5 | 0.218 | 1.445 | 6.63× |
| 10 | 10 | 0.499 | 2.859 | 5.73× |
| 100 | 1 | 0.082 | 0.276 | 3.36× |
| 100 | 5 | 0.240 | 1.633 | 6.80× |
| 100 | 10 | 0.536 | 2.849 | 5.31× |
| 1,000 | 1 | 0.468 | 0.591 | 1.26× |
| 1,000 | 5 | 0.615 | 3.245 | 5.27× |
| 1,000 | 10 | 0.958 | 6.416 | 6.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
