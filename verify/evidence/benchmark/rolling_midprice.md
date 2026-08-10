# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.12M | 0.008 | 119.69M | 0.041 | 4.40× | 4.91× |
| 10,000 | 0.083 | 121.13M | 0.080 | 125.46M | 0.110 | 1.33× | 1.38× |
| 100,000 | 0.824 | 121.37M | 0.754 | 132.56M | 0.719 | 0.87× | 0.95× |
| 1,000,000 | 9.817 | 101.87M | 8.734 | 114.50M | 7.397 | 0.75× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.107 | 1.15× |
| 1 | 5 | 0.288 | 0.498 | 1.73× |
| 1 | 10 | 0.520 | 0.974 | 1.87× |
| 10 | 1 | 0.060 | 0.099 | 1.66× |
| 10 | 5 | 0.284 | 0.500 | 1.76× |
| 10 | 10 | 0.531 | 1.028 | 1.94× |
| 100 | 1 | 0.062 | 0.124 | 1.99× |
| 100 | 5 | 0.312 | 0.577 | 1.85× |
| 100 | 10 | 0.563 | 1.075 | 1.91× |
| 1,000 | 1 | 0.061 | 0.108 | 1.77× |
| 1,000 | 5 | 0.265 | 0.590 | 2.22× |
| 1,000 | 10 | 0.550 | 1.718 | 3.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
