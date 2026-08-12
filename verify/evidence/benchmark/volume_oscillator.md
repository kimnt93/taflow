# VolumeOscillator benchmark (`VolumeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.12M | 0.018 | 55.52M | 0.213 | 13.87× | 11.82× |
| 10,000 | 0.119 | 84.17M | 0.114 | 87.72M | 0.950 | 8.00× | 8.34× |
| 100,000 | 1.100 | 90.94M | 1.059 | 94.40M | 4.296 | 3.91× | 4.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | 0.317 | 1.91× |
| 1 | 5 | 0.303 | 1.465 | 4.83× |
| 1 | 10 | 0.485 | 2.903 | 5.98× |
| 10 | 1 | 0.072 | 0.258 | 3.57× |
| 10 | 5 | 0.250 | 1.446 | 5.80× |
| 10 | 10 | 0.516 | 2.813 | 5.45× |
| 100 | 1 | 0.055 | 0.266 | 4.87× |
| 100 | 5 | 0.271 | 1.493 | 5.51× |
| 100 | 10 | 0.515 | 2.970 | 5.77× |
| 1,000 | 1 | 0.070 | 0.291 | 4.18× |
| 1,000 | 5 | 0.239 | 1.623 | 6.78× |
| 1,000 | 10 | 0.556 | 3.160 | 5.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
