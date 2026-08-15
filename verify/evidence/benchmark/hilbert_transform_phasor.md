# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.96M | 0.046 | 21.85M | 0.073 | 1.59× | 1.59× |
| 10,000 | 0.470 | 21.26M | 0.436 | 22.91M | 0.467 | 0.99× | 1.07× |
| 100,000 | 5.214 | 19.18M | 4.543 | 22.01M | 4.684 | 0.90× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.126 | 1.82× |
| 1 | 5 | 0.192 | 0.515 | 2.68× |
| 1 | 10 | 0.429 | 1.004 | 2.34× |
| 10 | 1 | 0.039 | 0.091 | 2.33× |
| 10 | 5 | 0.218 | 0.479 | 2.20× |
| 10 | 10 | 0.547 | 1.021 | 1.87× |
| 100 | 1 | 0.054 | 0.110 | 2.04× |
| 100 | 5 | 0.205 | 0.473 | 2.31× |
| 100 | 10 | 0.453 | 1.108 | 2.45× |
| 1,000 | 1 | 0.092 | 0.150 | 1.64× |
| 1,000 | 5 | 0.232 | 0.695 | 3.00× |
| 1,000 | 10 | 0.494 | 1.496 | 3.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
