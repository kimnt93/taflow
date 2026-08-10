# TradeVolumeIndex benchmark (`TradeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.63M | 0.006 | 159.35M | 0.194 | 26.54× | 30.96× |
| 10,000 | 0.066 | 150.94M | 0.062 | 162.35M | 0.809 | 12.21× | 13.14× |
| 100,000 | 0.719 | 139.09M | 0.719 | 139.15M | 6.963 | 9.69× | 9.69× |
| 1,000,000 | 7.420 | 134.77M | 6.949 | 143.90M | 67.167 | 9.05× | 9.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.270 | 4.70× |
| 1 | 5 | 0.283 | 1.332 | 4.71× |
| 1 | 10 | 0.522 | 2.428 | 4.65× |
| 10 | 1 | 0.059 | 0.228 | 3.86× |
| 10 | 5 | 0.299 | 1.351 | 4.53× |
| 10 | 10 | 0.504 | 2.407 | 4.78× |
| 100 | 1 | 0.058 | 0.230 | 3.96× |
| 100 | 5 | 0.245 | 1.264 | 5.16× |
| 100 | 10 | 0.542 | 2.433 | 4.49× |
| 1,000 | 1 | 0.058 | 0.280 | 4.79× |
| 1,000 | 5 | 0.244 | 1.711 | 7.03× |
| 1,000 | 10 | 0.503 | 3.122 | 6.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
