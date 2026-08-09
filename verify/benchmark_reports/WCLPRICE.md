# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.93M | 0.004 | 237.65M | 0.030 | 5.08× | 7.15× |
| 10,000 | 0.013 | 772.17M | 0.010 | 999.68M | 0.040 | 3.10× | 4.02× |
| 100,000 | 0.096 | 1.04G | 0.066 | 1.52G | 0.093 | 0.97× | 1.42× |
| 1,000,000 | 1.887 | 529.86M | 1.328 | 752.88M | 1.393 | 0.74× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.159 | 1.66× |
| 1 | 5 | 0.323 | 0.561 | 1.74× |
| 1 | 10 | 0.639 | 1.205 | 1.89× |
| 10 | 1 | 0.066 | 0.118 | 1.81× |
| 10 | 5 | 0.315 | 0.587 | 1.87× |
| 10 | 10 | 0.637 | 1.122 | 1.76× |
| 100 | 1 | 0.057 | 0.099 | 1.74× |
| 100 | 5 | 0.321 | 0.627 | 1.95× |
| 100 | 10 | 0.661 | 1.187 | 1.80× |
| 1,000 | 1 | 0.058 | 0.112 | 1.91× |
| 1,000 | 5 | 0.318 | 0.553 | 1.74× |
| 1,000 | 10 | 0.647 | 1.151 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
