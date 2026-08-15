# UpDownVolumeRatio benchmark (`UpDownVolumeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 214.00M | 0.003 | 310.44M | 4.239 | 907.15× | 1315.94× |
| 10,000 | 0.023 | 427.29M | 0.021 | 468.07M | 42.116 | 1799.57× | 1971.32× |
| 100,000 | 0.231 | 432.66M | 0.200 | 500.00M | 410.985 | 1778.16× | 2054.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.271 | 1.55× |
| 1 | 5 | 0.299 | 0.971 | 3.25× |
| 1 | 10 | 0.407 | 1.897 | 4.66× |
| 10 | 1 | 0.043 | 0.244 | 5.62× |
| 10 | 5 | 0.194 | 1.462 | 7.54× |
| 10 | 10 | 0.383 | 2.549 | 6.66× |
| 100 | 1 | 0.044 | 0.636 | 14.60× |
| 100 | 5 | 0.195 | 3.046 | 15.62× |
| 100 | 10 | 0.395 | 6.534 | 16.55× |
| 1,000 | 1 | 0.047 | 4.615 | 97.34× |
| 1,000 | 5 | 0.284 | 23.100 | 81.47× |
| 1,000 | 10 | 0.519 | 46.430 | 89.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
