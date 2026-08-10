# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.45M | 0.016 | 60.83M | 0.032 | 1.70× | 1.97× |
| 10,000 | 0.168 | 59.51M | 0.162 | 61.81M | 0.142 | 0.85× | 0.88× |
| 100,000 | 1.614 | 61.97M | 1.699 | 58.85M | 1.107 | 0.69× | 0.65× |
| 1,000,000 | 17.143 | 58.33M | 16.707 | 59.85M | 10.882 | 0.63× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.122 | 0.96× |
| 1 | 5 | 0.325 | 0.439 | 1.35× |
| 1 | 10 | 0.522 | 0.907 | 1.74× |
| 10 | 1 | 0.063 | 0.089 | 1.40× |
| 10 | 5 | 0.277 | 0.445 | 1.61× |
| 10 | 10 | 0.575 | 0.948 | 1.65× |
| 100 | 1 | 0.066 | 0.097 | 1.46× |
| 100 | 5 | 0.265 | 0.429 | 1.62× |
| 100 | 10 | 0.537 | 0.886 | 1.65× |
| 1,000 | 1 | 0.072 | 0.102 | 1.43× |
| 1,000 | 5 | 0.272 | 0.482 | 1.77× |
| 1,000 | 10 | 0.572 | 1.016 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
