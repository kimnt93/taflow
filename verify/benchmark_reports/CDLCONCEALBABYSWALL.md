# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.06M | 0.007 | 142.48M | 0.034 | 3.82× | 4.86× |
| 10,000 | 0.043 | 231.49M | 0.040 | 250.00M | 0.088 | 2.05× | 2.21× |
| 100,000 | 0.530 | 188.73M | 0.524 | 190.80M | 0.638 | 1.20× | 1.22× |
| 1,000,000 | 5.729 | 174.54M | 5.392 | 185.47M | 6.578 | 1.15× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.157 | 1.15× |
| 1 | 5 | 0.393 | 0.504 | 1.28× |
| 1 | 10 | 0.529 | 0.939 | 1.78× |
| 10 | 1 | 0.052 | 0.092 | 1.76× |
| 10 | 5 | 0.240 | 0.420 | 1.75× |
| 10 | 10 | 0.512 | 0.934 | 1.83× |
| 100 | 1 | 0.055 | 0.094 | 1.69× |
| 100 | 5 | 0.255 | 0.449 | 1.76× |
| 100 | 10 | 0.515 | 0.936 | 1.82× |
| 1,000 | 1 | 0.064 | 0.098 | 1.53× |
| 1,000 | 5 | 0.262 | 0.472 | 1.80× |
| 1,000 | 10 | 0.523 | 0.970 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
