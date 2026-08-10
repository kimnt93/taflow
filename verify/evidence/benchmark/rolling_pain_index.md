# RollingPainIndex benchmark (`PainIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.27M | 0.063 | 15.86M | 0.202 | 4.50× | 3.20× |
| 10,000 | 0.456 | 21.95M | 0.442 | 22.64M | 0.641 | 1.41× | 1.45× |
| 100,000 | 4.379 | 22.84M | 4.336 | 23.07M | 5.580 | 1.27× | 1.29× |
| 1,000,000 | 44.975 | 22.23M | 44.613 | 22.42M | 56.207 | 1.25× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.295 | 3.94× |
| 1 | 5 | 0.330 | 1.077 | 3.27× |
| 1 | 10 | 0.512 | 2.282 | 4.46× |
| 10 | 1 | 0.057 | 0.190 | 3.33× |
| 10 | 5 | 0.236 | 1.020 | 4.32× |
| 10 | 10 | 0.499 | 2.145 | 4.30× |
| 100 | 1 | 0.053 | 0.202 | 3.78× |
| 100 | 5 | 0.245 | 0.979 | 4.00× |
| 100 | 10 | 0.480 | 2.158 | 4.50× |
| 1,000 | 1 | 0.096 | 0.258 | 2.71× |
| 1,000 | 5 | 0.225 | 1.231 | 5.47× |
| 1,000 | 10 | 0.509 | 2.737 | 5.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
