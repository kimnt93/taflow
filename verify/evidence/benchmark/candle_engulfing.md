# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.32M | 0.002 | 558.71M | 0.031 | 5.70× | 17.10× |
| 10,000 | 0.015 | 668.66M | 0.011 | 939.68M | 0.082 | 5.46× | 7.67× |
| 100,000 | 0.107 | 937.33M | 0.091 | 1.10G | 0.584 | 5.48× | 6.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.122 | 1.76× |
| 1 | 5 | 0.232 | 0.465 | 2.00× |
| 1 | 10 | 0.390 | 0.884 | 2.27× |
| 10 | 1 | 0.043 | 0.083 | 1.94× |
| 10 | 5 | 0.174 | 0.416 | 2.39× |
| 10 | 10 | 0.397 | 0.917 | 2.31× |
| 100 | 1 | 0.045 | 0.085 | 1.89× |
| 100 | 5 | 0.189 | 0.423 | 2.24× |
| 100 | 10 | 0.394 | 0.951 | 2.42× |
| 1,000 | 1 | 0.051 | 0.103 | 2.01× |
| 1,000 | 5 | 0.197 | 0.457 | 2.32× |
| 1,000 | 10 | 0.406 | 0.945 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
