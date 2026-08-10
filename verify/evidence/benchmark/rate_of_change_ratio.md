# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.61M | 0.005 | 187.67M | 0.042 | 4.54× | 7.85× |
| 10,000 | 0.027 | 368.59M | 0.025 | 406.86M | 0.049 | 1.80× | 1.99× |
| 100,000 | 0.230 | 434.35M | 0.219 | 457.13M | 0.157 | 0.68× | 0.72× |
| 1,000,000 | 2.917 | 342.82M | 2.082 | 480.42M | 1.454 | 0.50× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.150 | 1.49× |
| 1 | 5 | 0.291 | 0.494 | 1.70× |
| 1 | 10 | 0.599 | 1.054 | 1.76× |
| 10 | 1 | 0.051 | 0.089 | 1.75× |
| 10 | 5 | 0.286 | 0.457 | 1.60× |
| 10 | 10 | 0.533 | 1.188 | 2.23× |
| 100 | 1 | 0.063 | 0.098 | 1.55× |
| 100 | 5 | 0.260 | 0.492 | 1.89× |
| 100 | 10 | 0.555 | 1.198 | 2.16× |
| 1,000 | 1 | 0.060 | 0.102 | 1.70× |
| 1,000 | 5 | 0.249 | 0.461 | 1.86× |
| 1,000 | 10 | 0.563 | 1.100 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
