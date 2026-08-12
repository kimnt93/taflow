# KlingerVolumeOscillator benchmark (`KVO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.24M | 0.027 | 37.52M | 0.339 | 10.93× | 12.71× |
| 10,000 | 0.230 | 43.49M | 0.209 | 47.92M | 1.598 | 6.95× | 7.66× |
| 100,000 | 2.194 | 45.57M | 1.979 | 50.53M | 14.246 | 6.49× | 7.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.319 | 2.30× |
| 1 | 5 | 0.361 | 1.680 | 4.65× |
| 1 | 10 | 0.528 | 3.001 | 5.69× |
| 10 | 1 | 0.074 | 0.304 | 4.11× |
| 10 | 5 | 0.302 | 1.734 | 5.75× |
| 10 | 10 | 0.593 | 3.386 | 5.71× |
| 100 | 1 | 0.067 | 0.285 | 4.27× |
| 100 | 5 | 0.326 | 1.746 | 5.36× |
| 100 | 10 | 0.642 | 3.327 | 5.18× |
| 1,000 | 1 | 0.083 | 0.430 | 5.15× |
| 1,000 | 5 | 0.307 | 2.337 | 7.62× |
| 1,000 | 10 | 0.693 | 4.729 | 6.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
