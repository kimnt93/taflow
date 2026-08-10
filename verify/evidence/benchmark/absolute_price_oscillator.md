# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.17M | 0.007 | 133.58M | 0.043 | 5.13× | 5.70× |
| 10,000 | 0.050 | 201.01M | 0.051 | 195.36M | 0.080 | 1.60× | 1.56× |
| 100,000 | 0.487 | 205.17M | 0.458 | 218.41M | 0.456 | 0.94× | 1.00× |
| 1,000,000 | 5.258 | 190.18M | 4.677 | 213.83M | 5.248 | 1.00× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.117 | 0.99× |
| 1 | 5 | 0.293 | 0.482 | 1.65× |
| 1 | 10 | 0.508 | 1.137 | 2.24× |
| 10 | 1 | 0.056 | 0.101 | 1.79× |
| 10 | 5 | 0.240 | 0.463 | 1.93× |
| 10 | 10 | 0.462 | 1.094 | 2.37× |
| 100 | 1 | 0.057 | 0.115 | 2.01× |
| 100 | 5 | 0.309 | 0.505 | 1.63× |
| 100 | 10 | 0.495 | 0.981 | 1.98× |
| 1,000 | 1 | 0.054 | 0.107 | 2.00× |
| 1,000 | 5 | 0.287 | 0.568 | 1.98× |
| 1,000 | 10 | 0.562 | 1.052 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
