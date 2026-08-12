# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.19M | 0.013 | 76.76M | 0.041 | 2.85× | 3.12× |
| 10,000 | 0.098 | 101.59M | 0.099 | 100.99M | 0.098 | 1.00× | 0.99× |
| 100,000 | 0.921 | 108.56M | 0.943 | 106.00M | 0.717 | 0.78× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.162 | 1.21× |
| 1 | 5 | 0.300 | 0.522 | 1.74× |
| 1 | 10 | 0.535 | 0.964 | 1.80× |
| 10 | 1 | 0.049 | 0.090 | 1.83× |
| 10 | 5 | 0.222 | 0.443 | 2.00× |
| 10 | 10 | 0.479 | 0.986 | 2.06× |
| 100 | 1 | 0.062 | 0.099 | 1.60× |
| 100 | 5 | 0.225 | 0.463 | 2.06× |
| 100 | 10 | 0.476 | 0.992 | 2.09× |
| 1,000 | 1 | 0.065 | 0.114 | 1.75× |
| 1,000 | 5 | 0.293 | 0.511 | 1.74× |
| 1,000 | 10 | 0.512 | 1.023 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
