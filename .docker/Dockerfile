FROM continuumio/miniconda3:latest

RUN apt update && apt install cmake -y\
 && conda install -c conda-forge\
 jupyter jupyterlab ipywidgets plotly tornado "rust>=1.70.0" rust-src -y --quiet\
 && export PATH="$HOME/.cargo/bin:$PATH"\
 && cargo install evcxr_jupyter\
 && evcxr_jupyter --install\
 && rm -rf /var/lib/apt/lists/*\
 && conda clean -afy\
 && rm -rf /root/.cache/pip

