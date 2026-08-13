import numpy as np
from taflow import InverseFisherTransform
def test_inverse_fisher_transform_lifecycle():
    s=InverseFisherTransform(1);s.extend([1]);assert s.value is not None;s.reset();assert len(s)==0
