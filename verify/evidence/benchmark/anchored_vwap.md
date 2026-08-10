# AnchoredVolumeWeightedAveragePrice benchmark (`anchored VWAP deviation bands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.18M | 0.013 | 77.99M | 1.321 | 78.18× | 103.04× |
| 10,000 | 0.086 | 115.83M | 0.078 | 127.60M | 13.060 | 151.27× | 166.66× |
| 100,000 | 0.799 | 125.09M | 0.771 | 129.70M | 131.242 | 164.17× | 170.22× |
| 1,000,000 | 9.249 | 108.13M | 8.029 | 124.55M | 1357.249 | 146.75× | 169.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.125 | 0.84× |
| 1 | 5 | 0.312 | 0.479 | 1.54× |
| 1 | 10 | 0.513 | 0.942 | 1.84× |
| 10 | 1 | 0.059 | 0.109 | 1.84× |
| 10 | 5 | 0.243 | 0.569 | 2.34× |
| 10 | 10 | 0.545 | 1.082 | 1.99× |
| 100 | 1 | 0.051 | 0.226 | 4.42× |
| 100 | 5 | 0.246 | 1.167 | 4.75× |
| 100 | 10 | 0.523 | 2.336 | 4.47× |
| 1,000 | 1 | 0.062 | 1.449 | 23.33× |
| 1,000 | 5 | 0.256 | 7.653 | 29.87× |
| 1,000 | 10 | 0.631 | 14.613 | 23.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
