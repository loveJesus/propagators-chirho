# For God so loved the world that he gave his only begotten Son,
#     that whoever believes in him should not perish but have eternal life.
#     John 3:16

"""
Propagators Chirho - A Rust-powered propagator network library.

This module provides constraint propagation and bidirectional computation
capabilities using interval arithmetic and lattice-based partial information.

Example usage::

    from propagators_chirho import PropagatorNetwork, Interval

    # Create a network
    net = PropagatorNetwork()

    # Create cells for a + b = c
    a = net.make_cell_chirho()
    b = net.make_cell_chirho()
    c = net.make_cell_chirho()

    net.add_adder_chirho(a, b, c)

    # Set values and propagate
    net.set_exact_chirho(a, 3.0)
    net.set_exact_chirho(b, 4.0)
    net.propagate_chirho()

    print(net.get_exact_chirho(c))  # 7.0
"""

# Import from the native Rust extension module
from .propagators_chirho import PropagatorNetwork, Interval

__all__ = ['PropagatorNetwork', 'Interval']
__version__ = "0.1.0"
__author__ = "loveJesus"
