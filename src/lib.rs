// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

extern crate pyo3;
extern crate qasm;

use regex::{Captures, Regex};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyList, PySequence, PyTuple};
use pyo3::wrap_pyfunction;
use pyo3::Python;

use qasm::{lex, parse, Argument, AstNode};

pub fn process(input: &str, qiskit_path: &Path) -> PyResult<String> {
    let cwd = env::current_dir().unwrap();
    let comment_regex = Regex::new(r"//.*").unwrap();
    // Removed All Comments
    let cleaned = comment_regex.replace_all(input, "");

    // Regex for include statments
    let include_regex = Regex::new(r#"include\s*"(?P<s>.*)";"#).unwrap();

    let replace_with_file = |caps: &Captures| -> String {
        let filename = &caps["s"];
        let path = if filename == "qelib1.inc" {
            qiskit_path.join("qasm/libs/qelib1.inc")
        } else {
            cwd.join(&caps["s"])
        };
        let mut f = File::open(path).expect("Couldn't Open An Include File");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Couldn't Read Include Statement");
        comment_regex.replace_all(&contents, "").into()
    };

    // Remove Includes
    let processed = include_regex.replace_all(&cleaned, replace_with_file);

    Ok(processed.into())
}

fn generate_qubit_list<'a>(
    py: &'a Python,
    qubit_map: &mut HashMap<String, u8>,
    qubits: Vec<String>,
) -> PyResult<&'a PyList> {
    let mut out_qubits: Vec<u8> = Vec::new();
    for qubit in qubits {
        if !qubit_map.contains_key(&qubit) {
            return Err(InvalidQubit::new_err("Qubit not defined"));
        }
        out_qubits.push(*qubit_map.get(&qubit).unwrap())
    }
    let output = PyList::new(*py, out_qubits);
    Ok(output)
}

