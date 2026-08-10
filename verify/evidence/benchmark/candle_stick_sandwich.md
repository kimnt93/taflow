# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.00M | 0.009 | 114.08M | 0.036 | 2.96× | 4.07× |
| 10,000 | 0.053 | 189.79M | 0.048 | 210.23M | 0.091 | 1.73× | 1.92× |
| 100,000 | 0.592 | 168.91M | 0.572 | 174.91M | 0.638 | 1.08× | 1.12× |
| 1,000,000 | 6.615 | 151.18M | 6.312 | 158.43M | 6.332 | 0.96× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.098 | 0.77× |
| 1 | 5 | 0.333 | 0.444 | 1.33× |
| 1 | 10 | 0.561 | 0.938 | 1.67× |
| 10 | 1 | 0.053 | 0.093 | 1.73× |
| 10 | 5 | 0.250 | 0.435 | 1.74× |
| 10 | 10 | 0.522 | 0.958 | 1.83× |
| 100 | 1 | 0.075 | 0.099 | 1.32× |
| 100 | 5 | 0.261 | 0.435 | 1.66× |
| 100 | 10 | 0.543 | 0.955 | 1.76× |
| 1,000 | 1 | 0.066 | 0.096 | 1.46× |
| 1,000 | 5 | 0.289 | 0.517 | 1.79× |
| 1,000 | 10 | 0.550 | 0.942 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
