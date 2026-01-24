# For God so loved the world that he gave his only begotten Son,
#     that whoever believes in him should not perish but have eternal life.
#     John 3:16

"""
Propagators Chirho - A Rust-powered propagator network library.

This module provides constraint propagation and bidirectional computation
capabilities using interval arithmetic and lattice-based partial information.

Example usage::

    from propagators_chirho import PropagatorNetworkChirho, IntervalChirho

    # Create a network
    net_chirho = PropagatorNetworkChirho()

    # Create cells for a + b = c
    a_chirho = net_chirho.make_cell_chirho()
    b_chirho = net_chirho.make_cell_chirho()
    c_chirho = net_chirho.make_cell_chirho()

    net_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho)

    # Set values and propagate
    net_chirho.set_exact_chirho(a_chirho, 3.0)
    net_chirho.set_exact_chirho(b_chirho, 4.0)
    net_chirho.propagate_chirho()

    print(net_chirho.get_exact_chirho(c_chirho))  # 7.0
"""

# Import from the native Rust extension module
from .propagators_chirho import PropagatorNetworkChirho, IntervalChirho

__all__ = ['PropagatorNetworkChirho', 'IntervalChirho']
__version__ = "0.1.0"
__author__ = "loveJesus"
