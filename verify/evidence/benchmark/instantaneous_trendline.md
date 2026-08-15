# InstantaneousTrendline benchmark (`InstantaneousTrendline` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.04M | 0.007 | 139.85M | 0.164 | 22.49× | 22.95× |
| 10,000 | 0.057 | 175.66M | 0.053 | 187.14M | 0.494 | 8.67× | 9.24× |
| 100,000 | 0.519 | 192.56M | 0.500 | 200.07M | 3.531 | 6.80× | 7.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.234 | 2.85× |
| 1 | 5 | 0.254 | 1.005 | 3.96× |
| 1 | 10 | 0.430 | 2.218 | 5.15× |
| 10 | 1 | 0.041 | 0.191 | 4.69× |
| 10 | 5 | 0.206 | 0.949 | 4.61× |
| 10 | 10 | 0.418 | 2.094 | 5.01× |
| 100 | 1 | 0.044 | 0.194 | 4.38× |
| 100 | 5 | 0.208 | 1.022 | 4.92× |
| 100 | 10 | 0.424 | 2.166 | 5.11× |
| 1,000 | 1 | 0.055 | 0.248 | 4.51× |
| 1,000 | 5 | 0.218 | 1.161 | 5.32× |
| 1,000 | 10 | 0.430 | 2.488 | 5.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
