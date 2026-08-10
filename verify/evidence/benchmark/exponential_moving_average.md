# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 181.94M | 0.005 | 204.32M | 0.037 | 6.65× | 7.47× |
| 10,000 | 0.030 | 334.55M | 0.029 | 348.93M | 0.065 | 2.16× | 2.25× |
| 100,000 | 0.320 | 312.99M | 0.269 | 371.14M | 0.338 | 1.06× | 1.25× |
| 1,000,000 | 4.033 | 247.95M | 3.478 | 287.50M | 2.902 | 0.72× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.129 | 1.46× |
| 1 | 5 | 0.336 | 0.502 | 1.49× |
| 1 | 10 | 0.490 | 0.981 | 2.00× |
| 10 | 1 | 0.055 | 0.102 | 1.83× |
| 10 | 5 | 0.242 | 0.484 | 2.00× |
| 10 | 10 | 0.477 | 0.992 | 2.08× |
| 100 | 1 | 0.061 | 0.097 | 1.58× |
| 100 | 5 | 0.245 | 0.507 | 2.07× |
| 100 | 10 | 0.520 | 1.090 | 2.10× |
| 1,000 | 1 | 0.062 | 0.142 | 2.30× |
| 1,000 | 5 | 0.312 | 0.541 | 1.74× |
| 1,000 | 10 | 0.600 | 1.109 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
