# ExponentiallyWeightedCovariance benchmark (`ewm covariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.70M | 0.008 | 122.89M | 1.211 | 128.01× | 148.82× |
| 10,000 | 0.054 | 185.55M | 0.048 | 206.88M | 11.599 | 215.22× | 239.96× |
| 100,000 | 0.483 | 207.08M | 0.609 | 164.28M | 118.763 | 245.94× | 195.11× |
| 1,000,000 | 5.451 | 183.44M | 4.586 | 218.05M | 1210.935 | 222.13× | 264.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.186 | 0.139 | 0.75× |
| 1 | 5 | 0.261 | 0.425 | 1.63× |
| 1 | 10 | 0.457 | 0.853 | 1.87× |
| 10 | 1 | 0.051 | 0.099 | 1.94× |
| 10 | 5 | 0.237 | 0.473 | 2.00× |
| 10 | 10 | 0.494 | 0.995 | 2.01× |
| 100 | 1 | 0.054 | 0.214 | 3.96× |
| 100 | 5 | 0.232 | 1.046 | 4.51× |
| 100 | 10 | 0.483 | 2.158 | 4.47× |
| 1,000 | 1 | 0.054 | 1.363 | 25.04× |
| 1,000 | 5 | 0.238 | 6.730 | 28.32× |
| 1,000 | 10 | 0.469 | 13.866 | 29.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
