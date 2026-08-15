# OvernightIntradayReturn benchmark (`OvernightIntradayReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.32M | 0.009 | 116.51M | 0.641 | 49.56× | 74.68× |
| 10,000 | 0.077 | 129.82M | 0.070 | 143.28M | 4.924 | 63.92× | 70.55× |
| 100,000 | 0.669 | 149.57M | 0.627 | 159.52M | 54.016 | 80.79× | 86.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.295 | 2.80× |
| 1 | 5 | 0.244 | 1.182 | 4.84× |
| 1 | 10 | 0.412 | 2.402 | 5.83× |
| 10 | 1 | 0.052 | 0.227 | 4.36× |
| 10 | 5 | 0.195 | 1.327 | 6.80× |
| 10 | 10 | 0.447 | 2.635 | 5.89× |
| 100 | 1 | 0.051 | 0.280 | 5.49× |
| 100 | 5 | 0.210 | 1.560 | 7.42× |
| 100 | 10 | 0.454 | 2.962 | 6.52× |
| 1,000 | 1 | 0.057 | 0.894 | 15.75× |
| 1,000 | 5 | 0.220 | 4.096 | 18.61× |
| 1,000 | 10 | 0.435 | 8.080 | 18.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
