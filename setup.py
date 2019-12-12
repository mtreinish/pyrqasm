from setuptools import setup
from setuptools_rust import Binding, RustExtension


def readme():
    with open('README.rst') as f:
        return f.read()

setup(
    name="pyrqasm",
    version="0.0.1",
    description="A python graph library implemented in Rust",
    long_description=readme(),
    author = "Matthew Treinish",
    author_email = "mtreinish@kortar.org",
    classifiers=[
        "License :: OSI Approved :: Apache Software License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Rust",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Operating System :: MacOS :: MacOS X",
        "Operating System :: Microsoft :: Windows",
        "Operating System :: POSIX :: Linux",
    ],
    url="https://github.com/mtreinish/pyrqasm",
    project_urls={
        "Bug Tracker": "https://github.com/mtreinish/pyrqasm/issues",
        "Source Code": "https://github.com/mtreinish/pyrqasm",
        "Documentation": "https://pyrqas,.readthedocs.io",
    },
    install_requires=["qiskit-terra>=0.11.0"],
    rust_extensions=[RustExtension("pyrqasm.pyrqasm", "Cargo.toml",
                                   binding=Binding.PyO3)],
    include_package_data=True,
    packages=["pyrqasm"],
    zip_safe=False,
    python_requires=">=3.5",
)
