# RollingGainLossRatio benchmark (`GainLossRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.09M | 0.023 | 43.57M | 0.164 | 7.72× | 7.14× |
| 10,000 | 0.201 | 49.77M | 0.191 | 52.42M | 0.576 | 2.87× | 3.02× |
| 100,000 | 2.004 | 49.91M | 1.922 | 52.02M | 5.474 | 2.73× | 2.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.053 | 0.241 | 4.55× |
| 1 | 5 | 0.213 | 0.930 | 4.36× |
| 1 | 10 | 0.376 | 2.096 | 5.57× |
| 10 | 1 | 0.050 | 0.193 | 3.87× |
| 10 | 5 | 0.198 | 0.929 | 4.69× |
| 10 | 10 | 0.407 | 2.237 | 5.50× |
| 100 | 1 | 0.052 | 0.201 | 3.87× |
| 100 | 5 | 0.199 | 0.954 | 4.80× |
| 100 | 10 | 0.420 | 2.203 | 5.24× |
| 1,000 | 1 | 0.068 | 0.249 | 3.67× |
| 1,000 | 5 | 0.234 | 1.199 | 5.12× |
| 1,000 | 10 | 0.452 | 2.576 | 5.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
