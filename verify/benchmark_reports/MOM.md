# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 203.92M | 0.004 | 261.20M | 0.032 | 6.56× | 8.41× |
| 10,000 | 0.021 | 475.48M | 0.018 | 543.37M | 0.035 | 1.66× | 1.90× |
| 100,000 | 0.184 | 543.59M | 0.160 | 624.94M | 0.063 | 0.34× | 0.39× |
| 1,000,000 | 2.099 | 476.42M | 1.790 | 558.54M | 0.555 | 0.26× | 0.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.153 | 0.156 | 1.02× |
| 1 | 5 | 0.362 | 0.520 | 1.44× |
| 1 | 10 | 0.462 | 0.928 | 2.01× |
| 10 | 1 | 0.050 | 0.101 | 2.04× |
| 10 | 5 | 0.215 | 0.440 | 2.05× |
| 10 | 10 | 0.444 | 0.937 | 2.11× |
| 100 | 1 | 0.048 | 0.093 | 1.96× |
| 100 | 5 | 0.219 | 0.430 | 1.96× |
| 100 | 10 | 0.453 | 0.914 | 2.02× |
| 1,000 | 1 | 0.053 | 0.094 | 1.77× |
| 1,000 | 5 | 0.236 | 0.458 | 1.94× |
| 1,000 | 10 | 0.481 | 0.978 | 2.03× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.168 | 5.95M | 61.062 | 363.32× | 179.13× |
| 100,000 | 10 | 0.852 | 0.478 | 20.91M | 60.322 | 126.13× | 66.84× |
| 100,000 | 1,000 | 4.088 | 3.006 | 332.68M | 60.585 | 20.16× | 9.99× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 243.07M | 378.47M | 1.00× | 3.70M | 3.14M | 1.00× | 632.21M |
| 5 | 526.56M | 1.05G | 2.77× | 3.09M | 3.40M | 1.08× | 615.13M |
| 10 | 573.74M | 1.26G | 3.34× | 2.85M | 3.15M | 1.00× | 587.05M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
