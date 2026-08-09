# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.34M | 0.010 | 96.60M | 0.040 | 3.83× | 3.88× |
| 10,000 | 0.124 | 80.83M | 0.121 | 82.49M | 0.173 | 1.40× | 1.43× |
| 100,000 | 1.265 | 79.04M | 1.250 | 80.01M | 1.433 | 1.13× | 1.15× |
| 1,000,000 | 12.877 | 77.66M | 12.439 | 80.39M | 14.185 | 1.10× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.120 | 0.97× |
| 1 | 5 | 0.303 | 0.516 | 1.70× |
| 1 | 10 | 0.507 | 0.934 | 1.84× |
| 10 | 1 | 0.053 | 0.091 | 1.72× |
| 10 | 5 | 0.233 | 0.416 | 1.78× |
| 10 | 10 | 0.559 | 1.028 | 1.84× |
| 100 | 1 | 0.054 | 0.092 | 1.72× |
| 100 | 5 | 0.254 | 0.447 | 1.76× |
| 100 | 10 | 0.527 | 0.930 | 1.76× |
| 1,000 | 1 | 0.068 | 0.109 | 1.61× |
| 1,000 | 5 | 0.260 | 0.543 | 2.09× |
| 1,000 | 10 | 0.549 | 1.153 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
