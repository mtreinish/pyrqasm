# pyrqasm

pyrqasm is a python module/extension written in rust that aims to be a fast
OPENQASM parser to Qiskit QuantumCircuit generator. You can use pyrqasm
instead of the built-in `qiskit.qasm` module. For example, instead of

```python
import qiskit
qc = qiskit.QuantumCircuit.from_qasm_file('circuit.qasm')
```

you can run:

```python
import pyrqasm
qc = pyrqasm.from_qasm_file('circuit.qasm')
```
