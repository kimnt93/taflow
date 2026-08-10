# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.33M | 0.020 | 49.59M | 0.053 | 2.13× | 2.62× |
| 10,000 | 0.190 | 52.57M | 0.193 | 51.75M | 0.226 | 1.19× | 1.17× |
| 100,000 | 1.962 | 50.97M | 1.939 | 51.56M | 1.883 | 0.96× | 0.97× |
| 1,000,000 | 20.676 | 48.36M | 19.793 | 50.52M | 18.880 | 0.91× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.189 | 2.20× |
| 1 | 5 | 0.416 | 0.584 | 1.40× |
| 1 | 10 | 0.697 | 1.138 | 1.63× |
| 10 | 1 | 0.078 | 0.099 | 1.26× |
| 10 | 5 | 0.362 | 0.637 | 1.76× |
| 10 | 10 | 0.674 | 1.277 | 1.90× |
| 100 | 1 | 0.060 | 0.115 | 1.93× |
| 100 | 5 | 0.350 | 0.694 | 1.98× |
| 100 | 10 | 0.783 | 1.136 | 1.45× |
| 1,000 | 1 | 0.105 | 0.174 | 1.67× |
| 1,000 | 5 | 0.412 | 0.698 | 1.69× |
| 1,000 | 10 | 0.733 | 1.497 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
