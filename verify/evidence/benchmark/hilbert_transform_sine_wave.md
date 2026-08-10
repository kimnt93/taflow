# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.132 | 7.56M | 0.128 | 7.82M | 0.455 | 3.44× | 3.56× |
| 10,000 | 1.333 | 7.50M | 1.331 | 7.51M | 4.845 | 3.64× | 3.64× |
| 100,000 | 14.960 | 6.68M | 13.704 | 7.30M | 46.499 | 3.11× | 3.39× |
| 1,000,000 | 135.626 | 7.37M | 136.807 | 7.31M | 442.350 | 3.26× | 3.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.131 | 1.43× |
| 1 | 5 | 0.330 | 0.481 | 1.46× |
| 1 | 10 | 0.475 | 0.940 | 1.98× |
| 10 | 1 | 0.049 | 0.093 | 1.91× |
| 10 | 5 | 0.227 | 0.423 | 1.87× |
| 10 | 10 | 0.482 | 0.923 | 1.91× |
| 100 | 1 | 0.060 | 0.115 | 1.92× |
| 100 | 5 | 0.212 | 0.553 | 2.60× |
| 100 | 10 | 0.493 | 1.179 | 2.39× |
| 1,000 | 1 | 0.190 | 0.548 | 2.89× |
| 1,000 | 5 | 0.340 | 2.699 | 7.93× |
| 1,000 | 10 | 0.566 | 5.418 | 9.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
