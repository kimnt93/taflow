# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.77M | 0.009 | 110.34M | 0.047 | 4.36× | 5.19× |
| 10,000 | 0.088 | 113.73M | 0.082 | 121.31M | 0.135 | 1.54× | 1.64× |
| 100,000 | 0.847 | 118.10M | 0.794 | 125.90M | 1.180 | 1.39× | 1.49× |
| 1,000,000 | 12.971 | 77.10M | 9.108 | 109.80M | 8.899 | 0.69× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.170 | 1.42× |
| 1 | 5 | 0.285 | 0.576 | 2.02× |
| 1 | 10 | 0.495 | 1.593 | 3.22× |
| 10 | 1 | 0.083 | 0.137 | 1.65× |
| 10 | 5 | 0.289 | 0.531 | 1.84× |
| 10 | 10 | 0.522 | 1.087 | 2.08× |
| 100 | 1 | 0.064 | 0.139 | 2.18× |
| 100 | 5 | 0.295 | 0.532 | 1.80× |
| 100 | 10 | 0.556 | 1.045 | 1.88× |
| 1,000 | 1 | 0.078 | 0.133 | 1.72× |
| 1,000 | 5 | 0.325 | 0.662 | 2.04× |
| 1,000 | 10 | 0.579 | 1.156 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
