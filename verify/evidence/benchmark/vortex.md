# Vortex benchmark (`Vortex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.68M | 0.011 | 90.60M | 0.502 | 36.47× | 45.46× |
| 10,000 | 0.100 | 100.18M | 0.095 | 105.07M | 3.833 | 38.40× | 40.28× |
| 100,000 | 0.943 | 106.08M | 0.920 | 108.70M | 40.726 | 43.20× | 44.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.275 | 3.23× |
| 1 | 5 | 0.298 | 1.022 | 3.43× |
| 1 | 10 | 0.382 | 2.204 | 5.76× |
| 10 | 1 | 0.045 | 0.211 | 4.65× |
| 10 | 5 | 0.190 | 1.370 | 7.21× |
| 10 | 10 | 0.407 | 2.289 | 5.63× |
| 100 | 1 | 0.053 | 0.249 | 4.69× |
| 100 | 5 | 0.204 | 1.527 | 7.49× |
| 100 | 10 | 0.433 | 2.685 | 6.20× |
| 1,000 | 1 | 0.057 | 0.820 | 14.29× |
| 1,000 | 5 | 0.209 | 3.327 | 15.91× |
| 1,000 | 10 | 0.435 | 7.244 | 16.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
