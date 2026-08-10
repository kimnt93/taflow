# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.50M | 0.008 | 125.95M | 0.029 | 3.01× | 3.70× |
| 10,000 | 0.041 | 244.01M | 0.037 | 269.57M | 0.044 | 1.07× | 1.18× |
| 100,000 | 0.401 | 249.32M | 0.301 | 332.05M | 0.162 | 0.40× | 0.54× |
| 1,000,000 | 4.556 | 219.51M | 4.084 | 244.87M | 2.056 | 0.45× | 0.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.110 | 1.34× |
| 1 | 5 | 0.266 | 0.446 | 1.68× |
| 1 | 10 | 0.525 | 1.020 | 1.94× |
| 10 | 1 | 0.070 | 0.089 | 1.28× |
| 10 | 5 | 0.248 | 0.466 | 1.88× |
| 10 | 10 | 0.534 | 1.043 | 1.95× |
| 100 | 1 | 0.062 | 0.113 | 1.84× |
| 100 | 5 | 0.299 | 0.446 | 1.49× |
| 100 | 10 | 0.530 | 0.935 | 1.76× |
| 1,000 | 1 | 0.070 | 0.100 | 1.43× |
| 1,000 | 5 | 0.323 | 0.483 | 1.49× |
| 1,000 | 10 | 0.620 | 1.015 | 1.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
