# SessionVolumeLevels benchmark (`anchored volume levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.83M | 0.057 | 17.61M | 14.532 | 244.63× | 255.96× |
| 10,000 | 0.477 | 20.95M | 0.464 | 21.57M | 139.260 | 291.74× | 300.38× |
| 100,000 | 5.052 | 19.80M | 4.840 | 20.66M | 1406.661 | 278.46× | 290.61× |
| 1,000,000 | 48.750 | 20.51M | 47.057 | 21.25M | 13965.298 | 286.47× | 296.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.145 | 2.03× |
| 1 | 5 | 0.271 | 0.771 | 2.84× |
| 1 | 10 | 0.566 | 1.605 | 2.83× |
| 10 | 1 | 0.069 | 0.403 | 5.84× |
| 10 | 5 | 0.268 | 2.005 | 7.47× |
| 10 | 10 | 0.511 | 4.170 | 8.16× |
| 100 | 1 | 0.070 | 2.023 | 28.98× |
| 100 | 5 | 0.278 | 10.741 | 38.69× |
| 100 | 10 | 0.645 | 21.666 | 33.61× |
| 1,000 | 1 | 0.116 | 15.186 | 131.25× |
| 1,000 | 5 | 0.638 | 84.951 | 133.09× |
| 1,000 | 10 | 1.096 | 168.546 | 153.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
