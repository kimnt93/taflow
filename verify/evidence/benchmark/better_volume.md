# BetterVolume benchmark (`BetterVolume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.54M | 0.019 | 53.50M | 0.273 | 12.44× | 14.62× |
| 10,000 | 0.186 | 53.77M | 0.174 | 57.53M | 1.462 | 7.86× | 8.41× |
| 100,000 | 1.765 | 56.66M | 1.786 | 56.00M | 13.997 | 7.93× | 7.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.283 | 3.44× |
| 1 | 5 | 0.268 | 1.067 | 3.98× |
| 1 | 10 | 0.429 | 2.443 | 5.69× |
| 10 | 1 | 0.046 | 0.226 | 4.89× |
| 10 | 5 | 0.188 | 1.059 | 5.63× |
| 10 | 10 | 0.410 | 2.264 | 5.52× |
| 100 | 1 | 0.046 | 0.231 | 5.02× |
| 100 | 5 | 0.193 | 1.333 | 6.91× |
| 100 | 10 | 0.430 | 2.370 | 5.52× |
| 1,000 | 1 | 0.066 | 0.370 | 5.58× |
| 1,000 | 5 | 0.238 | 1.962 | 8.23× |
| 1,000 | 10 | 0.439 | 3.842 | 8.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
