# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.49M | 0.017 | 57.92M | 0.042 | 2.14× | 2.45× |
| 10,000 | 0.161 | 62.17M | 0.151 | 66.38M | 0.177 | 1.10× | 1.18× |
| 100,000 | 1.542 | 64.84M | 1.542 | 64.85M | 1.450 | 0.94× | 0.94× |
| 1,000,000 | 15.987 | 62.55M | 15.703 | 63.68M | 14.845 | 0.93× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.146 | 1.33× |
| 1 | 5 | 0.342 | 0.505 | 1.48× |
| 1 | 10 | 0.540 | 0.932 | 1.73× |
| 10 | 1 | 0.062 | 0.094 | 1.51× |
| 10 | 5 | 0.253 | 0.419 | 1.65× |
| 10 | 10 | 0.507 | 0.908 | 1.79× |
| 100 | 1 | 0.058 | 0.092 | 1.60× |
| 100 | 5 | 0.270 | 0.442 | 1.64× |
| 100 | 10 | 0.538 | 0.906 | 1.68× |
| 1,000 | 1 | 0.074 | 0.121 | 1.64× |
| 1,000 | 5 | 0.280 | 0.536 | 1.91× |
| 1,000 | 10 | 0.586 | 1.144 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
