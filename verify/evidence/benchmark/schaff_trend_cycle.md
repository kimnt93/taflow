# SchaffTrendCycle benchmark (`stc` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.07M | 0.060 | 16.55M | 30.747 | 494.06× | 508.71× |
| 10,000 | 0.664 | 15.07M | 0.633 | 15.79M | 300.357 | 452.52× | 474.34× |
| 100,000 | 6.524 | 15.33M | 6.207 | 16.11M | 3051.284 | 467.69× | 491.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.243 | 1.90× |
| 1 | 5 | 0.262 | 1.004 | 3.83× |
| 1 | 10 | 0.495 | 2.170 | 4.39× |
| 10 | 1 | 0.052 | 0.200 | 3.82× |
| 10 | 5 | 0.241 | 1.001 | 4.16× |
| 10 | 10 | 0.478 | 2.057 | 4.30× |
| 100 | 1 | 0.070 | 4.836 | 68.88× |
| 100 | 5 | 0.294 | 26.705 | 90.70× |
| 100 | 10 | 0.565 | 51.326 | 90.84× |
| 1,000 | 1 | 0.150 | 31.169 | 207.90× |
| 1,000 | 5 | 0.427 | 163.461 | 383.02× |
| 1,000 | 10 | 0.624 | 333.733 | 534.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
