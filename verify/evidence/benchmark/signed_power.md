# SignedPower benchmark (`numpy.sign/abs/power` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.88M | 0.021 | 46.57M | 0.026 | 1.01× | 1.21× |
| 10,000 | 0.149 | 67.11M | 0.144 | 69.40M | 0.044 | 0.30× | 0.31× |
| 100,000 | 1.317 | 75.95M | 1.251 | 79.94M | 0.194 | 0.15× | 0.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.129 | 1.41× |
| 1 | 5 | 0.378 | 0.443 | 1.17× |
| 1 | 10 | 0.558 | 0.834 | 1.50× |
| 10 | 1 | 0.064 | 0.089 | 1.39× |
| 10 | 5 | 0.271 | 0.408 | 1.51× |
| 10 | 10 | 0.596 | 0.845 | 1.42× |
| 100 | 1 | 0.063 | 0.088 | 1.39× |
| 100 | 5 | 0.277 | 0.409 | 1.48× |
| 100 | 10 | 0.596 | 0.838 | 1.41× |
| 1,000 | 1 | 0.081 | 0.085 | 1.04× |
| 1,000 | 5 | 0.266 | 0.442 | 1.67× |
| 1,000 | 10 | 0.569 | 1.045 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
