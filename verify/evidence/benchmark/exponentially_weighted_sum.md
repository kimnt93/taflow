# ExponentiallyWeightedSum benchmark (`exponentially weighted sum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 199.19M | 0.004 | 230.63M | 0.181 | 36.08× | 41.77× |
| 10,000 | 0.038 | 265.95M | 0.034 | 296.50M | 1.892 | 50.32× | 56.10× |
| 100,000 | 0.337 | 297.04M | 0.316 | 316.48M | 16.253 | 48.28× | 51.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.123 | 0.95× |
| 1 | 5 | 0.285 | 0.435 | 1.53× |
| 1 | 10 | 0.384 | 0.852 | 2.22× |
| 10 | 1 | 0.046 | 0.086 | 1.87× |
| 10 | 5 | 0.183 | 0.427 | 2.33× |
| 10 | 10 | 0.383 | 0.894 | 2.34× |
| 100 | 1 | 0.044 | 0.098 | 2.22× |
| 100 | 5 | 0.195 | 0.488 | 2.50× |
| 100 | 10 | 0.409 | 0.986 | 2.41× |
| 1,000 | 1 | 0.044 | 0.270 | 6.12× |
| 1,000 | 5 | 0.197 | 1.442 | 7.33× |
| 1,000 | 10 | 0.552 | 2.711 | 4.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