fn qasm_ast_to_circuit(source: String) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let qiskit = py.import("qiskit")?;
    let std_gates = py.import("qiskit.extensions.standard")?;
    let circuits = py.import("qiskit.circuit")?;
    let raw_qiskit_path = qiskit.get("__path__")?;
    let qiskit_path_list: &PyList = raw_qiskit_path.try_into()?;
    let qiskit_path_str = qiskit_path_list.get_item(0).to_string();
    let qiskit_path = Path::new(&qiskit_path_str);
    let processed_source = process(&source, qiskit_path)?;
    let mut tokens = lex(&processed_source);
    let mut gates: HashMap<String, PyObject> = HashMap::new();
    let mut qregs: HashMap<String, PyObject> = HashMap::new();
    let mut cregs: HashMap<String, PyObject> = HashMap::new();
    let mut standard_extension = HashMap::new();
    standard_extension.insert(
        "u1".to_string(),
        std_gates.get("U1Gate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "u2".to_string(),
        std_gates.get("U2Gate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "u3".to_string(),
        std_gates.get("U3Gate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "x".to_string(),
        std_gates.get("XGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "y".to_string(),
        std_gates.get("YGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "z".to_string(),
        std_gates.get("ZGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "t".to_string(),
        std_gates.get("TGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "tdg".to_string(),
        std_gates.get("TdgGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "s".to_string(),
        std_gates.get("SGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "sdg".to_string(),
        std_gates.get("SdgGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "swap".to_string(),
        std_gates.get("SwapGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "rx".to_string(),
        std_gates.get("RXGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "ry".to_string(),
        std_gates.get("RYGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "rz".to_string(),
        std_gates.get("RZGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "rzz".to_string(),
        std_gates.get("RZZGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "id".to_string(),
        std_gates.get("IdGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "h".to_string(),
        std_gates.get("HGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "cx".to_string(),
        std_gates.get("CnotGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "cy".to_string(),
        std_gates.get("CyGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "cz".to_string(),
        std_gates.get("CzGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "ch".to_string(),
        std_gates.get("CHGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "crz".to_string(),
        std_gates.get("CrzGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "cu1".to_string(),
        std_gates.get("Cu1Gate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "cu3".to_string(),
        std_gates.get("Cu3Gate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "ccx".to_string(),
        std_gates.get("ToffoliGate").unwrap().to_object(py),
    );
    standard_extension.insert(
        "cswap".to_string(),
        std_gates.get("FredkinGate").unwrap().to_object(py),
    );

    let ast = match parse(&mut tokens) {
        Ok(ast) => ast,
        Err(_error) => {
            println!("{}", _error);
            return Err(InvalidNodeType::new_err("Parser Error"));
        }
    };
    let raw_circ = qiskit.call0("QuantumCircuit")?;
    let qc = raw_circ.to_object(py);
    for node in ast {
        println!("{:?}", node);
        match node {
            AstNode::Gate(name, qubits, _params, nodes) => {
                let mut qubit_map = HashMap::new();
                let mut count: u8 = 0;
                for qubit in qubits.clone() {
                    qubit_map.insert(qubit, count);
                    count = count + 1;
                }
                let raw_custom_circ =
                    qiskit.call1("QuantumCircuit", (qubits.len(),))?;
                let custom_circ = raw_custom_circ.to_object(py);
                for subnode in nodes {
                    match subnode {
                        AstNode::ApplyGate(name, raw_qubits, params) => {
                            let mut qubits = Vec::new();
                            for qubit in raw_qubits {
                                match qubit {
                                    Argument::Qubit(name, index) => {
                                        qubits.push(name);
                                    }
                                    Argument::Register(name) => {
                                        qubits.push(name);
                                    }
                                }
                            }
                            let lc_name = name.to_ascii_lowercase();
                            let out_params = PyTuple::new(py, params);
                            if standard_extension.contains_key(&lc_name) {
                                let raw_gate =
                                    standard_extension.get(&lc_name).unwrap();
                                let gate = raw_gate.call1(py, out_params)?;
                                let qubit_list = generate_qubit_list(
                                    &py,
                                    &mut qubit_map,
                                    qubits,
                                )?;
                                custom_circ.call_method1(
                                    py,
                                    "append",
                                    (gate, qubit_list),
                                )?;
                            } else if gates.contains_key(&lc_name) {
                                let raw_gate = gates.get(&lc_name).unwrap();
                                let gate = raw_gate.call1(py, out_params)?;
                                let qubit_list = generate_qubit_list(
                                    &py,
                                    &mut qubit_map,
                                    qubits,
                                )?;
                                custom_circ.call_method1(
                                    py,
                                    "append",
                                    (gate, qubit_list),
                                )?;
                            } else if name == "U".to_string() {
                                let raw_gate =
                                    standard_extension.get("u3").unwrap();
                                let gate = raw_gate.call1(py, out_params)?;
                                let qubit_list = generate_qubit_list(
                                    &py,
                                    &mut qubit_map,
                                    qubits,
                                )?;
                                custom_circ.call_method1(
                                    py,
                                    "append",
                                    (gate, qubit_list),
                                )?;
                            } else if name == "CX".to_string() {
                                let raw_gate =
                                    standard_extension.get("u3").unwrap();
                                let gate = raw_gate.call1(py, out_params)?;
                                let qubit_list = generate_qubit_list(
                                    &py,
                                    &mut qubit_map,
                                    qubits,
                                )?;
                                custom_circ.call_method1(
                                    py,
                                    "append",
                                    (gate, qubit_list),
                                )?;
                            }
                        }
                        _ => {
                            return Err(InvalidNodeType::new_err(
                                "Invalid Node type",
                            ))
                        }
                    }
                }

                let custom_gate = custom_circ
                    .call_method0(py, "to_instruction")
                    .unwrap()
                    .to_object(py);
                gates.insert(name, custom_gate);
            }
            AstNode::Opaque(name, qubits, params) => {
                let gate = circuits
                    .call_method1("Gate", (&name, qubits.len(), params))
                    .unwrap()
                    .to_object(py);
                gates.insert(name, gate);
            }
            AstNode::QReg(name, num) => {
                let raw_qreg =
                    qiskit.call_method1("QuantumRegister", (num, &name))?;
                //let qreg_tup  = raw_qreg.extract()?;
                let qreg_obj = raw_qreg.to_object(py);
                let out_params = PyTuple::new(py, &[&qreg_obj]);
                qc.call_method1(py, "add_register", out_params)?;
                qregs.insert(name, qreg_obj);
            }
            AstNode::CReg(name, num) => {
                let raw_creg =
                    qiskit.call_method1("ClassicalRegister", (num, &name))?;
                let creg = raw_creg.to_object(py);
                cregs.insert(name, creg);
                let out_params = PyTuple::new(py, &cregs);
                qc.call_method1(py, "add_register", out_params)?;
            }
            AstNode::ApplyGate(name, qubits, params) => {
                let lc_name = name.to_ascii_lowercase();
                let out_params = PyTuple::new(py, params);
                let mut out_qubits: Vec<PyObject> = Vec::new();
                let mut registers: HashMap <String, Vec<PyObject>> = HashMap::new();
                for qubit in qubits {
                    let qubit_tmp: PyObject;
                    match qubit {
                        Argument::Register(reg_name) => {
                            let qreg_obj = qregs.get(&reg_name).unwrap();
                            out_qubits.push(qreg_obj.clone_ref(py));
                        }
                        Argument::Qubit(reg_name, index) => {
                            if registers.contains_key(&reg_name) {
                                let register = registers.get(&reg_name).unwrap();
                                out_qubits.push(register.get(index as usize).unwrap().clone_ref(py));
                            } else {
                                let qreg_obj = qregs.get(&reg_name).unwrap();
                                let qubit_vec: Vec<PyObject> =
                                    qreg_obj.extract(py)?;
                                let qubit_tmp_orig = &qubit_vec.get(index as usize).unwrap();
                                let qubit_clone = qubit_tmp_orig.clone_ref(py);
                                qubit_tmp = qubit_clone;
                                registers.insert(reg_name.to_string(), qubit_vec);
                                out_qubits.push(qubit_tmp)
                            }
                        }
                    }
                }

                if standard_extension.contains_key(&lc_name) {
                    let raw_gate = standard_extension.get(&lc_name).unwrap();
                    let gate = raw_gate.call1(py, out_params)?;
                    let out_qubits_list = PyList::new(py, out_qubits);
                    let out_params = PyTuple::new(
                        py,
                        &[gate, out_qubits_list.to_object(py)],
                    );
                    qc.call_method1(py, "append", out_params)?;
                } else if gates.contains_key(&lc_name) {
                    let raw_gate = gates.get(&lc_name).unwrap();
                    let gate = raw_gate.call1(py, out_params)?;
                    let out_qubits_list = PyList::new(py, out_qubits);
                    let out_params = PyTuple::new(
                        py,
                        &[gate, out_qubits_list.to_object(py)],
                    );
                    qc.call_method1(py, "append", out_params)?;
                }
            }
            AstNode::Barrier(arg) => match arg {
                Argument::Register(reg_name) => {
                    let qreg_obj = qregs.get(&reg_name).unwrap();
                    let out_params = PyTuple::new(py, &[&qreg_obj]);
                    qc.call_method1(py, "barrier", out_params)?;
                }
                Argument::Qubit(reg_name, index) => {
                    let qreg_obj = qregs.get(&reg_name).unwrap();
                    let qubit_vec: Vec<PyObject> = qreg_obj.extract(py)?;
                    let q_out = qubit_vec.get(index as usize).unwrap();
                    let out_params = PyTuple::new(py, &[&q_out]);
                    qc.call_method1(py, "barrier", out_params)?;
                }
            },
            AstNode::Reset(arg) => match arg {
                Argument::Register(reg_name) => {
                    let qreg_obj = qregs.get(&reg_name).unwrap();
                    let qubit_vec: Vec<PyObject> = qreg_obj.extract(py)?;
                    for q_out in qubit_vec {
                        let out_params = PyTuple::new(py, &[&q_out]);
                        qc.call_method1(py, "barrier", out_params)?;
                    }
                }
                Argument::Qubit(reg_name, index) => {
                    let qreg_obj = qregs.get(&reg_name).unwrap();
                    let qubit_vec: Vec<PyObject> = qreg_obj.extract(py)?;
                    let q_out = qubit_vec.get(index as usize).unwrap();
                    let out_params = PyTuple::new(py, &[&q_out]);
                    qc.call_method1(py, "barrier", out_params)?;
                }
            },
            AstNode::Measure(qubit_arg, clbit_arg) => {
                let qubit: &PyObject;
                let clbit: &PyObject;
                let qubit_ref: PyObject;
                match qubit_arg {
                    Argument::Register(name) => {
                        qubit = qregs.get(&name).unwrap();
                    }
                    Argument::Qubit(name, index) => {
                        let qreg = qregs.get(&name).unwrap();
                        let qreg_seq: &PySequence = qreg.cast_as(py)?;
                        let raw_qubit = qreg_seq.get_item(index as isize)?;
                        qubit_ref = raw_qubit.to_object(py);
                        qubit = &qubit_ref;
                    }
                }
                let clbit_ref: PyObject;
                match clbit_arg {
                    Argument::Register(name) => {
                        clbit = cregs.get(&name).unwrap();
                    }
                    Argument::Qubit(name, index) => {
                        let creg = cregs.get(&name).unwrap();
                        let creg_seq: &PySequence = creg.cast_as(py)?;
                        let raw_clbit = creg_seq.get_item(index as isize)?;
                        clbit_ref = raw_clbit.to_object(py);
                        clbit = &clbit_ref;
                    }
                }
                qc.call_method1(py, "measure", (qubit, clbit))?;
            }
            AstNode::If(clreg, value, ast_node) => {}
        }
        println!("\nNode:\n");
        println!("\n");
        //        qc.call1('x');
    }
    Ok(qc)
}

#[pyfunction]
fn parse_qasm_file(path: String) -> PyResult<PyObject> {
    let mut source = String::new();

    let mut f = File::open(path)?;
    f.read_to_string(&mut source)?;
    qasm_ast_to_circuit(source)
}

#[pyfunction]
fn parse_qasm_str(qasm: String) -> PyResult<PyObject> {
    qasm_ast_to_circuit(qasm)
}

#[pymodule]
fn pyrqasm(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(parse_qasm_file)).unwrap();
    m.add_wrapped(wrap_pyfunction!(parse_qasm_str)).unwrap();
    Ok(())
}

create_exception!(pyrqasm, InvalidQubit, PyException);
create_exception!(pyrqasm, InvalidNodeType, PyException);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
