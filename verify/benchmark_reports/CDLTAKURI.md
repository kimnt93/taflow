# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.59M | 0.008 | 133.17M | 0.036 | 3.90× | 4.83× |
| 10,000 | 0.059 | 170.41M | 0.058 | 173.82M | 0.111 | 1.89× | 1.93× |
| 100,000 | 0.628 | 159.35M | 0.612 | 163.33M | 0.804 | 1.28× | 1.31× |
| 1,000,000 | 6.479 | 154.35M | 6.575 | 152.09M | 8.087 | 1.25× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.157 | 1.20× |
| 1 | 5 | 0.320 | 0.489 | 1.53× |
| 1 | 10 | 0.502 | 0.917 | 1.83× |
| 10 | 1 | 0.054 | 0.092 | 1.71× |
| 10 | 5 | 0.247 | 0.423 | 1.71× |
| 10 | 10 | 0.509 | 0.934 | 1.84× |
| 100 | 1 | 0.053 | 0.095 | 1.78× |
| 100 | 5 | 0.246 | 0.430 | 1.75× |
| 100 | 10 | 0.537 | 0.947 | 1.77× |
| 1,000 | 1 | 0.062 | 0.110 | 1.79× |
| 1,000 | 5 | 0.244 | 0.487 | 1.99× |
| 1,000 | 10 | 0.582 | 1.019 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
