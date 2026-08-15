# McClellanSummationIndex benchmark (`McClellanSummationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.90M | 0.006 | 169.40M | 8.275 | 1108.00× | 1401.72× |
| 10,000 | 0.057 | 174.65M | 0.051 | 194.30M | 83.702 | 1461.85× | 1626.32× |
| 100,000 | 0.514 | 194.62M | 0.515 | 194.13M | 824.306 | 1604.30× | 1600.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.262 | 2.89× |
| 1 | 5 | 0.226 | 1.062 | 4.71× |
| 1 | 10 | 0.451 | 2.140 | 4.74× |
| 10 | 1 | 0.048 | 0.288 | 5.95× |
| 10 | 5 | 0.189 | 1.812 | 9.57× |
| 10 | 10 | 0.406 | 2.914 | 7.18× |
| 100 | 1 | 0.052 | 1.222 | 23.50× |
| 100 | 5 | 0.190 | 5.703 | 29.95× |
| 100 | 10 | 0.400 | 11.082 | 27.70× |
| 1,000 | 1 | 0.053 | 8.638 | 163.93× |
| 1,000 | 5 | 0.289 | 44.510 | 153.88× |
| 1,000 | 10 | 0.515 | 90.680 | 176.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
