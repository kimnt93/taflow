# ProjectionBands benchmark (`rolling projection mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.10M | 0.016 | 60.90M | 0.081 | 4.88× | 4.94× |
| 10,000 | 0.145 | 69.05M | 0.143 | 70.02M | 0.282 | 1.94× | 1.97× |
| 100,000 | 1.415 | 70.66M | 1.380 | 72.48M | 2.207 | 1.56× | 1.60× |
| 1,000,000 | 14.193 | 70.46M | 14.108 | 70.88M | 22.308 | 1.57× | 1.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.164 | 2.22× |
| 1 | 5 | 0.326 | 0.557 | 1.71× |
| 1 | 10 | 0.514 | 1.044 | 2.03× |
| 10 | 1 | 0.048 | 0.111 | 2.32× |
| 10 | 5 | 0.231 | 0.497 | 2.15× |
| 10 | 10 | 0.474 | 1.089 | 2.30× |
| 100 | 1 | 0.058 | 0.150 | 2.58× |
| 100 | 5 | 0.230 | 0.703 | 3.05× |
| 100 | 10 | 0.488 | 1.413 | 2.89× |
| 1,000 | 1 | 0.066 | 0.157 | 2.38× |
| 1,000 | 5 | 0.242 | 0.753 | 3.11× |
| 1,000 | 10 | 0.498 | 1.680 | 3.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
