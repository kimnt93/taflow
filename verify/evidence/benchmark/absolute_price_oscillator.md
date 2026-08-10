# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.83M | 0.007 | 139.71M | 0.037 | 4.74× | 5.18× |
| 10,000 | 0.046 | 215.76M | 0.044 | 229.54M | 0.073 | 1.58× | 1.69× |
| 100,000 | 0.590 | 169.37M | 0.402 | 248.77M | 0.467 | 0.79× | 1.16× |
| 1,000,000 | 4.700 | 212.76M | 4.214 | 237.33M | 4.534 | 0.96× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.187 | 2.34× |
| 1 | 5 | 0.374 | 0.488 | 1.31× |
| 1 | 10 | 0.456 | 0.936 | 2.05× |
| 10 | 1 | 0.048 | 0.094 | 1.97× |
| 10 | 5 | 0.209 | 0.445 | 2.13× |
| 10 | 10 | 0.469 | 0.936 | 2.00× |
| 100 | 1 | 0.046 | 0.100 | 2.15× |
| 100 | 5 | 0.227 | 0.448 | 1.98× |
| 100 | 10 | 0.473 | 0.965 | 2.04× |
| 1,000 | 1 | 0.058 | 0.098 | 1.70× |
| 1,000 | 5 | 0.259 | 0.499 | 1.92× |
| 1,000 | 10 | 0.555 | 1.047 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
