# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.58M | 0.010 | 96.55M | 0.036 | 3.28× | 3.49× |
| 10,000 | 0.087 | 115.20M | 0.082 | 122.00M | 0.108 | 1.24× | 1.32× |
| 100,000 | 0.845 | 118.39M | 0.795 | 125.81M | 0.813 | 0.96× | 1.02× |
| 1,000,000 | 8.765 | 114.09M | 8.989 | 111.25M | 7.941 | 0.91× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.144 | 1.51× |
| 1 | 5 | 0.242 | 0.461 | 1.91× |
| 1 | 10 | 0.499 | 0.978 | 1.96× |
| 10 | 1 | 0.052 | 0.095 | 1.82× |
| 10 | 5 | 0.249 | 0.451 | 1.81× |
| 10 | 10 | 0.517 | 0.940 | 1.82× |
| 100 | 1 | 0.055 | 0.094 | 1.70× |
| 100 | 5 | 0.295 | 0.480 | 1.63× |
| 100 | 10 | 0.513 | 0.945 | 1.84× |
| 1,000 | 1 | 0.062 | 0.095 | 1.54× |
| 1,000 | 5 | 0.256 | 0.511 | 1.99× |
| 1,000 | 10 | 0.578 | 1.171 | 2.02× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.185 | 5.41M | 815.297 | 4407.06× | 141.87× |
| 100,000 | 10 | 1.097 | 0.630 | 15.87M | 815.728 | 1294.30× | 48.39× |
| 100,000 | 1,000 | 11.039 | 10.057 | 99.44M | 1084.447 | 107.83× | 3.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 105.01M | 83.54M | 1.00× | 2.53M | 3.35M | 1.00× | 110.34M |
| 5 | 285.47M | 343.97M | 4.12× | 2.03M | 2.38M | 0.71× | 103.15M |
| 10 | 386.33M | 517.97M | 6.20× | 1.91M | 2.57M | 0.77× | 106.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
