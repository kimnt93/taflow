# RollingKurtosis benchmark (`Kurtosis` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.55M | 0.051 | 19.63M | 0.170 | 2.98× | 3.33× |
| 10,000 | 0.532 | 18.80M | 0.431 | 23.21M | 0.535 | 1.01× | 1.24× |
| 100,000 | 4.283 | 23.35M | 4.523 | 22.11M | 4.790 | 1.12× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.298 | 2.12× |
| 1 | 5 | 0.353 | 1.338 | 3.79× |
| 1 | 10 | 0.692 | 2.602 | 3.76× |
| 10 | 1 | 0.084 | 0.231 | 2.76× |
| 10 | 5 | 0.308 | 1.242 | 4.04× |
| 10 | 10 | 0.634 | 2.256 | 3.56× |
| 100 | 1 | 0.071 | 0.228 | 3.20× |
| 100 | 5 | 0.313 | 1.254 | 4.01× |
| 100 | 10 | 0.615 | 2.289 | 3.72× |
| 1,000 | 1 | 0.114 | 0.254 | 2.23× |
| 1,000 | 5 | 0.318 | 1.430 | 4.50× |
| 1,000 | 10 | 0.644 | 2.644 | 4.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
