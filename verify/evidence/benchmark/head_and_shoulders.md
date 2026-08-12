# HeadAndShoulders benchmark (`HeadAndShoulders` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.08M | 0.012 | 84.87M | 0.251 | 17.81× | 21.26× |
| 10,000 | 0.098 | 102.47M | 0.134 | 74.62M | 1.355 | 13.88× | 10.11× |
| 100,000 | 0.950 | 105.30M | 0.891 | 112.25M | 12.681 | 13.35× | 14.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.223 | 1.61× |
| 1 | 5 | 0.361 | 0.855 | 2.37× |
| 1 | 10 | 0.541 | 1.757 | 3.25× |
| 10 | 1 | 0.057 | 0.168 | 2.93× |
| 10 | 5 | 0.267 | 1.227 | 4.59× |
| 10 | 10 | 0.569 | 1.901 | 3.34× |
| 100 | 1 | 0.060 | 0.200 | 3.35× |
| 100 | 5 | 0.290 | 1.295 | 4.46× |
| 100 | 10 | 0.600 | 1.915 | 3.19× |
| 1,000 | 1 | 0.072 | 0.355 | 4.96× |
| 1,000 | 5 | 0.299 | 1.807 | 6.05× |
| 1,000 | 10 | 0.583 | 3.091 | 5.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
