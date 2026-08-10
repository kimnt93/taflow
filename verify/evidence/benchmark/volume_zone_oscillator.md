# VolumeZoneOscillator benchmark (`VZO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.73M | 0.014 | 71.13M | 0.205 | 11.81× | 14.55× |
| 10,000 | 0.115 | 86.76M | 0.117 | 85.79M | 0.837 | 7.26× | 7.18× |
| 100,000 | 1.112 | 89.94M | 1.108 | 90.27M | 7.362 | 6.62× | 6.65× |
| 1,000,000 | 11.567 | 86.45M | 11.326 | 88.29M | 70.928 | 6.13× | 6.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.290 | 4.26× |
| 1 | 5 | 0.257 | 1.210 | 4.72× |
| 1 | 10 | 0.507 | 2.252 | 4.44× |
| 10 | 1 | 0.063 | 0.218 | 3.46× |
| 10 | 5 | 0.229 | 1.245 | 5.43× |
| 10 | 10 | 0.508 | 2.298 | 4.52× |
| 100 | 1 | 0.056 | 0.231 | 4.09× |
| 100 | 5 | 0.244 | 1.278 | 5.25× |
| 100 | 10 | 0.528 | 2.396 | 4.54× |
| 1,000 | 1 | 0.073 | 0.292 | 3.99× |
| 1,000 | 5 | 0.271 | 1.653 | 6.10× |
| 1,000 | 10 | 0.551 | 3.093 | 5.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
