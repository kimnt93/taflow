# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.90M | 0.018 | 56.51M | 0.054 | 2.70× | 3.05× |
| 10,000 | 0.150 | 66.53M | 0.145 | 68.99M | 0.210 | 1.40× | 1.45× |
| 100,000 | 1.541 | 64.88M | 1.482 | 67.47M | 1.794 | 1.16× | 1.21× |
| 1,000,000 | 16.748 | 59.71M | 14.572 | 68.63M | 16.002 | 0.96× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.124 | 1.38× |
| 1 | 5 | 0.283 | 0.541 | 1.91× |
| 1 | 10 | 0.650 | 1.260 | 1.94× |
| 10 | 1 | 0.061 | 0.107 | 1.75× |
| 10 | 5 | 0.276 | 0.551 | 2.00× |
| 10 | 10 | 0.533 | 1.145 | 2.15× |
| 100 | 1 | 0.072 | 0.128 | 1.78× |
| 100 | 5 | 0.283 | 0.565 | 2.00× |
| 100 | 10 | 0.543 | 1.074 | 1.98× |
| 1,000 | 1 | 0.078 | 0.136 | 1.76× |
| 1,000 | 5 | 0.325 | 0.676 | 2.08× |
| 1,000 | 10 | 0.562 | 1.177 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
