# AbsoluteBreadthIndex benchmark (`AbsoluteBreadthIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 212.09M | 0.003 | 303.93M | 8.469 | 1796.33× | 2574.12× |
| 10,000 | 0.029 | 339.97M | 0.025 | 404.32M | 82.985 | 2821.27× | 3355.28× |
| 100,000 | 0.254 | 393.13M | 0.230 | 434.58M | 845.272 | 3322.99× | 3673.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.192 | 0.276 | 1.44× |
| 1 | 5 | 2.050 | 1.540 | 0.75× |
| 1 | 10 | 0.537 | 2.926 | 5.45× |
| 10 | 1 | 0.080 | 0.299 | 3.74× |
| 10 | 5 | 0.225 | 2.257 | 10.05× |
| 10 | 10 | 0.450 | 2.955 | 6.57× |
| 100 | 1 | 0.046 | 1.055 | 23.15× |
| 100 | 5 | 0.194 | 5.794 | 29.82× |
| 100 | 10 | 0.408 | 11.193 | 27.42× |
| 1,000 | 1 | 0.051 | 8.707 | 172.24× |
| 1,000 | 5 | 0.361 | 44.121 | 122.37× |
| 1,000 | 10 | 0.571 | 90.973 | 159.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
